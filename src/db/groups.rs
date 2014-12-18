use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use types::{ Id, CommonResult };
use authentication::{ User };
use std::fmt::{ Show };
  
type Members = Vec<User>;

pub trait DbGroups {
    /// возращает членов группы
    fn get_members( &mut self, group_id: Id ) -> CommonResult<Members>;
    /// считывает кол-во пользователей в группе
    fn get_members_count( &mut self, group_id: Id ) -> CommonResult<u32>;
    /// проверяет пользователя на принадлежность к группе
    fn is_member( &mut self, user_id: Id, group_id: Id ) -> CommonResult<bool>;
    /// проверяет существоание группы
    fn is_group_exists( &mut self, group_id: Id ) -> CommonResult<bool>;
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
    fn is_group_exists( &mut self, group_id: Id ) -> CommonResult<bool> {
        is_group_exists_impl( self, group_id )
            .map_err( |e| fn_failed( "is_group_exists", e ) )
    }
}

fn fn_failed<E: Show>( fn_name: &str, e: E ) -> String {
    format!( "DbGroups {} failed: {}", fn_name, e )
}

fn get_members_impl( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<Members> {
    let mut stmt = try!( conn.prepare( 
        "SELECT 
            g.user_id,
            u.login, 
        FROM group_members AS g LEFT JOIN users AS u ON ( u.id = g.user_id )
        WHERE u.id IS NOT NULL AND g.group_id = ?
    "));
    let mut members = Vec::new();
    for row in try!( stmt.execute( &[ &group_id ] ) ) {
        let row = try!( row );
        members.push( User{ 
            id: from_value( &row[ 0 ] ), 
            name: from_value( &row[ 1 ] )
        });
    }
    Ok( members )
}

fn get_members_count_impl( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<u32> {
    let mut stmt = try!( conn.prepare( "SELECT COUNT(id) FROM group_members WHERE group_id=?" ));
    let mut result = try!( stmt.execute( &[ &group_id ] ) );
    let row = try!( result.next().unwrap() );
    Ok( from_value( &row[ 0 ] ) )
}

fn is_member_impl( conn: &mut MyPooledConn, user_id: Id, group_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT id FROM group_members WHERE user_id=? AND group_id=?" ) );
    let mut result = try!( stmt.execute( &[ &user_id, &group_id ] ) );
    Ok( result.count() == 1 )
}

fn is_group_exists_impl( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "SELECT id FROM groups WHERE id=?" ) );
    let mut result = try!( stmt.execute( &[ &group_id ] ) );
    Ok( result.count() == 1 )
}