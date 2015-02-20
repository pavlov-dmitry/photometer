use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value, ToValue, FromValue, Value, from_value_opt };
use types::{ Id, CommonResult, EmptyResult };
use time::{ Timespec };
use std::fmt::Display;
use events::{ ScheduledEventInfo, EventState, FullEventInfo };
use database::Database;

//type EventHandler<'a> = |EventInfo|:'a -> EmptyResult;
type EventInfos = Vec<ScheduledEventInfo>;

pub trait DbEvents {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos>;
    /// считывает события которые исполняются в опеределенный момент
    fn active_events( &mut self ) -> CommonResult<EventInfos>;
    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos>;
    /// информация о событии 
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<ScheduledEventInfo>>;
    /// добавляет события пачкой
    fn add_events( &mut self, events: &[FullEventInfo] ) -> EmptyResult;
    /// помечает что данное событие завершено
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult;
    
}

/// scheduled_events.state { NOT_STARTED_YET = 0, ACTIVE = 1, FINISHED = 2 }

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `scheduled_events` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `event_id` int(4) NOT NULL DEFAULT '0',
            `event_name` varchar(128) NOT NULL DEFAULT '',
            `start_time` int(11) NOT NULL DEFAULT '0',
            `end_time` int(11) NOT NULL DEFAULT '0',
            `data` TEXT NOT NULL DEFAULT '',
            `state` ENUM( 
                'not_started_yet', 
                'active', 
                'finished', 
                'disabled' 
            ) NOT NULL DEFAULT 'not_started_yet',
            `user_editable` BOOL NOT NULL DEFAULT false
            PRIMARY KEY ( `id` ),
            KEY `time_idx` ( `start_time`, `end_time`, `state` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::events::create_tables"
    )
}

impl DbEvents for MyPooledConn {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos> {
        get_events_impl( self, "start_time <= ? AND state='not_started_yet'", &[ &moment.sec ] )
            .map_err( |e| fn_failed( "starting_events", e ) )
    }

    /// считывает события которые исполняются в опеределенный момент
    fn active_events( &mut self ) -> CommonResult<EventInfos> {
        get_events_impl( self, "state='active'", &[] )
            .map_err( |e| fn_failed( "active_events", e ) )
    }

    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos> {
        get_events_impl( self, "end_time <= ? AND state='active'", &[ &moment.sec ] )
            .map_err( |e| fn_failed( "ending_events", e ) )
    }

    /// информация о событии 
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<ScheduledEventInfo>> {
        event_info_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "event_info", e ) )   
    }

    /// добавляет события
    fn add_events( &mut self, events: &[FullEventInfo] ) -> EmptyResult {
        add_events_impl( self, events )
            .map_err( |e| fn_failed( "add_events", e ) )
    }

    /// помечает что данное событие завершено
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult {
        set_event_state_impl( self, scheduled_id, state )
            .map_err( |e| fn_failed( "set_event_state", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> String {
    format!( "DbEvents {} failed: {}", fn_name, e )
}

const NOT_STARTED_YET_STR : &'static str = "not_started_yet";
const ACTIVE_STR : &'static str = "active";
const FINISHED_STR : &'static str = "finished";
const DISABLED_STR : &'static str = "disabled";

impl ToValue for EventState {
    fn to_value(&self) -> Value {
        match self {
            &EventState::NotStartedYet => NOT_STARTED_YET_STR.to_value(),
            &EventState::Active => ACTIVE_STR.to_value(),
            &EventState::Finished => FINISHED_STR.to_value(),
            &EventState::Disabled => DISABLED_STR.to_value()
        }
    }
}

impl FromValue for EventState {
    fn from_value(v: &Value) -> EventState {
        from_value_opt::<EventState>( v ).expect( "fail converting EventState from db value!" )
    }
    fn from_value_opt(v: &Value) -> Option<EventState> {
        from_value_opt::<String>( v )
            .and_then( |string| match string.as_slice() {
                NOT_STARTED_YET_STR => Some( EventState::NotStartedYet ),
                ACTIVE_STR => Some( EventState::Active ),
                FINISHED_STR => Some( EventState::Finished ),
                DISABLED_STR => Some( EventState::Disabled ),
                _ => None
            })
    }
}

fn get_events_impl( conn: &mut MyPooledConn, where_cond: &str, values: &[&ToValue] ) -> MyResult<Vec<ScheduledEventInfo>> {
    let query = format!(
        "SELECT 
            id,
            event_id,
            event_name,
            data,
            state
        FROM scheduled_events
        WHERE {}",
        where_cond
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let sql_result = try!( stmt.execute( values ) );
    let mut events = Vec::new();
    for sql_row in sql_result {
        let row = try!( sql_row );
        let mut values = row.iter();
        events.push( ScheduledEventInfo {
            scheduled_id: from_value( values.next().unwrap() ),
            id: from_value( values.next().unwrap() ),
            name: from_value( values.next().unwrap() ),
            data: from_value( values.next().unwrap() ),
            state: from_value( values.next().unwrap() ),
        })
    }
    Ok( events )
}

fn event_info_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Option<ScheduledEventInfo>> {
    let mut stmt = try!( conn.prepare( 
        "SELECT 
            event_id,
            event_name,
            data,
            state
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
                data: from_value( values.next().unwrap() ),
                state: from_value( values.next().unwrap() )
            })
        },
        None => None
    };
    Ok( result )
}

fn add_events_impl( conn: &mut MyPooledConn, events: &[FullEventInfo] ) -> MyResult<()> {
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

    let mut stmt = try!( conn.prepare( &query ) );
    let mut values: Vec<&ToValue> = Vec::new();
    for i in range( 0, events.len() ) {
        let event = &events[ i ];
        values.push( &event.id );
        values.push( &event.name );
        values.push( &event.start_time.sec );
        values.push( &event.end_time.sec );
        values.push( &event.data );
    }

    try!( stmt.execute( &values ) );
    Ok( () )
}

fn set_event_state_impl( conn: &mut MyPooledConn, scheduled_id: Id, state: EventState ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare( "UPDATE scheduled_events SET state=? WHERE id=?" ) );
    try!( stmt.execute( &[ &state, &scheduled_id ] ) );
    Ok( () )
}