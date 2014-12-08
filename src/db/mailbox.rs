use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use time;
use time::{ Timespec };
use types::{ Id, CommonResult, EmptyResult, MailInfo };

pub trait DbMailbox {
    /// посылает письмо одному из участников
    fn send_mail( &mut self, recepient_id: Id, sender_name: &str, subject: &str, body: &str ) -> EmptyResult;
    /// подсчитывает кол-во писем у определенного участника
    fn messages_count( &mut self, owner_id: Id, only_unreaded: bool ) -> CommonResult<u32>;
    /// читает сообщения с пагинацией в обратном от создания порядке
    fn messages_from_last( &mut self, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: |&MailInfo| ) -> EmptyResult;
}

impl DbMailbox for MyPooledConn {
    /// посылает письмо одному из участников
    fn send_mail( &mut self, recepient_id: Id, sender_name: &str, subject: &str, body: &str ) -> EmptyResult {
        send_mail_impl( self, recepient_id, sender_name, subject, body )
            .map_err( |e| format!( "DbMailbox send_mail failed: {}", e ) )
    }
    /// подсчитывает кол-во писем у определенного участника
    fn messages_count( &mut self, owner_id: Id, only_unreaded: bool ) -> CommonResult<u32> {
        messages_count_impl( self, owner_id, only_unreaded )
            .map_err( |e| format!( "DbMailbox messages_count failed: {}", e ) )
    }
    /// читает сообщения с пагинацией в обратном от создания порядке
    fn messages_from_last( &mut self, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: |&MailInfo| ) -> EmptyResult {
        messages_from_last_impl( self, owner_id, only_unreaded, offset, count, take_mail )
            .map_err( |e| format!( "DbMailbox messages_from_last failes: {}", e ) )
    }
}

fn send_mail_impl( conn: &mut MyPooledConn, recepient_id: Id, sender_name: &str, subject: &str, body: &str ) -> MyResult<()> {
    let now_time = time::get_time();
    let mut stmt = try!( conn.prepare( "
        INSERT INTO mailbox (
            creation_time,
            recepient_id,
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
        &recepient_id,
        &sender_name,
        &subject,
        &body,
        &readed
    ]));
    Ok( () )
}

fn messages_count_impl( conn: &mut MyPooledConn, owner_id: Id, only_unreaded: bool ) -> MyResult<u32> {
    let where_postfix = if only_unreaded { " AND readed='false'" } else { "" };
    let query = format!( "SELECT COUNT(id) FROM mailbox WHERE recepient_id=? {};", where_postfix );
    
    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut sql_result = try!( stmt.execute( &[ &owner_id ] ) );
    
    let sql_row = try!( sql_result.next().unwrap() );
    Ok( from_value( &sql_row[ 0 ] ) )
}

fn messages_from_last_impl(conn: &mut MyPooledConn, owner_id: Id, only_unreaded: bool, offset: u32, count: u32, take_mail: |&MailInfo| ) -> MyResult<()> {
    let where_postfix = if only_unreaded { "AND readed='false'" } else { "" };
    let query = format!( "
        SELECT 
            creation_time,
            sender_name,
            subject,
            body,
            readed
        FROM mailbox
        WHERE recepient_id = ? {}
        ORDER BY creation_time DESC
        LIMIT ? OFFSET ?;
    ", where_postfix );
    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut sql_result = try!( stmt.execute( &[ &owner_id, &count, &offset ] ) );

    for sql_row in sql_result {
        let values = try!( sql_row );
        let mail_info = MailInfo {
            creation_time: Timespec::new( from_value( &values[ 0 ] ), 0 ),
            sender_name: from_value( &values[ 1 ] ),
            subject: from_value( &values[ 2 ] ),
            body: from_value( &values[ 3 ] ),
            readed: from_value( &values[ 4 ] )
        };
        take_mail( &mail_info );
    }
    Ok( () )
}