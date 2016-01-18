use std::str;
use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult, MyError };
use mysql::value::{
    IntoValue,
    from_row,
    ToValue,
    ToRow,
    FromValue,
    Value,
    ConvIr
};
use types::{ Id, CommonResult, EmptyResult, CommonError };
use time::{ Timespec };
use std::fmt::Display;
use events::{ EventId, MaybeEventId, ScheduledEventInfo, EventState, FullEventInfo };
use database::Database;
use parse_utils::{ GetMsecs, IntoTimespec };

type EventInfos = Vec<ScheduledEventInfo>;
type UnwatchedInfos = Vec<(Id, u32)>;

pub trait DbEvents {
    /// считывает события которые должны стратануть за период
    fn starting_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos>;
    /// считывает события которые исполняются в опеределенный момент
    fn active_events( &mut self ) -> CommonResult<EventInfos>;
    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos>;
    /// информация о событии
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<ScheduledEventInfo>>;
    /// информация о событиях
    fn event_infos( &mut self, scheduled_ids: &[Id] ) -> CommonResult<EventInfos>;
    /// добавляет события пачкой
    fn add_events( &mut self, events: &[FullEventInfo] ) -> EmptyResult;
    fn add_disabled_event( &mut self, event: &FullEventInfo ) -> CommonResult<Id>;
    /// помечает что данное событие завершено
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult;
    /// информация о вермени начала определeнного события
    fn event_start_time( &mut self, scheduled_id: Id ) -> CommonResult<Option<Timespec>>;
    /// кол-во непросмотренных событий в группах для пользователя
    fn get_unwatched_events_by_group( &mut self, user_id: Id ) -> CommonResult<UnwatchedInfos>;
    /// возвращает текущее расписание группы
    fn get_group_timetable( &mut self, group_id: Id ) -> CommonResult<EventInfos>;
    /// "выключает" событие если оно еще не началось
    fn disable_event_if_not_started( &mut self, scheduled_id: Id ) -> EmptyResult;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `scheduled_events` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `event_id` int(4) NOT NULL DEFAULT '0',
            `event_name` varchar(128) NOT NULL DEFAULT '',
            `start_time` bigint(20) NOT NULL DEFAULT '0',
            `end_time` bigint(20) NOT NULL DEFAULT '0',
            `data` TEXT NOT NULL DEFAULT '',
            `state` ENUM(
                'not_started_yet',
                'active',
                'finished',
                'disabled'
            ) NOT NULL DEFAULT 'not_started_yet',
            `user_editable` BOOL NOT NULL DEFAULT false,
            `group_attached` BOOL NOT NULL DEFAULT false,
            `group_id` bigint(20) NOT NULL DEFAULT '0',
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
        let params: &[&ToValue] = &[ &moment.msecs() ];
        get_events_impl( self, "start_time <= ? AND state='not_started_yet'", params )
            .map_err( |e| fn_failed( "starting_events", e ) )
    }

    /// считывает события которые исполняются в опеределенный момент
    fn active_events( &mut self ) -> CommonResult<EventInfos> {
        let params: &[&ToValue] = &[];
        get_events_impl( self, "state='active'", params )
            .map_err( |e| fn_failed( "active_events", e ) )
    }

    /// считывает собыятия которые должны закончится за период
    fn ending_events( &mut self, moment: &Timespec ) -> CommonResult<EventInfos> {
        let params: &[&ToValue] = &[ &moment.msecs() ];
        get_events_impl( self, "end_time <= ? AND state='active'", params )
            .map_err( |e| fn_failed( "ending_events", e ) )
    }

    /// информация о событии
    fn event_info( &mut self, scheduled_id: Id ) -> CommonResult<Option<ScheduledEventInfo>> {
        event_info_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "event_info", e ) )
    }

    /// информация о событиях
    fn event_infos( &mut self, scheduled_ids: &[Id] ) -> CommonResult<EventInfos> {
        event_infos_impl( self, scheduled_ids )
            .map_err( |e| fn_failed( "event_infos", e ) )
    }

    /// добавляет события
    fn add_events( &mut self, events: &[FullEventInfo] ) -> EmptyResult {
        add_events_impl( self, events )
            .map_err( |e| fn_failed( "add_events", e ) )
    }
    fn add_disabled_event( &mut self, event: &FullEventInfo ) -> CommonResult<Id> {
        add_disabled_event_impl( self, event )
            .map_err( |e| fn_failed( "add_events", e ) )
    }

    /// помечает что данное событие завершено
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult {
        set_event_state_impl( self, scheduled_id, state )
            .map_err( |e| fn_failed( "set_event_state", e ) )
    }

    /// информация о вермени начала определeнного события
    fn event_start_time( &mut self, scheduled_id: Id ) -> CommonResult<Option<Timespec>> {
        event_start_time_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "event_start_time", e ) )
    }

    /// кол-во непросмотренных событий в группах для пользователя
    fn get_unwatched_events_by_group( &mut self, user_id: Id ) -> CommonResult<UnwatchedInfos> {
        get_unwatched_events_by_group_impl( self, user_id )
            .map_err( |e| fn_failed( "get_unwatched_events_by_group", e ) )
    }

    /// возвращает текущее расписание группы
    fn get_group_timetable( &mut self, group_id: Id ) -> CommonResult<EventInfos> {
        let params: &[&ToValue] = &[ &group_id ];
        get_events_impl( self,
                         "state='not_started_yet' AND user_editable=true AND group_attached=true AND group_id=?",
                         params )
            .map_err( |e| fn_failed( "get_group_timetable", e ) )
    }

    /// "выключает" событие если оно еще не началось
    fn disable_event_if_not_started( &mut self, scheduled_id: Id ) -> EmptyResult {
        disable_event_if_not_started_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "disable_event_if_not_started", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbEvents {} failed: {}", fn_name, e ) )
}

const NOT_STARTED_YET_STR : &'static str = "not_started_yet";
const ACTIVE_STR : &'static str = "active";
const FINISHED_STR : &'static str = "finished";
const DISABLED_STR : &'static str = "disabled";

impl ToValue for EventState {
    fn to_value(&self) -> Value {
        let bytes = match self {
            &EventState::NotStartedYet => NOT_STARTED_YET_STR.bytes().collect(),
            &EventState::Active => ACTIVE_STR.bytes().collect(),
            &EventState::Finished => FINISHED_STR.bytes().collect(),
            &EventState::Disabled => DISABLED_STR.bytes().collect(),
        };
        Value::Bytes( bytes )
    }
}

pub struct EventStateIr {
    val: EventState,
    bytes: Vec<u8>
}

impl ConvIr<EventState> for EventStateIr
{
    fn new(v: Value) -> MyResult<EventStateIr> {
        match v {
            Value::Bytes( bytes ) => {
                let val = match str::from_utf8( &bytes ) {
                    Ok( s ) => match s {
                        NOT_STARTED_YET_STR => Some( EventState::NotStartedYet ),
                        ACTIVE_STR => Some( EventState::Active ),
                        FINISHED_STR => Some( EventState::Finished ),
                        DISABLED_STR => Some( EventState::Disabled ),
                        _ => None
                    },
                    _ => None
                };
                match val {
                    Some( val ) => Ok( EventStateIr{ val: val, bytes: bytes } ),
                    None => Err( MyError::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err( MyError::FromValueError( v ) )
        }
    }
    fn commit(self) -> EventState {
        self.val
    }
    fn rollback(self) -> Value {
        Value::Bytes( self.bytes )
    }
}

impl FromValue for EventState {
    type Intermediate = EventStateIr;
}

fn get_events_impl<T: ToRow>( conn: &mut MyPooledConn, where_cond: &str, values: T ) -> MyResult<Vec<ScheduledEventInfo>> {
    let query = format!(
        "SELECT
            id,
            event_id,
            start_time,
            end_time,
            event_name,
            data,
            state,
            group_attached,
            group_id
        FROM scheduled_events
        WHERE {}",
        where_cond
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let sql_result = try!( stmt.execute( values ) );
    let mut events = Vec::new();
    for sql_row in sql_result {
        let row = try!( sql_row );
        let ( scheduled_id,
              id,
              start_time,
              end_time,
              name,
              data,
              state,
              group_attached,
              group_id )
            = from_row::<(Id, EventId, u64, u64, String, String, EventState, bool, Id)>( row );

        events.push( ScheduledEventInfo {
            scheduled_id: scheduled_id,
            id: id,
            start_time: start_time.into_timespec(),
            end_time: end_time.into_timespec(),
            name: name,
            data: data,
            state: state,
            group: if group_attached { Some( group_id ) } else { None }
        })
    }
    Ok( events )
}

fn event_info_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Option<ScheduledEventInfo>> {
    let mut stmt = try!( conn.prepare(
        "SELECT
            event_id,
            start_time,
            end_time,
            event_name,
            data,
            state,
            group_attached,
            group_id
        FROM scheduled_events
        WHERE id = ?
    " ) );
    let params: &[ &ToValue ] = &[ &scheduled_id ];
    let mut sql_result = try!( stmt.execute( params ) );
    let result = match sql_result.next() {
        Some( sql_row ) => {
            let row = try!( sql_row );
            let ( id,
                  start_time,
                  end_time,
                  name,
                  data,
                  state,
                  group_attached,
                  group_id )
                = from_row::<(EventId, u64, u64, String, String, EventState, bool, Id)>( row );

            Some( ScheduledEventInfo {
                id: id,
                start_time: start_time.into_timespec(),
                end_time: end_time.into_timespec(),
                scheduled_id: scheduled_id,
                name: name,
                data: data,
                state: state,
                group: if group_attached { Some( group_id ) } else { None }
            })
        },
        None => None
    };
    Ok( result )
}

fn event_infos_impl( conn: &mut MyPooledConn, scheduled_ids: &[Id] ) -> MyResult<EventInfos> {
    // проверка на пустату
    if scheduled_ids.is_empty() {
        return Ok( Vec::new() );
    }

    let mut where_cond = String::from( "id in ( ?" );
    for _ in 1..scheduled_ids.len() {
        where_cond.push_str( ", ?" );
    }
    where_cond.push_str( ")" );
    let values: Vec<Value> = scheduled_ids
        .iter()
        .map( |id| id.into_value() )
        .collect();
    get_events_impl( conn, &where_cond, &values )
}

fn event_start_time_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<Option<Timespec>> {
    let mut stmt = try!( conn.prepare("
        SELECT
            `start_time`
        FROM
            `scheduled_events`
        WHERE
            `id` = ?
    ") );

    let params: &[ &ToValue ] = &[ &scheduled_id ];
    let mut sql_result = try!( stmt.execute( params ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            let (msecs,): (u64,) = from_row( row );
            Some( msecs.into_timespec() )
        }
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
            data,
            group_attached,
            group_id
        )
        VALUES( ?, ?, ?, ?, ?, ?, ? )"
    );

    for _ in 1 .. events.len() {
        query.push_str( ", ( ?, ?, ?, ?, ?, ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( &query ) );
    let mut values: Vec<Value> = Vec::new();
    for i in 0 .. events.len() {
        let event = &events[ i ];
        values.push( event.id.into_value() );
        values.push( event.name.clone().into_value() );
        values.push( event.start_time.msecs().into_value() );
        values.push( event.end_time.msecs().into_value() );
        values.push( event.data.clone().into_value() );
        let (group_attached, group_id) = to_group_info( event.group );
        values.push( group_attached.into_value() );
        values.push( group_id.into_value() );
    }

    try!( stmt.execute( values ) );
    Ok( () )
}

fn add_disabled_event_impl( conn: &mut MyPooledConn, event: &FullEventInfo ) -> MyResult<Id> {
    let mut stmt = try!( conn.prepare("
        INSERT
            INTO `scheduled_events` (
                `event_id`,
                `event_name`,
                `start_time`,
                `end_time`,
                `data`,
                `state`,
                `user_editable`,
                `group_attached`,
                `group_id`
            )
        VALUES ( ?, ?, ?, ?, ?, ?, true, ?, ? )
    ") );

    let (group_attached, group_id) = to_group_info( event.group );
    let params: &[ &ToValue ] = &[
        &event.id,
        &event.name,
        &event.start_time.msecs(),
        &event.end_time.msecs(),
        &event.data,
        &EventState::Disabled,
        &group_attached,
        &group_id
    ];
    let result = try!( stmt.execute( params ) );

    Ok( result.last_insert_id() )
}

fn set_event_state_impl( conn: &mut MyPooledConn, scheduled_id: Id, state: EventState ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare( "UPDATE scheduled_events SET state=? WHERE id=?" ) );
    let params: &[ &ToValue ] = &[ &state, &scheduled_id ];
    try!( stmt.execute( params ) );
    Ok( () )
}

/// "выключает" событие если оно еще не началось
fn disable_event_if_not_started_impl( conn: &mut MyPooledConn, scheduled_id: Id ) -> MyResult<()> {
    let mut stmt = try!( conn.prepare( "
        UPDATE
            `scheduled_events`
        SET
            `state`='disabled'
        WHERE
            `id`=? AND
            `state`='not_started_yet'
    " ) );
    let params: &[ &ToValue ] = &[ &scheduled_id ];
    try!( stmt.execute( params ) );
    Ok( () )
}

/// кол-во непросмотренных событий в группах для пользователя
fn get_unwatched_events_by_group_impl( conn: &mut MyPooledConn, user_id: Id ) -> MyResult<UnwatchedInfos> {
    let query = "SELECT
                     se.group_id, COUNT( se.id )
                 FROM
                     scheduled_events AS se
                 LEFT JOIN
                     group_members AS gm ON ( gm.group_id = se.group_id )
                 WHERE
                     gm.user_id = ? AND se.end_time > gm.last_visited_time
                 GROUP BY
                     se.group_id";
    let mut stmt = try!( conn.prepare( query ) );
    let sql_result = try!( stmt.execute( (user_id,) ) );

    let mut result = Vec::new();
    for row in sql_result {
        let row = try!( row );
        result.push( from_row( row ) );
    }
    Ok( result )
}

fn to_group_info( group: Option<Id> ) -> (bool, Id) {
    match group {
        Some( id ) => (true, id),
        None => (false, 0)
    }
}

impl ToValue for EventId {
    fn to_value(&self) -> Value {
        Value::UInt( *self as u64 )
    }
}

pub struct EventIdIr
{
    val: EventId,
    raw: i64
}

impl ConvIr<EventId> for EventIdIr {
    fn new(v: Value) -> MyResult<EventIdIr> {
        match v {
            Value::Int( num ) => {
                let maybe_event_id: MaybeEventId = From::from( num );
                let MaybeEventId( maybe_id ) = maybe_event_id;
                match maybe_id {
                    Some( id ) => Ok( EventIdIr {
                        val: id,
                        raw: num
                    }),
                    None => Err( MyError::FromValueError( Value::Int( num ) ) )
                }
            }
            _ => Err( MyError::FromValueError( v ) )
        }
    }
    fn commit(self) -> EventId {
        self.val
    }
    fn rollback(self) -> Value {
        Value::Int( self.raw )
    }
}

impl FromValue for EventId {
    type Intermediate = EventIdIr;
}
