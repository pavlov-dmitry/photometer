use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use types::{ Id, CommonResult, EmptyResult, EventInfo, EventState };
use time;
use time::{ Timespec };

pub trait DbEvents {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, from: &Timespec, to: &Timespec, event: |EventInfo| ) -> EmptyResult;
    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, from: &Timespec, to: &Timespec, event: |EventInfo| ) -> EmptyResult;
    /// информация о событии 
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<EventInfo>>;
    /// текущая состояние события
    fn current_event_state( &mut self, scheduled_id: Id ) -> CommonResult<Option<EventState>>;
}

impl DbEvents for MyPooledConn {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, from: &Timespec, to: &Timespec, event: |EventInfo| ) -> EmptyResult {
        get_events_impl( self, from, to, event, "start_time" )
            .map_err( |e| format!( "DbEvents starting_events failed: {}", e ) )
    }

    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, from: &Timespec, to: &Timespec, event: |EventInfo| ) -> EmptyResult {
        get_events_impl( self, from, to, event, "end_time" )
            .map_err( |e| format!( "DbEvents ending_events failed: {}", e ) )
    }

    /// информация о событии 
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<EventInfo>> {
        event_info_impl( self, scheduled_id )
            .map_err( |e| format!( "DbEvents event_info failed: {}", e ) )   
    }

    /// текущая состояние события
    fn current_event_state( &mut self, scheduled_id: Id ) -> CommonResult<Option<EventState>> {
        current_event_state_impl( self, scheduled_id )
            .map_err( |e| format!( "DbEvents current_event_state failed: {}", e ) )
    }

}

fn get_events_impl( conn: &mut MyPooledConn, from: &Timespec, to: &Timespec, event: |EventInfo|, time: &str ) -> MyResult<()> {
    let query = format!(
        "SELECT 
            id,
            event_id,
            data
        FROM scheduled_events
        WHERE {} BETWEEN ? AND ?",
        time
    );
    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut sql_result = try!( stmt.execute( &[ &from.sec, &to.sec ] ) );
    for sql_row in sql_result {
        let row = try!( sql_row );
        let mut values = row.iter();
        event( EventInfo {
            scheduled_id: from_value( values.next().unwrap() ),
            id: from_value( values.next().unwrap() ),
            data: from_value( values.next().unwrap() )
        });
    }
    Ok( () )
}

fn event_info_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Option<EventInfo>> {
    let mut stmt = try!( conn.prepare( 
        "SELECT 
            event_id,
            data,
        FROM scheduled_events
        WHERE id = ?
    " ) );
    let mut sql_result = try!( stmt.execute( &[ &scheduled_id ] ) );
    let result = match sql_result.next() {
        Some( sql_row ) => {
            let row = try!( sql_row );
            let mut values = row.iter();
            Some( EventInfo {
                id: from_value( values.next().unwrap() ),
                scheduled_id: scheduled_id,
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
