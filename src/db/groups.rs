use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_row, ToValue, Value, IntoValue };
use types::{ Id, CommonResult, EmptyResult, CommonError };
use authentication::{ User };
use std::fmt::Display;
use database::Database;
use std::convert::From;
use time::Timespec;

type Members = Vec<User>;
type Groups = Vec<(Id, String)>;

pub trait DbGroups {
    /// возращает членов группы
    fn get_members( &mut self, group_id: Id ) -> CommonResult<Members>;
    /// считывает кол-во пользователей в группе
    fn get_members_count( &mut self, group_id: Id ) -> CommonResult<u32>;
    /// проверяет пользователя на принадлежность к группе
    fn is_member( &mut self, user_id: Id, group_id: Id ) -> CommonResult<bool>;
    /// проверяет существоание группы
    fn is_group_id_exists( &mut self, group_id: Id ) -> CommonResult<bool>;
    /// проверяет существоание группы
    fn is_group_exists( &mut self, name: &String ) -> CommonResult<bool>;
    /// создать новую группу
    fn create_group( &mut self, name: &String, desc: &String ) -> CommonResult<Id>;
    /// добавляет членов группы
    fn add_members( &mut self, group_id: Id, members: &[ Id ] ) -> EmptyResult;
    /// установка последнего времени посещения
    fn set_last_visited_time( &mut self, user_id: Id, group_id: Id, time: Timespec ) -> EmptyResult;
    /// считывание последнего времени посещения
    fn get_last_visited_time( &mut self, user_id: Id, group_id: Id ) -> CommonResult<Timespec>;
    /// список групп в которых пользователь является членом
    fn member_in_groups( &mut self, user_id: Id ) -> CommonResult<Groups>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
            // `timetable_version` int(4) unsigned DEFAULT '0',
    try!( db.execute(
        "CREATE TABLE IF NOT EXISTS `groups` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `name` varchar(128) NOT NULL DEFAULT '',
            `description` TEXT NOT NULL DEFAULT '',
            PRIMARY KEY ( `id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::groups::create_tables(groups)"
    ));
    db.execute(
        "CREATE TABLE IF NOT EXISTS `group_members` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `user_id` bigint(20) NOT NULL DEFAULT '0',
            `group_id` bigint(20) NOT NULL DEFAULT '0',
            `last_visited_time` int(11) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` ),
            KEY `members_idx` ( `user_id`, `group_id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::groups::create_tables(group_members)"
    )
}

impl DbGroups for MyPooledConn {
    /// возращает итератор на членов группы
    fn get_members<'a>( &mut self, group_id: Id ) -> CommonResult<Members> {
        get_members_impl( self, group_id )
            .map_err( |e| fn_failed( "members", e ) )
    }
    fn get_members_count( &mut self, group_id: Id ) -> CommonResult<u32> {
        get_members_count_impl( self, group_id )
            .map_err( |e| fn_failed( "members_count", e ) )
    }
    fn is_member( &mut self, user_id: Id, group_id: Id ) -> CommonResult<bool> {
        is_member_impl( self, user_id, group_id )
            .map_err( |e| fn_failed( "is_member", e ) )
    }
    fn is_group_id_exists( &mut self, group_id: Id ) -> CommonResult<bool> {
        is_group_id_exists_impl( self, group_id )
            .map_err( |e| fn_failed( "is_group_id_exists", e ) )
    }
    /// проверяет существоание группы
    fn is_group_exists( &mut self, name: &String ) -> CommonResult<bool> {
        is_group_exists_impl( self, &name )
            .map_err( |e| fn_failed( "is_group_exists", e ) )
    }
    /// создать новую группу
    fn create_group( &mut self, name: &String, desc: &String ) -> CommonResult<Id> {
        create_group_impl( self, name, desc )
            .map_err( |e| fn_failed( "create_group", e ) )
    }
    /// добавляет членов группы
    fn add_members( &mut self, group_id: Id, members: &[ Id ] ) -> EmptyResult {
        add_members_impl( self, group_id, members )
            .map_err( |e| fn_failed( "add_members", e ) )
    }
    /// установка последнего времени посещения
    fn set_last_visited_time( &mut self, user_id: Id, group_id: Id, time: Timespec ) -> EmptyResult {
        set_last_visited_time_impl( self, user_id, group_id, time )
            .map_err( |e| fn_failed( "set_last_visited_time", e ) )
    }
    /// считывание последнего времени посещения
    fn get_last_visited_time( &mut self, user_id: Id, group_id: Id ) -> CommonResult<Timespec> {
        get_last_visited_time_impl( self, user_id, group_id )
            .map_err( |e| fn_failed( "get_last_visited_time", e ) )
    }
    /// список групп в которых пользователь является членом
    fn member_in_groups( &mut self, user_id: Id ) -> CommonResult<Groups> {
        member_in_groups_impl( self, user_id )
            .map_err( |e| fn_failed( "member_in_groups", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbGroups {} failed: {}", fn_name, e ) )
}

fn get_members_impl( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<Members> {
    let mut stmt = try!( conn.prepare(
        "SELECT
            g.user_id,
            u.login,
            u.mail
        FROM group_members AS g LEFT JOIN users AS u ON ( u.id = g.user_id )
        WHERE u.id IS NOT NULL AND g.group_id = ?
    "));
    let mut members = Vec::new();
    let params: &[ &ToValue ] = &[ &group_id ];
    for row in try!( stmt.execute( params ) ) {
        let row = try!( row );
        let (id, name, mail) = from_row( row );
        members.push( User{
            id: id,
            name: name,
            mail: mail
        });
    }
    Ok( members )
}

fn get_members_count_impl( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<u32> {
    let mut stmt = try!( conn.prepare( "SELECT COUNT(id) FROM group_members WHERE group_id=?" ));
    let params: &[ &ToValue ] = &[ &group_id ];
    let mut result = try!( stmt.execute( params ) );
    let row = try!( result.next().unwrap() );
    let (count,) = from_row( row );
    Ok( count )
}

fn is_member_impl( conn: &mut MyPooledConn, user_id: Id, group_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT id FROM group_members WHERE user_id=? AND group_id=?" ) );
    let params: &[ &ToValue ] = &[ &user_id, &group_id ];
    let result = try!( stmt.execute( params ) );
    Ok( result.count() == 1 )
}

fn is_group_id_exists_impl( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT id FROM groups WHERE id=?" ) );
    let params: &[ &ToValue ] = &[ &group_id ];
    let result = try!( stmt.execute( params ) );
    Ok( result.count() == 1 )
}

fn is_group_exists_impl( conn: &mut MyPooledConn, name: &str ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT id FROM groups WHERE name=?" ) );
    let params: &[ &ToValue ] = &[ &name ];
    let result = try!( stmt.execute( params ) );
    Ok( result.count() == 1 )
}

fn create_group_impl( conn: &mut MyPooledConn, name: &String, desc: &String ) -> MyResult<Id> {
    let mut stmt = try!( conn.prepare( "
        INSERT INTO groups (
            name,
            description
        )
        VALUES ( ?, ? )
    "));
    let params: &[ &ToValue ] = &[ name, desc ];
    let result = try!( stmt.execute( params ) );
    Ok( result.last_insert_id() )
}

fn add_members_impl( conn: &mut MyPooledConn, group_id: Id, members: &[ Id ] ) -> MyResult<()> {
    let mut query: String = String::from("
        INSERT INTO group_members (
            user_id,
            group_id
        )
        VALUES( ?, ? )
    ");

    for _ in (1 .. members.len()) {
        query.push_str( ", ( ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( &query ) );

    let mut values: Vec<Value> = Vec::new();
    for i in (0 .. members.len()) {
        values.push( members[ i ].into_value() );
        values.push( group_id.into_value() );
    }

    try!( stmt.execute( values ) );
    Ok( () )
}

/// установка последнего времени посещения
fn set_last_visited_time_impl( conn: &mut MyPooledConn, user_id: Id, group_id: Id, time: Timespec ) -> MyResult<()> {
    let query = "UPDATE
                     group_members
                 SET
                    last_visited_time = ?
                 WHERE
                     user_id = ?
                 AND
                     group_id = ?";
    let mut stmt = try!( conn.prepare( query ) );
    try!( stmt.execute( ( time.sec, user_id, group_id ) ) );
    Ok( () )
}

/// считывание последнего времени посещения
fn get_last_visited_time_impl( conn: &mut MyPooledConn, user_id: Id, group_id: Id ) -> MyResult<Timespec> {
    let query = "SELECT last_visited_time FROM group_members WHERE user_id=? AND group_id=?";
    let mut stmt = try!( conn.prepare( query ) );
    let mut result = try!( stmt.execute( ( &user_id, &group_id ) ) );
    let row = try!( result.next().unwrap() );
    let time = from_row::<i64>( row );
    Ok( Timespec::new( time, 0 ) )
}

fn member_in_groups_impl( conn: &mut MyPooledConn, user_id: Id ) -> MyResult<Groups> {
    let query = "SELECT
                     gm.group_id, g.name
                 FROM
                     group_members AS gm
                 LEFT JOIN
                     groups AS g ON( g.id = gm.group_id )
                 WHERE
                     gm.user_id = ?";
    let mut stmt = try!( conn.prepare( query ) );
    let result = try!( stmt.execute( (user_id,) ) );
    let mut groups = Vec::new();
    for row in result {
        let row = try!( row );
        groups.push( from_row( row ) );
    }
    Ok( groups )
}
