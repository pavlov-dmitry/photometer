use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_row, ToValue, Value, IntoValue };
use std::fmt::Display;
use types::{ Id, EmptyResult, CommonResult, CommonError };
use database::Database;

pub struct Votes {
    pub all_count: usize,
    pub yes: Vec<Id>,
    pub no: Vec<Id>
}

pub trait DbVotes {
    /// добавляет право голоса по какому-то событию
    fn add_rights_of_voting( &mut self, scheduled_id: Id, users: &[Id]  ) -> EmptyResult;
    /// добавляет право голоса по какому-то событию для всей группы
    fn add_rights_of_voting_for_group( &mut self, scheduled_id: Id, group_id: Id ) -> EmptyResult;
    /// проверяет на то что все проголосовали
    fn is_all_voted( &mut self, scheduled_id: Id ) -> CommonResult<bool>;
    /// проверяет голосвал ли этот пользователь уже или нет
    fn is_need_user_vote( &mut self, scheduled_id: Id, user_id: Id ) -> CommonResult<bool>;
    /// возращает голоса
    fn get_votes( &mut self, scheduled_id: Id ) -> CommonResult<Votes>;
    /// голосуем
    fn set_vote( &mut self, scheduled_id: Id, user_id: Id, vote: bool ) -> EmptyResult;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute( "
        CREATE TABLE IF NOT EXISTS `votes` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `scheduled_id` bigint(20) NOT NULL DEFAULT '0',
            `user_id` bigint(20) NOT NULL DEFAULT '0',
            `voted` BOOL NOT NULL DEFAULT false,
            `vote` BOOL NOT NULL DEFAULT false,
            PRIMARY KEY ( `id` ),
            KEY `voted_idx` ( `scheduled_id`, `user_id`, `voted` ) USING BTREE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::votes::create_tables"
    )
}

impl DbVotes for MyPooledConn {
    /// добавляет право голоса по какому-то событию
    fn add_rights_of_voting( &mut self, scheduled_id: Id, users: &[Id] ) -> EmptyResult {
        add_rights_of_voting_impl( self, scheduled_id, users )
            .map_err( |e| fn_failed( "add_rights_of_voting", e ) )
    }
    /// добавляет право голоса по какому-то событию для всей группы
    fn add_rights_of_voting_for_group( &mut self, scheduled_id: Id, group_id: Id ) -> EmptyResult {
        add_rights_of_voting_for_group_impl( self, scheduled_id, group_id )
            .map_err( |e| fn_failed( "add_rights_of_voting_for_group", e ) )
    }
    /// проверяет на то что все проголосовали
    fn is_all_voted( &mut self, scheduled_id: Id ) -> CommonResult<bool> {
        is_all_voted_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "is_all_voted", e ) )
    }
    /// проверяет голосвал ли этот пользователь уже или нет
    fn is_need_user_vote( &mut self, scheduled_id: Id, user_id: Id ) -> CommonResult<bool> {
        is_need_user_vote_impl( self, scheduled_id, user_id )
            .map_err( |e| fn_failed( "is_need_user_vote", e ) )
    }
    /// возращает голоса
    fn get_votes( &mut self, scheduled_id: Id ) -> CommonResult<Votes> {
        get_votes_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "get_votes", e ) )
    }
    /// голосуем
    fn set_vote( &mut self, scheduled_id: Id, user_id: Id, vote: bool ) -> EmptyResult {
        set_vote_impl( self, scheduled_id, user_id, vote )
            .map_err( |e| fn_failed( "set_vote", e ) )
    }

}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbVotes `{}` failed: {}", fn_name, e ) )
}

fn add_rights_of_voting_impl( conn: &mut MyPooledConn, scheduled_id: Id, users: &[Id] ) -> MyResult<()> {
    let mut query = format!(
        "INSERT INTO votes (
            scheduled_id,
            user_id
        ) VALUES ( ?, ? )"
    );

    for _ in (1 .. users.len()) {
        query.push_str( ", ( ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( &query ) );

    let mut values: Vec<Value> = Vec::new();
    for i in (0 .. users.len()) {
        values.push( scheduled_id.into_value() );
        values.push( users[ i ].into_value() );
    }

    try!( stmt.execute( &values ) );
    Ok( () )
}

fn add_rights_of_voting_for_group_impl( conn: &mut MyPooledConn, scheduled_id: Id, group_id: Id ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare(
        "INSERT
            INTO `votes` ( `scheduled_id`, `user_id` )
        SELECT
            ? as `scheduled_id`,
            `gm`.`user_id`
        FROM
            `group_members` as `gm`
        WHERE
           `gm`.`group_id` = ?
        "
    ));

    let params: &[ &ToValue ] = &[ &scheduled_id, &group_id ];
    try!( stmt.execute( params ) );
    Ok( () )
}

fn is_all_voted_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT COUNT( id ) FROM votes WHERE scheduled_id = ? AND voted=false" ) );
    let params: &[ &ToValue ] = &[ &scheduled_id ];
    let mut result = try!( stmt.execute( params ) );
    let row = try!( result.next().unwrap() );
    let (count,) : (u32,) = from_row( row );
    Ok( count == 0 )
}

fn is_need_user_vote_impl( conn: &mut MyPooledConn, scheduled_id: Id, user_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "
        SELECT
            COUNT( `id` )
        FROM
            `votes`
        WHERE
            `scheduled_id` = ?
            AND `user_id` = ?
            AND `voted` = false
    "));
    let params: &[ &ToValue ] = &[ &scheduled_id, &user_id ];
    let mut result = try!( stmt.execute( params ) );
    let row = try!( result.next().unwrap() );
    let (count,) : (u32,) = from_row( row );
    Ok( count == 1 )
}

fn get_votes_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Votes> {
    let mut stmt = try!( conn.prepare( "SELECT user_id, voted, vote FROM votes WHERE scheduled_id = ?" ) );
    let params: &[ &ToValue ] = &[ &scheduled_id ];
    let result = try!( stmt.execute( params ) );

    let mut votes = Votes {
        yes: Vec::new(),
        no: Vec::new(),
        all_count: 0
    };

    for row in result {
        let row = try!( row );
        let (user_id, voted, vote): (Id, bool, bool) = from_row( row );
        if voted {
            if vote == true {
                votes.yes.push( user_id );
            }
            else {
                votes.no.push( user_id );
            }
        }
        votes.all_count += 1;
    }

    Ok( votes )
}

fn set_vote_impl( conn: &mut MyPooledConn, scheduled_id: Id, user_id: Id, vote: bool ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare( "UPDATE votes SET vote=?, voted=true WHERE scheduled_id=? AND user_id=?" ) );
    let params: &[ &ToValue ] = &[ &vote, &scheduled_id, &user_id ];
    try!( stmt.execute( params ) );
    Ok( () )
}
