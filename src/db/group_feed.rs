use std::str;
use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult, MyError };
use mysql::value::{ from_value, ToValue, FromValue, Value, ConvIr };
use database::Database;
use std::fmt::Display;
use types::{
    Id,
    EmptyResult,
    CommonResult,
    CommonError,
    ShortInfo
};
use events::feed_types::{
    FeedEventState,
    FeedEventInfo
};

pub trait DbGroupFeed {
    /// Добавить новое событие в ленту группы
    fn add_to_group_feed( &mut self,
                          creation_time: u64,
                          group_id: Id,
                          scheduled_id: Id,
                          state: FeedEventState,
                          data: &str ) -> EmptyResult;
    /// Получить события в ленте
    fn get_group_feed( &mut self, group_id: Id, count: u32, offset: u32 ) -> CommonResult<Vec<FeedEventInfo>>;
    /// получить событие по идентификатору
    fn get_feed_info( &mut self, id: Id ) -> CommonResult<Option<FeedEventInfo>>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `group_feed` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `creation_time` bigint(20) NOT NULL DEFAULT '0',
            `scheduled_id` bigint(20) NOT NULL DEFAULT '0',
            `group_id` bigint(20) NOT NULL DEFAULT '0',
            `state` ENUM(
                'start',
                'finish'
            ) NOT NULL DEFAULT 'start',
            `data` TEXT NOT NULL DEFAULT '',
            PRIMARY KEY ( `id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::group_feed::create_tables"
    )
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbGroupFeed func '{}' failed: {}", fn_name, e ) )
}

impl DbGroupFeed for MyPooledConn {
    /// Добавить новое событие в ленту группы
    fn add_to_group_feed( &mut self,
                          creation_time: u64,
                          group_id: Id,
                          scheduled_id: Id,
                          state: FeedEventState,
                          data: &str ) -> EmptyResult
    {
        add_to_group_feed_impl( self, creation_time, group_id, scheduled_id, state, data )
            .map_err( |e| fn_failed( "add_to_group_feed", e ) )
    }

    /// Получить события в ленте
    fn get_group_feed( &mut self,
                       group_id: Id,
                       count: u32,
                       offset: u32 ) -> CommonResult<Vec<FeedEventInfo>>
    {
        get_group_feed_impl( self, group_id, count, offset )
            .map_err( |e| fn_failed( "get_group_feed", e ) )
    }

    /// получить событие по идентификатору
    fn get_feed_info( &mut self, id: Id ) -> CommonResult<Option<FeedEventInfo>> {
        get_feed_info_impl( self, id )
            .map_err( |e| fn_failed( "get_feed_info", e ) )
    }
}

fn add_to_group_feed_impl( conn: &mut MyPooledConn,
                           creation_time: u64,
                           group_id: Id,
                           scheduled_id: Id,
                           state: FeedEventState,
                           data: &str ) -> MyResult<()>
{
    let mut stmt = try!( conn.prepare(
        "INSERT INTO `group_feed` (
            `creation_time`,
            `scheduled_id`,
            `group_id`,
            `state`,
            `data`
        )
        VALUES( ?, ?, ?, ?, ? )"
    ));
    let params: &[ &ToValue ] = &[ &creation_time, &scheduled_id, &group_id, &state, &data ];
    try!( stmt.execute( params ) );
    Ok( () )
}

const FIELDS: &'static str = "
             f.id,
             f.creation_time,
             f.state,
             f.data,
             e.id,
             e.event_id,
             e.start_time,
             e.end_time,
             e.event_name,
             g.id,
             g.name,
             e.creator_id,
             u.login";

fn read_fields<I: Iterator<Item = Value>>( mut values: I ) -> FeedEventInfo {
    let id = from_value( values.next().unwrap() );
    let creation_time = from_value( values.next().unwrap() );
    let state = from_value( values.next().unwrap() );
    let data = from_value( values.next().unwrap() );
    let scheduled_id = from_value( values.next().unwrap() );
    let event_id = from_value( values.next().unwrap() );
    let start_time = from_value( values.next().unwrap() );
    let end_time = from_value( values.next().unwrap() );
    let event_name = from_value( values.next().unwrap() );
    let group = ShortInfo{
        id: from_value( values.next().unwrap() ),
        name: from_value( values.next().unwrap() )
    };
    let user_id = from_value( values.next().unwrap() );
    let user = match user_id {
        0 => {
            // дочитываем значение
            let _ = from_value::<Option<String>>( values.next().unwrap() );
            None
        },
        _ => Some( ShortInfo {
            id: user_id,
            name: from_value( values.next().unwrap() )
        })
    };

    FeedEventInfo {
        id: id,
        creation_time: creation_time,
        state: state,
        data: data,
        scheduled_id: scheduled_id,
        event_id: event_id,
        start_time: start_time,
        end_time: end_time,
        event_name: event_name,
        group: group,
        creator: user
    }
}

fn get_feed_info_impl( conn: &mut MyPooledConn, id: Id ) -> MyResult<Option<FeedEventInfo>> {
    let query = format!(
        "SELECT {}
         FROM `group_feed` as f
         LEFT JOIN `scheduled_events` as e ON ( e.id = f.scheduled_id )
         LEFT JOIN `groups` as g ON ( g.id = f.group_id )
         LEFT JOIN `users` as u ON ( u.id = e.creator_id )
         WHERE f.id = ?",
        FIELDS
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let mut sql_result = try!( stmt.execute( (id,) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            Some( read_fields( row.into_iter() ) )
        },
        None => None
    };
    Ok( result )
}

fn get_group_feed_impl( conn: &mut MyPooledConn,
                        group_id: Id,
                        count: u32,
                        offset: u32 ) -> MyResult<Vec<FeedEventInfo>>
{
    let query = format!(
        "SELECT {}
         FROM `group_feed` as f
         LEFT JOIN `scheduled_events` as e ON ( e.id = f.scheduled_id )
         LEFT JOIN `groups` as g ON ( g.id = f.group_id )
         LEFT JOIN `users` as u ON ( u.id = e.creator_id )
         WHERE f.group_id=?
         ORDER BY f.creation_time DESC
         LIMIT ?
         OFFSET ?;",
        FIELDS
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let params: &[ &ToValue ] = &[ &group_id, &count, &offset ];
    let sql_result = try!( stmt.execute( params ) );

    let (low_count, _) = sql_result.size_hint();
    let mut feed: Vec<_> = Vec::with_capacity( low_count );
    for row in sql_result {
        let row = try!( row );
        let values = row.into_iter();
        feed.push( read_fields( values ) );
    }
    Ok( feed )
}

const START: &'static str = "start";
const FINISH: &'static str = "finish";

impl ToValue for FeedEventState {
    fn to_value(&self) -> Value {
        let bytes = match self {
            &FeedEventState::Start => START.bytes().collect(),
            &FeedEventState::Finish => FINISH.bytes().collect()
        };
        Value::Bytes( bytes )
    }
}

pub struct FeedEventStateIr {
    val: FeedEventState,
    bytes: Vec<u8>
}

impl ConvIr<FeedEventState> for FeedEventStateIr {
    fn new(v: Value) -> MyResult<FeedEventStateIr> {
        match v {
            Value::Bytes( bytes ) => {
                let val = match str::from_utf8( &bytes ) {
                    Ok( s ) => match s {
                        START => Some( FeedEventState::Start ),
                        FINISH => Some( FeedEventState::Finish ),
                        _ => None
                    },
                    _ => None
                };
                match val {
                    Some( val ) => Ok( FeedEventStateIr{ val: val, bytes: bytes }),
                    None => Err( MyError::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err( MyError::FromValueError( v ) )
        }
    }
    fn commit(self) -> FeedEventState {
        self.val
    }
    fn rollback(self) -> Value {
        Value::Bytes( self.bytes )
    }
}

impl FromValue for FeedEventState {
    type Intermediate = FeedEventStateIr;
}
