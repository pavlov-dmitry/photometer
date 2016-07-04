use mysql;
use mysql::conn::pool::{ PooledConn };
use database::Database;
use types::{
    Id,
    EmptyResult,
    CommonError,
    CommonResult
};
use std::fmt::Display;
use mysql::value::{
    Value,
    ToValue,
    from_row
};

pub struct MessageLink {
    pub user_id: Id,
    pub message_id: Id
}

pub trait DbCommentMessageLink {
    /// добавляет ссылки комментариев и сообщений с оповещениями
    fn add_comment_message_links( &mut self, comment_id: Id, links: Vec<MessageLink> ) -> EmptyResult;
    /// возвращает список писем связанных с сообщениями у определенного пользователя
    fn get_linked_messages( &mut self, user_id: Id, comment_ids: Vec<Id> ) -> CommonResult<Vec<Id>>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `comment_message_link` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `user_id` bigint(20) NOT NULL DEFAULT '0',
            `comment_id` bigint(20) NOT NULL DEFAULT '0',
            `message_id` bigint(20) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` ),
            KEY `user_comment_idx` ( `user_id`, `comment_id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::comment_message_link::create_tables"
    )
}

impl DbCommentMessageLink for PooledConn {
    /// добавляет ссылки комментариев и сообщений с оповещениями
    fn add_comment_message_links( &mut self, comment_id: Id, links: Vec<MessageLink> ) -> EmptyResult {
        add_comment_message_links_impl( self, comment_id, links )
            .map_err( |e| fn_failed( "add_comment_message_links", e ) )
    }

    /// возвращает список писем связанных с сообщениями у определенного пользователя
    fn get_linked_messages( &mut self, user_id: Id, comment_ids: Vec<Id> ) -> CommonResult<Vec<Id>> {
        get_linked_messages_impl( self, user_id, comment_ids )
            .map_err( |e| fn_failed( "get_linked_messages", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbCommentMessageLink {} failed: {}", fn_name, e ) )
}

fn add_comment_message_links_impl(
    conn: &mut PooledConn,
    comment_id: Id,
    links: Vec<MessageLink>
) -> mysql::Result<()>
{
    let mut query = format!(
        "INSERT INTO comment_message_link (
            user_id,
            comment_id,
            message_id
        ) VALUES ( ?, ?, ? )"
    );

    for _ in 1 .. links.len() {
        query.push_str( ", ( ?, ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( &query ) );

    let mut values: Vec<Value> = Vec::new();
    for link in links {
        values.push( link.user_id.to_value() );
        values.push( comment_id.to_value() );
        values.push( link.message_id.to_value() );
    }

    try!( stmt.execute( &values ) );
    Ok( () )
}

fn get_linked_messages_impl(
    conn: &mut PooledConn,
    user_id: Id,
    comment_ids: Vec<Id>
) -> mysql::Result<Vec<Id>>
{
    if comment_ids.is_empty() {
        return Ok( Vec::new() );
    }

    let mut query = format!(
        "SELECT message_id
        FROM comment_message_link
        WHERE user_id=?
          AND comment_id IN ( ?"
    );
    for _ in 1 .. comment_ids.len() {
        query.push_str( ",? ");
    }
    query.push_str( ")" );

    let mut stmt = try!( conn.prepare( &query ) );

    let mut values: Vec<Value> = Vec::new();
    values.push( user_id.to_value() );
    for id in comment_ids {
        values.push( id.to_value() );
    }

    let mut result = Vec::new();
    let sql_result = try!( stmt.execute( &values ) );
    for row in sql_result {
        let row = try!( row );
        let (id,) = from_row( row );
        result.push( id );
    }
    Ok( result )
}
