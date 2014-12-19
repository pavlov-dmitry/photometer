use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use std::fmt::{ Show };

pub struct Votes {
    pub all_count: u32,
    pub yes: Vec<Id>,
    pub no: Vec<Id>
}

pub trait DbVotes {
    /// добавляет право голоса по какому-то событию
    fn add_rights_of_voting( &mut self, scheduled_id: Id, users: &Vec<Id> ) -> EmptyResult;
    /// проверяет на то что все проголосовали 
    fn is_all_voted( &mut self, scheduled_id: Id ) -> CommonResult<bool>;
    /// возращает голоса
    fn get_votes( &mut self, scheduled_id: Id ) -> CommonResult<Votes>;
    /// голосуем 
    fn set_vote( &mut self, scheduled_id: Id, user_id: Id, vote: bool ) -> EmptyResult;
}

impl DbVotes fro MyPooledConn {
    /// добавляет право голоса по какому-то событию
    fn add_rights_of_voting( &mut self, scheduled_id: Id, users: &[Id] ) -> EmptyResult {
        add_rights_of_voting_impl( self, scheduled_id, users )
            .map_err( |e| fn_failed( "add_rights_of_voting", e ) )
    }
    /// проверяет на то что все проголосовали 
    fn is_all_voted( &mut self, scheduled_id: Id ) -> CommonResult<bool> {
        is_all_voted( self, scheduled_id )
            .map_err( |e| fn_failed( "is_all_voted", e ) )
    }
    /// возращает голоса
    fn get_votes( &mut self, scheduled_id: Id ) -> CommonResult<Votes> {
        get_votes( self, scheduled_id )
            .map_err( |e| fn_failed( "get_votes", e ) )
    }
    /// голосуем 
    fn set_vote( &mut self, scheduled_id: Id, user_id: Id, vote: bool ) -> EmptyResult {
        set_vote_impl( self, scheduled_id, user_id, vote )
            .map_err( |e| fn_failed( "set_vote", e ) )
    }

}

fn fn_failed<E: Show>( fn_name: &str, e E ) -> String {
    format!( "DbVotes `{}` failed: {}", fn_name, e )
}

fn add_rights_of_voting_impl( conn: &mut MyPooledConn, scheduled_id: Id, users: &Vec<Id> ) -> MyResult<()> {
    let mut query = format!(
        "INSERT INTO votes (
            scheduled_id,
            user_id
        ) VALUES ( ?, ? )"
    );

    for _ in range( 1, user.len() ) {
        query_push_str( ", ( ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( query.as_slice() ) );

    let values: Vec<&ToValue> = Vec::new();
    for user in users.iter() {
        values.push( &scheduled_id );
        values.push( &user );
    }

    try!( stmt.execute( values.as_slice() ) )
    Ok( () )
}

fn is_all_voted_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT COUNT( id ) FROM votes WHERE scheduled_id = ? AND voted=false" ) );
    let mut result = try!( stmt.execute( &[ &scheduled_id ] ) );
    let row = try!( result.next().unwrap() );
    Ok( from_value::<u32>( &row[ 0 ] ) == 0 )
}

fn get_votes_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Votes> {
    let mut stmt = try!( conn.prepare( "SELECT user_id, voted, vote FROM votes WHERE scheduled_id = ?" ) );
    let mut result = try!( stmt.execute( &[ &scheduled_id ] ) );

    let mut votes = Votes {
        yes: Vec::new(),
        no: Vec::new(),
        all_count = result.len()
    }

    for row in result {
        let row = try!( row );
        let user_id : Id = from_value( &row[ 0 ] );
        let voted: bool = from_value( &row[ 1 ] );
        let vote: bool = from_value( &row[ 2 ] );
        if voted {
            if vote == true { 
                votes.yes.push( user_id );
            }
            else {
                votes.no.push( user_id );
            }
        }
    }
    
    Ok( votes )
}

fn set_vote_impl( conn: &mut MyPooledConn, scheduled_id: Id, user_id: Id, vote: bool ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare( "UPDATE votes SET vote=? WHERE scheduled_id=? AND user_id=?" ) );
    try!( stmt.execute( &[ &vote, &scheduled_id, &user_id ] ) );
    Ok( () )
}