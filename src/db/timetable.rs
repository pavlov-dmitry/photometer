use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value, ToValue };
use types::{ CommonResult, Id, EmptyResult };
use time::Timespec;
use std::fmt::{ Show };

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
    fn add_new_timetable_version( &mut self, group_id: Id, new_timetable: &TimetableEvents ) -> EmptyResult;
}

impl DbTimetable for MyPooledConn {
    /// выбирает события которые должны стартануть за определенный период по версии расписания
    fn timetable_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<TimetableEvents> {
        timetable_events_impl( self, from, to )
            .map_err( |e| fn_failed( "starting_events", e ) )
    }  

    /// добавляет новый вариант расписания для группы
    fn add_new_timetable_version( &mut self, group_id: Id, new_timetable: &TimetableEvents ) -> EmptyResult {
        add_new_timetable_version_impl( self, group_id, new_timetable )
            .map_err( |e| fn_failed( "add_new_timetable_version", e ) )
    } 
}

fn fn_failed<E: Show>( fn_name: &str, e: E ) -> String {
    format!( "DbTimetable {} failed: {}", fn_name, e )
}

fn timetable_events_impl( conn: &mut MyPooledConn, from: &Timespec, to: &Timespec ) -> MyResult<TimetableEvents> {
    let mut stmt = try!( conn.prepare( "
        SELECT 
            group_id,
            event_id,
            event_name,
            start_time,
            end_time
            params
        FROM timetable
        WHERE version=? AND ? BETWEEN start_time AND end_time 
    "));
    let mut result = try!( stmt.execute( &[ &from.sec, &to.sec ] ) );

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

fn add_new_timetable_version_impl( conn: &mut MyPooledConn, group_id: Id, new_timetable: &TimetableEvents ) -> MyResult<()> {
    let max_version = try!( get_max_timetable_version( conn, group_id ) );
    let new_version = max_version + 1;
    let mut query = format!("
        INSERT INTO timetable (
            group_id
            event_id
            event_name
            start_time
            end_time
            params
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
    Ok( () )
}

fn get_max_timetable_version( conn: &mut MyPooledConn, group_id: Id ) -> MyResult<u32> {
    let mut stmt = try!( conn.prepare( "SELECT MAX( version ) FROM timetable WHERE group_id=?" ) );
    let mut result = try!( stmt.execute( &[ &group_id ] ) );
    let row = try!( result.next().unwrap() );
    Ok( from_value( &row[ 0 ] ) )
}