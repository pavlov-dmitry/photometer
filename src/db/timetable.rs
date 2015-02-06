use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value, from_value_opt, ToValue };
use types::{ CommonResult, Id, EmptyResult };
use time::Timespec;
use std::fmt::Display;
use database::Database;

pub struct TimetableEventInfo {
    pub group_id: Id,
    pub event_id: Id,
    pub event_name: String,
    pub start_time: Timespec,
    pub end_time: Timespec,
    pub params: String
}

type TimetableEvents = Vec<TimetableEventInfo>;

pub trait DbTimetable {
    /// выбирает события которые должны стартануть за определенный период по версии расписания
    fn timetable_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<TimetableEvents>;
    /// добавляет новый вариант расписания для группы
    fn add_new_timetable_version( &mut self, group_id: Id, new_timetable: &TimetableEvents ) -> CommonResult<u32>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(  
        "CREATE TABLE IF NOT EXISTS `timetable` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `group_id` bigint(20) NOT NULL DEFAULT '0',
            `event_id` int(4) NOT NULL DEFAULT '0',
            `event_name` varchar(128) NOT NULL DEFAULT '',
            `start_time` int(11) NOT NULL DEFAULT '0',
            `end_time` int(11) NOT NULL DEFAULT '0',
            `params` TEXT NOT NULL DEFAULT '',
            `version` int(4) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` ),
            KEY `time_idx` ( `start_time`, `end_time`, `version` ) USING BTREE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::timetable::create_tables"
    )
}

impl DbTimetable for MyPooledConn {
    /// выбирает события которые должны стартануть за определенный период по версии расписания
    fn timetable_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<TimetableEvents> {
        timetable_events_impl( self, from, to )
            .map_err( |e| fn_failed( "starting_events", e ) )
    }  

    /// добавляет новый вариант расписания для группы
    fn add_new_timetable_version( &mut self, group_id: Id, new_timetable: &TimetableEvents ) -> CommonResult<u32> {
        add_new_timetable_version_impl( self, group_id, new_timetable )
            .map_err( |e| fn_failed( "add_new_timetable_version", e ) )
    } 
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> String {
    format!( "DbTimetable {} failed: {}", fn_name, e )
}

fn timetable_events_impl( conn: &mut MyPooledConn, from: &Timespec, to: &Timespec ) -> MyResult<TimetableEvents> {
    /*let mut stmt = try!( conn.prepare( "
        SELECT 
            group_id,
            event_id,
            event_name,
            start_time,
            end_time,
            params
        FROM timetable
        WHERE version=(SELECT timetable_version FROM groups WHERE groups.id=timetable.group_id) AND (start_time BETWEEN ? AND ?) 
    "));*/

    let mut stmt = try!( conn.prepare( "
        SELECT 
            `tt`.`group_id`,
            `tt`.`event_id`,
            `tt`.`event_name`,
            `tt`.`start_time`,
            `tt`.`end_time`,
            `tt`.`params`
        FROM
            `timetable` AS `tt`
        LEFT JOIN
            `groups` AS `g` ON ( `tt`.`version` = `g`.`timetable_version` AND `g`.`id` = `tt`.`group_id` )
        WHERE
            `g`.`id` IS NOT NULL
            AND ( ? < `tt`.`start_time` AND `tt`.`start_time` <= ? )
    "));
    
    let result = try!( stmt.execute( &[ &from.sec, &to.sec ] ) );

    let mut events = Vec::new();

    for row in result {
        let row = try!( row );
        let mut values = row.iter();

        events.push( TimetableEventInfo {
            group_id: from_value( values.next().unwrap() ),
            event_id: from_value( values.next().unwrap() ),
            event_name: from_value( values.next().unwrap() ),
            start_time: Timespec::new( from_value( values.next().unwrap() ), 0 ),
            end_time: Timespec::new( from_value( values.next().unwrap() ), 0 ),
            params: from_value( values.next().unwrap() ),
        });
    }
    Ok( events )
}

fn add_new_timetable_version_impl( conn: &mut MyPooledConn, group_id: Id, new_timetable: &TimetableEvents ) -> MyResult<u32> {
    let max_version = try!( get_max_timetable_version( conn, group_id ) );
    let new_version = max_version + 1;
    let mut query = format!("
        INSERT INTO timetable (
            group_id,
            event_id,
            event_name,
            start_time,
            end_time,
            params,
            version
        )
        VALUES( ?, ?, ?, ?, ?, ?, ? )
    ");

    for _ in range( 1, new_timetable.len() ) {
        query.push_str( ", ( ?, ?, ?, ?, ?, ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut values: Vec<&ToValue> = Vec::new();
    for event in new_timetable.iter() {
        values.push( &group_id );
        values.push( &event.event_id );
        values.push( &event.event_name );
        values.push( &event.start_time.sec );
        values.push( &event.end_time.sec );
        values.push( &event.params );
        values.push( &new_version );
    }

    try!( stmt.execute( values.as_slice() ) );
    Ok( new_version )
}

fn get_max_timetable_version( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<u32> {
    let mut stmt = try!( conn.prepare( "SELECT MAX( version ) FROM timetable WHERE group_id=?" ) );
    let mut result = try!( stmt.execute( &[ &group_id ] ) );
    let row = try!( result.next().unwrap() );
    match from_value_opt( &row[ 0 ] ) {
        Some( val ) => Ok( val ),
        None => Ok( 0 )
    }
}