use std::str;
use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::error::{ Result, Error };
use mysql::value::{
    from_row,
    from_value,
    ToValue,
    FromValue,
    Value,
    ConvIr
};
use types::{
    Id,
    CommonResult,
    EmptyResult,
    CommonError,
    ShortInfo
};
use time::{ Timespec };
use std::fmt::Display;
use events::{
    EventId,
    MaybeEventId,
    ScheduledEventInfo,
    EventState,
    FullEventInfo
};
use database::Database;
use parse_utils::{ GetMsecs, IntoTimespec };

pub type EventInfos = Vec<ScheduledEventInfo>;

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
    /// возвращает текущее расписание группы
    fn get_group_timetable( &mut self, group_id: Id ) -> CommonResult<EventInfos>;
    /// "выключает" событие если оно еще не началось
    fn disable_event_if_not_started( &mut self, scheduled_id: Id ) -> EmptyResult;
    /// увеличивает счётчие комментариев к событию
    fn increment_event_comments_count( &mut self, scheduled_id: Id ) -> EmptyResult;
    /// выбирает информацию о следующей событии в группу
    fn get_next_event_after( &mut self, moment: &Timespec, event_id: EventId ) -> CommonResult<Option<(Id, Timespec)>>;
    /// обновляет время начала для события
    fn update_event_start_time( &mut self, scheduled_id: Id, new_start: &Timespec ) -> EmptyResult;
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
            `creator_id` bigint(20) NOT NULL DEFAULT '0',
            `comments_count` int(11) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` ),
            KEY `time_idx` ( `start_time`, `end_time`, `state` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::events::create_tables"
    )
}

impl DbEvents for PooledConn {
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

    /// увеличивает счётчие комментариев к событию
    fn increment_event_comments_count( &mut self, scheduled_id: Id ) -> EmptyResult {
        increment_event_comments_count_impl( self, scheduled_id )
            .map_err( |e| fn_failed( "increment_event_comments_count", e ) )
    }

    /// выбирает информацию о следующей событии в группу
    fn get_next_event_after( &mut self, moment: &Timespec, event_id: EventId ) -> CommonResult<Option<(Id, Timespec)>> {
        get_next_event_after_impl( self, moment, event_id )
            .map_err( |e| fn_failed( "get_next_event_after", e ) )
    }

    /// обновляет время начала для события
    fn update_event_start_time( &mut self, scheduled_id: Id, new_start: &Timespec ) -> EmptyResult {
        update_event_start_time_impl( self, scheduled_id, new_start )
            .map_err( |e| fn_failed( "update_event_start_time", e ) )
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
    fn new(v: Value) -> Result<EventStateIr> {
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
                    _ => None,
                };
                match val {
                    Some( val ) => Ok( EventStateIr{ val: val, bytes: bytes } ),
                    None => Err( Error::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err( Error::FromValueError( v ) )
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

fn get_events_impl<T: Into<mysql::Params>>( conn: &mut PooledConn, where_cond: &str, values: T ) -> Result<Vec<ScheduledEventInfo>> {
    let query = format!(
        "SELECT
            e.id,
            e.event_id,
            e.start_time,
            e.end_time,
            e.event_name,
            e.data,
            e.state,
            e.group_attached,
            e.group_id,
            g.name,
            e.creator_id,
            u.login
        FROM scheduled_events AS e
        LEFT JOIN groups AS g ON ( g.id = e.group_id )
        LEFT JOIN users AS u ON ( u.id = e.creator_id )
        WHERE {}",
        where_cond
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let sql_result = try!( stmt.execute( values ) );
    let mut events = Vec::new();
    for sql_row in sql_result {
        let row = try!( sql_row );
        let mut values = row.unwrap().into_iter();

        let scheduled_id = from_value( values.next().unwrap() );
        let id = from_value( values.next().unwrap() );
        let start_time: u64 = from_value( values.next().unwrap() );
        let end_time: u64 = from_value( values.next().unwrap() );
        let name = from_value( values.next().unwrap() );
        let data = from_value( values.next().unwrap() );
        let state = from_value( values.next().unwrap() );
        let group_attached = from_value( values.next().unwrap() );
        let group_id = from_value( values.next().unwrap() );
        let group_name: Option<String> = from_value( values.next().unwrap() );
        let creator_id = from_value( values.next().unwrap() );
        let creator_name: Option<String> = from_value( values.next().unwrap() );

        events.push( ScheduledEventInfo {
            scheduled_id: scheduled_id,
            id: id,
            start_time: start_time.into_timespec(),
            end_time: end_time.into_timespec(),
            name: name,
            data: data,
            state: state,
            group: match group_attached {
                true => Some( ShortInfo{
                    id: group_id,
                    name: group_name.unwrap()
                }),
                false => None
            },
            creator: match creator_id {
                0 => None,
                _ => Some( ShortInfo {
                    id: creator_id,
                    name: creator_name.unwrap(),
                })
            }
        })
    }
    Ok( events )
}

fn event_info_impl( conn: &mut PooledConn, scheduled_id: Id ) -> mysql::Result<Option<ScheduledEventInfo>> {
    let mut stmt = try!( conn.prepare(
        "SELECT
            e.event_id,
            e.start_time,
            e.end_time,
            e.event_name,
            e.data,
            e.state,
            e.group_attached,
            e.group_id,
            g.name,
            e.creator_id,
            u.login
        FROM scheduled_events AS e
        LEFT JOIN groups AS g ON ( g.id = e.group_id )
        LEFT JOIN users AS u ON ( u.id = e.creator_id )
        WHERE e.id = ?
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
                  group_id,
                  group_name,
                  creator_id,
                  creator_name )
                = from_row::<(EventId,
                              u64,
                              u64,
                              String,
                              String,
                              EventState,
                              bool,
                              Id,
                              Option<String>,
                              Id,
                              Option<String>)>( row );

            Some( ScheduledEventInfo {
                id: id,
                start_time: start_time.into_timespec(),
                end_time: end_time.into_timespec(),
                scheduled_id: scheduled_id,
                name: name,
                data: data,
                state: state,
                group: match group_attached {
                    true => Some( ShortInfo{
                        id: group_id,
                        name: group_name.unwrap()
                    }),
                    false => None
                },
                creator: match creator_id {
                    0 => None,
                    _ => Some( ShortInfo {
                        id: creator_id,
                        name: creator_name.unwrap(),
                    })
                }
            })
        },
        None => None
    };
    Ok( result )
}

fn event_infos_impl( conn: &mut PooledConn, scheduled_ids: &[Id] ) -> mysql::Result<EventInfos> {
    // проверка на пустату
    if scheduled_ids.is_empty() {
        return Ok( Vec::new() );
    }

    let mut where_cond = String::from( "e.id in ( ?" );
    for _ in 1..scheduled_ids.len() {
        where_cond.push_str( ", ?" );
    }
    where_cond.push_str( ")" );
    let values: Vec<Value> = scheduled_ids
        .iter()
        .map( |id| id.to_value() )
        .collect();
    get_events_impl( conn, &where_cond, &values )
}

fn event_start_time_impl( conn: &mut PooledConn, scheduled_id: Id ) -> mysql::Result<Option<Timespec>> {
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

fn add_events_impl( conn: &mut PooledConn, events: &[FullEventInfo] ) -> mysql::Result<()> {
    let mut query = format!(
        "INSERT INTO scheduled_events (
            event_id,
            event_name,
            start_time,
            end_time,
            data,
            group_attached,
            group_id,
            creator_id
        )
        VALUES( ?, ?, ?, ?, ?, ?, ?, ? )"
    );

    for _ in 1 .. events.len() {
        query.push_str( ", ( ?, ?, ?, ?, ?, ?, ?, ? )" );
    }

    let mut stmt = try!( conn.prepare( &query ) );
    let mut values: Vec<Value> = Vec::new();
    for i in 0 .. events.len() {
        let event = &events[ i ];
        values.push( event.id.to_value() );
        values.push( event.name.clone().to_value() );
        values.push( event.start_time.msecs().to_value() );
        values.push( event.end_time.msecs().to_value() );
        values.push( event.data.clone().to_value() );

        let (group_attached, group_id) = to_group_info( event.group );
        values.push( group_attached.to_value() );
        values.push( group_id.to_value() );

        let creator_id = match event.creator {
            Some( creator_id ) => creator_id,
            None => 0
        };
        values.push( creator_id.to_value() );
    }

    try!( stmt.execute( values ) );
    Ok( () )
}

fn add_disabled_event_impl( conn: &mut PooledConn, event: &FullEventInfo ) -> mysql::Result<Id> {
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

fn set_event_state_impl( conn: &mut PooledConn, scheduled_id: Id, state: EventState ) -> mysql::Result<()> {
    let mut stmt = try!( conn.prepare( "UPDATE scheduled_events SET state=? WHERE id=?" ) );
    let params: &[ &ToValue ] = &[ &state, &scheduled_id ];
    try!( stmt.execute( params ) );
    Ok( () )
}

/// "выключает" событие если оно еще не началось
fn disable_event_if_not_started_impl( conn: &mut PooledConn, scheduled_id: Id ) -> mysql::Result<()> {
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


fn increment_event_comments_count_impl( conn: &mut PooledConn, scheduled_id: Id ) -> mysql::Result<()> {
    try!( conn.prep_exec( "UPDATE scheduled_events SET comments_count=comments_count+1 WHERE id=?",
                           (scheduled_id,) ) );
    Ok( () )
}

fn get_next_event_after_impl(
    conn: &mut PooledConn,
    moment: &Timespec,
    event_id: EventId
) -> mysql::Result<Option<(Id, Timespec)>>
{
    let mut stmt = try!( conn.prepare( "
        SELECT
            `id`,
            `start_time`
        FROM `scheduled_events`
        WHERE `start_time` > ?
          AND `event_id` = ?
          AND `state` NOT IN ('disabled', 'finished')
    " ) );
    let mut sql_result = try!( stmt.execute( (&moment.msecs(), &event_id) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            let (id, start_time) : (Id, u64) = from_row( row );
            Some( (id, start_time.into_timespec()) )
        },
        None => None
    };
    Ok( result )
}

fn update_event_start_time_impl(
    conn: &mut PooledConn,
    scheduled_id: Id,
    new_start: &Timespec
) -> mysql::Result<()>
{
    let mut stmt = try!( conn.prepare( "
        UPDATE `scheduled_events`
        SET `start_time`=?
        WHERE `id`=?
          AND `state`='not_started_yet'
    " ) );
    try!( stmt.execute( (&new_start.msecs(), &scheduled_id) ) );
    Ok( () )
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
    fn new(v: Value) -> mysql::Result<EventIdIr> {
        match v {
            Value::Int( num ) => {
                let maybe_event_id: MaybeEventId = From::from( num );
                let MaybeEventId( maybe_id ) = maybe_event_id;
                match maybe_id {
                    Some( id ) => Ok( EventIdIr {
                        val: id,
                        raw: num
                    }),
                    None => Err( mysql::Error::FromValueError( Value::Int( num ) ) )
                }
            }
            _ => Err( mysql::Error::FromValueError( v ) )
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
