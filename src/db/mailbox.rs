use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use time;
use time::{ Timespec };
use types::{ Id, CommonResult, EmptyResult, MailInfo };
use std::fmt::{ Show };

pub trait DbMailbox {
    /// посылает письмо одному из участников
    fn send_mail( &mut self, recipient_id: Id, sender_name: &str, subject: &str, body: &str ) -> EmptyResult;
    /// подсчитывает кол-во писем у определенного участника
    fn messages_count( &mut self, owner_id: Id, only_unreaded: bool ) -> CommonResult<u32>;
    /// читает сообщения с пагинацией в обратном от создания порядке
    fn messages_from_last( &mut self, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: |&MailInfo| ) -> EmptyResult;
    /// помечает сообщение как прочитанное
    fn mark_as_readed( &mut self, owner_id: Id, message_id: Id ) -> CommonResult<bool>;
}

impl DbMailbox for MyPooledConn {
    /// посылает письмо одному из участников
    fn send_mail( &mut self, recipient_id: Id, sender_name: &str, subject: &str, body: &str ) -> EmptyResult {
        send_mail_impl( self, recipient_id, sender_name, subject, body )
            .map_err( |e| fn_failed( "send_mail", e ) )
    }
    /// подсчитывает кол-во писем у определенного участника
    fn messages_count( &mut self, owner_id: Id, only_unreaded: bool ) -> CommonResult<u32> {
        messages_count_impl( self, owner_id, only_unreaded )
            .map_err( |e| fn_failed( "messages_count", e ) )
    }
    /// читает сообщения с пагинацией в обратном от создания порядке
    fn messages_from_last( &mut self, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: |&MailInfo| ) -> EmptyResult {
        messages_from_last_impl( self, owner_id, only_unreaded, offset, count, take_mail )
            .map_err( |e| fn_failed( "messages_from_last", e ) )
    }
    /// помечает сообщение как прочитанное
    fn mark_as_readed( &mut self, owner_id: Id, message_id: Id ) -> CommonResult<bool> {
        mark_as_readed_impl( self, owner_id, message_id )
            .map_err( |e| fn_failed( "mark_as_readed", e ) )
    }

}

#[inline]
fn fn_failed<E: Show>( fn_name: &str, e: E ) -> String {
    format!( "DbMailbox {} failed: {}", fn_name, e )
}

fn send_mail_impl( conn: &mut MyPooledConn, recipient_id: Id, sender_name: &str, subject: &str, body: &str ) -> MyResult<()> {
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

    try!( stmt.execute( &[
        &now_time.sec,
        &recipient_id,
        &sender_name,
        &subject,
        &body,
        &readed
    ]));
    Ok( () )
}

fn messages_count_impl( conn: &mut MyPooledConn, owner_id: Id, only_unreaded: bool ) -> MyResult<u32> {
    let where_postfix = if only_unreaded { " AND readed='false'" } else { "" };
    let query = format!( "SELECT COUNT(id) FROM mailbox WHERE recipient_id=? {};", where_postfix );
    
    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut sql_result = try!( stmt.execute( &[ &owner_id ] ) );

    let sql_row = try!( sql_result.next().unwrap() );
    Ok( from_value( &sql_row[ 0 ] ) )
}

fn messages_from_last_impl( conn: &mut MyPooledConn, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: |&MailInfo| ) -> MyResult<()> {
    let where_postfix = if only_unreaded { "AND readed='false'" } else { "" };
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
        ORDER BY creation_time DESC
        LIMIT ? OFFSET ?;
    ", where_postfix );
    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut sql_result = try!( stmt.execute( &[ &owner_id, &count, &offset ] ) );

    for sql_row in sql_result {
        let values = try!( sql_row );
        let mail_info = MailInfo {
            id: from_value( &values[ 0 ] ),
            creation_time: Timespec::new( from_value( &values[ 1 ] ), 0 ),
            sender_name: from_value( &values[ 2 ] ),
            subject: from_value( &values[ 3 ] ),
            body: from_value( &values[ 4 ] ),
            readed: from_value( &values[ 5 ] )
        };
        take_mail( &mail_info );
    }
    Ok( () )
}

fn mark_as_readed_impl( conn: &mut MyPooledConn, owner_id: Id, message_id: Id ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "UPDATE mailbox SET readed=true WHERE id=? AND recipient_id=?" ) );
    let sql_result = try!( stmt.execute( &[ &message_id, &owner_id ] ) );
    Ok( 1 == sql_result.affected_rows() )
}