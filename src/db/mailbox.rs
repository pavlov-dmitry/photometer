use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{
    from_row,
    ToValue,
    Value
};
use time;
use types::{ Id, CommonResult, EmptyResult, MailInfo, CommonError };
use std::fmt::Display;
use parse_utils::{ GetMsecs };
use database::Database;

pub trait DbMailbox {
    /// посылает письмо одному из участников
    fn send_mail_to( &mut self, recipient_id: Id, sender_name: &str, subject: &str, body: &str ) -> CommonResult<Id>;
    /// подсчитывает кол-во писем у определенного участника
    fn messages_count( &mut self, owner_id: Id, only_unreaded: bool ) -> CommonResult<u32>;
    /// читает сообщения с пагинацией в обратном от создания порядке
    fn messages_from_last<F: FnMut(MailInfo)>( &mut self, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: &mut F ) -> EmptyResult;
    /// помечает сообщение как прочитанное
    fn mark_as_readed( &mut self, owner_id: Id, message_ids: &[Id] ) -> EmptyResult;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `mailbox` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `creation_time` bigint(20) NOT NULL DEFAULT '0',
            `recipient_id` int(4) unsigned DEFAULT '0',
            `sender_name` varchar(128) NOT NULL DEFAULT '',
            `subject` varchar(128) NOT NULL DEFAULT '',
            `body` TEXT NOT NULL,
            `readed` BOOL NOT NULL DEFAULT false,
            PRIMARY KEY ( `id` ),
            KEY `unreaded_messages` ( `recipient_id`, `readed`, `creation_time` ) USING BTREE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::mailbox::create_tables"
    )
}

impl DbMailbox for PooledConn {
    /// посылает письмо одному из участников
    fn send_mail_to( &mut self, recipient_id: Id, sender_name: &str, subject: &str, body: &str ) -> CommonResult<Id> {
        send_mail_impl( self, recipient_id, sender_name, subject, body )
            .map_err( |e| fn_failed( "send_mail", e ) )
    }
    /// подсчитывает кол-во писем у определенного участника
    fn messages_count( &mut self, owner_id: Id, only_unreaded: bool ) -> CommonResult<u32> {
        messages_count_impl( self, owner_id, only_unreaded )
            .map_err( |e| fn_failed( "messages_count", e ) )
    }
    /// читает сообщения с пагинацией в обратном от создания порядке
    fn messages_from_last<F: FnMut(MailInfo)>( &mut self, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: &mut F ) -> EmptyResult {
        messages_from_last_impl( self, owner_id, only_unreaded, offset, count, take_mail )
            .map_err( |e| fn_failed( "messages_from_last", e ) )
    }
    /// помечает сообщение как прочитанное
    fn mark_as_readed( &mut self, owner_id: Id, message_ids: &[Id] ) -> EmptyResult {
        mark_as_readed_impl( self, owner_id, message_ids )
            .map_err( |e| fn_failed( "mark_as_readed", e ) )
    }

}

#[inline]
fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbMailbox {} failed: {}", fn_name, e ) )
}

fn send_mail_impl( conn: &mut PooledConn, recipient_id: Id, sender_name: &str, subject: &str, body: &str ) -> mysql::Result<Id> {
    let now_time = time::get_time();
    let mut stmt = try!( conn.prepare( "
        INSERT INTO mailbox (
            creation_time,
            recipient_id,
            sender_name,
            subject,
            body,
            readed
        )
        values( ?, ?, ?, ?, ?, ? )
    " ) );

    let readed = false;

    let params: &[ &ToValue ] = &[
        &now_time.msecs(),
        &recipient_id,
        &sender_name,
        &subject,
        &body,
        &readed
    ];
    let result = try!( stmt.execute( params ) );
    Ok( result.last_insert_id() )
}

fn messages_count_impl( conn: &mut PooledConn, owner_id: Id, only_unreaded: bool ) -> mysql::Result<u32> {
    let where_postfix = if only_unreaded { " AND readed='false'" } else { "" };
    let query = format!( "SELECT COUNT(id) FROM mailbox WHERE recipient_id=? {};", where_postfix );

    let mut stmt = try!( conn.prepare( &query ) );
    let mut sql_result = try!( stmt.execute( (owner_id,) ) );

    let sql_row = try!( sql_result.next().unwrap() );
    let (count,) = from_row( sql_row );
    Ok( count )
}

fn messages_from_last_impl<F: FnMut(MailInfo)>(
    conn: &mut PooledConn,
    owner_id: Id,
    only_unreaded: bool,
    offset: u32,
    count: u32,
    take_mail: &mut F
) -> mysql::Result<()>
{
    let where_postfix = if only_unreaded { "AND readed='false'" } else { "" };
    let order = if only_unreaded { "ASC" } else { "DESC" };
    let query = format!( "
        SELECT
            id,
            creation_time,
            sender_name,
            subject,
            body,
            readed
        FROM mailbox
        WHERE recipient_id = ? {}
        ORDER BY creation_time {}
        LIMIT ? OFFSET ?;
    ", where_postfix, order );
    let mut stmt = try!( conn.prepare( &query ) );
    let params: &[ &ToValue ] = &[ &owner_id, &count, &offset ];
    let sql_result = try!( stmt.execute( params ) );

    for sql_row in sql_result {
        let row = try!( sql_row );
        let (id, creation_time, sender_name, subject, body, readed) = from_row( row );
        let mail_info = MailInfo {
            id: id,
            creation_time: creation_time,
            sender_name: sender_name,
            subject: subject,
            body: body,
            readed: readed
        };
        take_mail( mail_info );
    }
    Ok( () )
}

fn mark_as_readed_impl( conn: &mut PooledConn, owner_id: Id, message_ids: &[Id] ) -> mysql::Result<()> {
    if message_ids.is_empty() {
        return Ok( () );
    }

    let mut query = format!("
        UPDATE mailbox
        SET readed=true
        WHERE id IN ( ?
    ");
    for _ in 1 .. message_ids.len() {
        query.push_str( ", ?" );
    }
    query.push_str( ") AND recipient_id=?" );

    let mut stmt = try!( conn.prepare( query ) );
    let mut values: Vec<Value> = Vec::new();
    for id in message_ids {
        values.push( id.to_value() );
    }
    values.push( owner_id.to_value() );

    try!( stmt.execute( values ) );
    Ok( () )
}
