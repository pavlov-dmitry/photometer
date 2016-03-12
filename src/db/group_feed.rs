use std::str;
use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{ from_value, from_row, ToValue, FromValue, Value, ConvIr };
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

pub type UnwatchedFeedsByGroup = Vec<(Id, u32)>;

pub trait DbGroupFeed {
    /// Добавить новое событие в ленту группы
    fn add_to_group_feed( &mut self,
                          creation_time: u64,
                          group_id: Id,
                          scheduled_id: Id,
                          state: FeedEventState,
                          data: &str ) -> EmptyResult;
    /// Получить события в ленте
    fn get_group_feed( &mut self, user_id: Id, group_id: Id, count: u32, offset: u32 ) -> CommonResult<Vec<FeedEventInfo>>;
    /// получить событие по идентификатору
    fn get_feed_info( &mut self, user_id: Id, id: Id ) -> CommonResult<Option<FeedEventInfo>>;
    /// получить кол-во не просмотренных сообщений для пользователя
    fn get_unwatched_feed_elements_by_groups(
        &mut self,
        user_id: Id
    ) -> CommonResult<UnwatchedFeedsByGroup>;
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
            PRIMARY KEY ( `id` ),
            KEY `group_idx` ( `group_id` ),
            KEY `sheduled_idx` ( `scheduled_id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::group_feed::create_tables"
    )
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbGroupFeed func '{}' failed: {}", fn_name, e ) )
}

impl DbGroupFeed for PooledConn {
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
                       user_id: Id,
                       group_id: Id,
                       count: u32,
                       offset: u32 ) -> CommonResult<Vec<FeedEventInfo>>
    {
        get_group_feed_impl( self, user_id, group_id, count, offset )
            .map_err( |e| fn_failed( "get_group_feed", e ) )
    }

    /// получить событие по идентификатору
    fn get_feed_info( &mut self, user_id: Id, id: Id ) -> CommonResult<Option<FeedEventInfo>> {
        get_feed_info_impl( self, user_id, id )
            .map_err( |e| fn_failed( "get_feed_info", e ) )
    }

    /// получить кол-во не просмотренных сообщений для пользователя
    fn get_unwatched_feed_elements_by_groups(
        &mut self,
        user_id: Id
    ) -> CommonResult<UnwatchedFeedsByGroup>
    {
        get_unwatched_feed_elements_by_groups_impl( self, user_id )
            .map_err( |e| fn_failed( "get_unwatched_feed_elements_by_groups", e ) )
    }
}

fn add_to_group_feed_impl( conn: &mut PooledConn,
                           creation_time: u64,
                           group_id: Id,
                           scheduled_id: Id,
                           state: FeedEventState,
                           data: &str ) -> mysql::Result<()>
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
    u.login,
    v.id,
    e.comments_count,
    ( SELECT COUNT( cc.id )
      FROM comments AS cc
      LEFT JOIN visited AS vv
           ON ( vv.content_type='comment'
                AND vv.content_id = cc.id
                AND vv.user_id=? )
      WHERE cc.comment_for='event' AND cc.for_id=f.scheduled_id AND vv.id is NULL )";

fn read_fields<I: Iterator<Item = Value>>( mut values: I ) -> FeedEventInfo {
    let mut next = || values.next().expect( "DbGroupFeed invalid columns count on read FeedInfo");

    let id = from_value( next() );
    let creation_time = from_value( next() );
    let state = from_value( next() );
    let data = from_value( next() );
    let scheduled_id = from_value( next() );
    let event_id = from_value( next() );
    let start_time = from_value( next() );
    let end_time = from_value( next() );
    let event_name = from_value( next() );
    let group = ShortInfo{
        id: from_value( next() ),
        name: from_value( next() )
    };
    let user_id = from_value( next() );
    let user = match user_id {
        0 => {
            // дочитываем значение
            let _ = from_value::<Option<String>>( next() );
            None
        },
        _ => Some( ShortInfo {
            id: user_id,
            name: from_value( next() )
        })
    };
    let visited_id: Option<Id> = from_value( next() );
    let is_new = match visited_id {
        Some( _ ) => false,
        None => true
    };
    let comments_count = from_value( next() );
    let unreaded_comments = from_value( next() );

    FeedEventInfo {
        id: id,
        is_new: is_new,
        creation_time: creation_time,
        state: state,
        data: data,
        scheduled_id: scheduled_id,
        event_id: event_id,
        start_time: start_time,
        end_time: end_time,
        event_name: event_name,
        group: group,
        creator: user,
        comments_count: comments_count,
        unreaded_comments: unreaded_comments
    }
}

fn get_feed_info_impl( conn: &mut PooledConn, user_id: Id, id: Id ) -> mysql::Result<Option<FeedEventInfo>> {
    let query = format!(
        "SELECT {}
         FROM `group_feed` as f
         LEFT JOIN `scheduled_events` as e ON ( e.id = f.scheduled_id )
         LEFT JOIN `groups` as g ON ( g.id = f.group_id )
         LEFT JOIN `users` as u ON ( u.id = e.creator_id )
         LEFT JOIN `visited` as v ON ( v.user_id=? AND v.content_type='feed' AND v.content_id=f.id )
         WHERE f.id = ?",
        FIELDS
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let mut sql_result = try!( stmt.execute( (user_id, user_id, id) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            Some( read_fields( row.unwrap().into_iter() ) )
        },
        None => None
    };
    Ok( result )
}

fn get_group_feed_impl( conn: &mut PooledConn,
                        user_id: Id,
                        group_id: Id,
                        count: u32,
                        offset: u32 ) -> mysql::Result<Vec<FeedEventInfo>>
{
    let query = format!(
        "SELECT {}
         FROM `group_feed` as f
         LEFT JOIN `scheduled_events` as e ON ( e.id = f.scheduled_id )
         LEFT JOIN `groups` as g ON ( g.id = f.group_id )
         LEFT JOIN `users` as u ON ( u.id = e.creator_id )
         LEFT JOIN `visited` as v ON ( v.user_id=? AND v.content_type='feed' AND v.content_id=f.id )
         WHERE f.group_id=?
         ORDER BY f.creation_time DESC
         LIMIT ?
         OFFSET ?;",
        FIELDS
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let params: &[ &ToValue ] = &[ &user_id, &user_id, &group_id, &count, &offset ];
    let sql_result = try!( stmt.execute( params ) );

    let (low_count, _) = sql_result.size_hint();
    let mut feed: Vec<_> = Vec::with_capacity( low_count );
    for row in sql_result {
        let row = try!( row );
        let values = row.unwrap().into_iter();
        feed.push( read_fields( values ) );
    }
    Ok( feed )
}

fn get_unwatched_feed_elements_by_groups_impl( conn: &mut PooledConn,
                                               user_id: Id ) -> mysql::Result<UnwatchedFeedsByGroup>
{
    let query = "
        SELECT f.group_id, COUNT( f.id )
        FROM group_feed AS f
        LEFT JOIN group_members AS gm ON( gm.group_id = f.group_id )
        LEFT JOIN visited AS v ON ( v.content_type='feed' AND v.content_id=f.id AND v.user_id=gm.user_id )
        WHERE gm.user_id=? AND v.id is NULL
        GROUP BY f.group_id
    ";
    let mut stmt = try!( conn.prepare( query ) );
    let sql_result = try!( stmt.execute( (user_id,) ) );
    let mut result = Vec::new();
    for row in sql_result {
        let row = try!( row );
        result.push( from_row( row ) );
    }
    Ok( result )
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
    fn new(v: Value) -> mysql::Result<FeedEventStateIr> {
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
                    None => Err( mysql::Error::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err( mysql::Error::FromValueError( v ) )
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
