use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use types::{ CommonResult, Id };
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
}

impl DbTimetable for MyPooledConn {
    /// выбирает события которые должны стартануть за определенный период по версии расписания
    fn timetable_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<TimetableEvents> {
        timetable_events_impl( self, from, to )
            .map_err( |e| fn_failed( "starting_events", e ) )
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