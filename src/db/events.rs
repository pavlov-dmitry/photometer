use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value, ToValue };
use types::{ Id, CommonResult, EmptyResult };
use time;
use time::{ Timespec };
use std::fmt::Show;
use events::{ ScheduledEventInfo, EventState, FullEventInfo };

//type EventHandler<'a> = |EventInfo|:'a -> EmptyResult;
type EventInfos = Vec<ScheduledEventInfo>;

pub trait DbEvents {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<EventInfos>;
    /// считывает события которые исполняются в опеределенный момент
    fn active_events( &mut self, time: &Timespec ) -> CommonResult<EventInfos>;
    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<EventInfos>;
    /// информация о событии 
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<ScheduledEventInfo>>;
    /// текущая состояние события
    fn current_event_state( &mut self, scheduled_id: Id ) -> CommonResult<Option<EventState>>;
    /// добавляет события пачкой
    fn add_events( &mut self, events: &Vec<FullEventInfo> ) -> EmptyResult;
    /// помечает что данное событие завершено
    fn mark_event_as_finished( &mut self, scheduled_id: Id ) -> EmptyResult;
    
}

impl DbEvents for MyPooledConn {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, from: &Timespec, to: &Timespec ) -> CommonResult<EventInfos> {
        get_events_impl( self, "start_time BETWEEN ? AND ?", &[ &from.sec, &to.sec ] )
            .map_err( |e| fn_failed( "starting_events", e ) )
    }

    /// считывает события которые исполняются в опеределенный момент
    fn active_events( &mut self, time: &Timespec ) -> CommonResult<EventInfos> {
        get_events_impl( self, "? BETWEEN start_time AND end_time AND finished=false", &[ &time.sec ] )
            .map_err( |e| fn_failed( "active_events", e ) )
    }

    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, _from: &Timespec, to: &Timespec ) -> CommonResult<EventInfos> {
        get_events_impl( self, "end_time < ? AND finished=false", &[ &to.sec ] )
            .map_err( |e| fn_failed( "ending_events", e ) )
    }

    /// информация о событии 
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<ScheduledEventInfo>> {
        event_info_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "event_info", e ) )   
    }

    /// текущая состояние события
    fn current_event_state( &mut self, scheduled_id: Id ) -> CommonResult<Option<EventState>> {
        current_event_state_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "current_event_state", e ) )
    }

    /// добавляет события
    fn add_events( &mut self, events: &Vec<FullEventInfo> ) -> EmptyResult {
        add_events_impl( self, events )
            .map_err( |e| fn_failed( "add_events", e ) )
    }

    /// помечает что данное событие завершено
    fn mark_event_as_finished( &mut self, scheduled_id: Id ) -> EmptyResult {
        mark_event_as_finished_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "mark_event_as_finished", e ) )
    }
}

fn fn_failed<E: Show>( fn_name: &str, e: E ) -> String {
    format!( "DbEvents {} failed: {}", fn_name, e )
}

fn get_events_impl( conn: &mut MyPooledConn, where_cond: &str, values: &[&ToValue] ) -> MyResult<Vec<ScheduledEventInfo>> {
    let query = format!(
        "SELECT 
            id,
            event_id,
            event_name,
            data
        FROM scheduled_events
        WHERE {}",
        where_cond
    );
    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut sql_result = try!( stmt.execute( values ) );
    let mut events = Vec::new();
    for sql_row in sql_result {
        let row = try!( sql_row );
        let mut values = row.iter();
        events.push( ScheduledEventInfo {
            scheduled_id: from_value( values.next().unwrap() ),
            id: from_value( values.next().unwrap() ),
            name: from_value( values.next().unwrap() ),
            data: from_value( values.next().unwrap() )
        })
    }
    Ok( events )
}

fn event_info_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Option<ScheduledEventInfo>> {
    let mut stmt = try!( conn.prepare( 
        "SELECT 
            event_id,
            event_name,
            data
        FROM scheduled_events
        WHERE id = ?
    " ) );
    let mut sql_result = try!( stmt.execute( &[ &scheduled_id ] ) );
    let result = match sql_result.next() {
        Some( sql_row ) => {
            let row = try!( sql_row );
            let mut values = row.iter();
            Some( ScheduledEventInfo {
                id: from_value( values.next().unwrap() ),
                scheduled_id: scheduled_id,
                name: from_value( values.next().unwrap() ),
                data: from_value( values.next().unwrap() )
            })
        },
        None => None
    };
    Ok( result )
}

fn current_event_state_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Option<EventState>> {
    let mut stmt = try!( conn.prepare( 
        "SELECT 
            start_time,
            end_time,
        FROM scheduled_events
        WHERE id = ?
    " ) );
    let mut sql_result = try!( stmt.execute( &[ &scheduled_id ] ) );
    let result = match sql_result.next() {
        Some( sql_row ) => {
            let row = try!( sql_row );
            let mut values = row.iter();
            let start_time: i64 = from_value( values.next().unwrap() );
            let end_time: i64 = from_value( values.next().unwrap() );
            let current_time = time::get_time().sec;
            let result = if current_time < start_time {
                EventState::NotStartedYet
            }
            else if start_time < current_time && current_time < end_time {
                EventState::Active
            }
            else {
                EventState::Ended
            };
            Some( result )
        },
        None => None
    };
    Ok( result )
}

fn add_events_impl( conn: &mut MyPooledConn, events: &Vec<FullEventInfo> ) -> MyResult<()> {
    let mut query = format!( 
        "INSERT INTO scheduled_events (
            event_id,
            event_name,
            start_time,
            end_time,
            data
        )
        VALUES( ?, ?, ?, ?, ? )"
    );

    for _ in range( 1, events.len() ) {
        query.push_str( ", ( ?, ?, ?, ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut values: Vec<&ToValue> = Vec::new();
    for event in  events.iter() {
        values.push( &event.id );
        values.push( &event.name );
        values.push( &event.start_time.sec );
        values.push( &event.end_time.sec );
        values.push( &event.data );
    }

    try!( stmt.execute( values.as_slice() ) );
    Ok( () )
}

fn mark_event_as_finished_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare( "UPDATE scheduled_events SET finished=true WHERE id=?" ) );
    try!( stmt.execute( &[ &scheduled_id ] ) );
    Ok( () )
}