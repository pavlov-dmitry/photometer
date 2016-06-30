use std::str;
use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{
    Value,
    FromValue,
    ConvIr,
    from_value,
    from_row
};
use database::Database;
use std::fmt::Display;
use types::{
    Id,
    EmptyResult,
    CommonResult,
    CommonError,
    CommentInfo,
    ShortInfo,
    CommentFor
};

pub trait DbComments {
    /// Добавляет комментарий
    fn add_comment( &mut self,
                     user_id: Id,
                     time: u64,
                     comment_for: CommentFor,
                     for_id: Id,
                     text: &str ) -> EmptyResult;
    /// редактирование комментария
    fn edit_comment( &mut self, comment_id: Id, time: u64, text: &str ) -> EmptyResult;
    /// чтение информации о комментарии
    fn get_comment_info( &mut self,
                          reader_id: Id,
                          comment_id: Id ) -> CommonResult<Option<CommentInfo>>;
    /// Чтение комментариев
    fn get_comments( &mut self,
                      reader_id: Id,
                      comments_for: CommentFor,
                      for_id: Id,
                      count: u32,
                      offset: u32 ) -> CommonResult<Vec<CommentInfo>>;
    /// чтение кол-ва комментариев
    fn get_comments_count( &mut self,
                            comments_for: CommentFor,
                            for_id: Id ) -> CommonResult<Option<u32>>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `comments` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `time` bigint(20) NOT NULL DEFAULT '0',
            `edit_time` bigint(20) NOT NULL DEFAULT '0',
            `user_id` bigint(20) NOT NULL DEFAULT '0',
            `comment_for` ENUM(
                'photo',
                'event'
            ) NOT NULL DEFAULT 'photo',
            `for_id` bigint(20) NOT NULL DEFAULT '0',
            `text` TEXT NOT NULL DEFAULT '',
            PRIMARY KEY ( `id` ),
            KEY `for_idx` ( `comment_for`, `for_id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::comments::create_tables"
    )
}

impl DbComments for PooledConn {
    /// Добавляет комментарий
    fn add_comment( &mut self,
                     user_id: Id,
                     time: u64,
                     comment_for: CommentFor,
                     for_id: Id,
                     text: &str ) -> EmptyResult
    {
        add_comment_impl( self,
                          user_id,
                          time,
                          comment_for,
                          for_id,
                          text )
            .map_err( |e| fn_failed( "add_comment", e ) )
    }
    /// редактирование комментария
    fn edit_comment( &mut self, comment_id: Id, time: u64, text: &str ) -> EmptyResult {
        edit_comment_impl( self, comment_id, time, text )
            .map_err( |e| fn_failed( "edit_comment", e ) )
    }
    /// чтение информации о комментарии
    fn get_comment_info( &mut self,
                          reader_id: Id,
                          comment_id: Id ) -> CommonResult<Option<CommentInfo>>
    {
        get_comment_info_impl( self,
                               reader_id,
                               comment_id )
            .map_err( |e| fn_failed( "get_comment_info", e ) )
    }
    /// Чтение комментариев
    fn get_comments( &mut self,
                      reader_id: Id,
                      comments_for: CommentFor,
                      for_id: Id,
                      count: u32,
                      offset: u32 ) -> CommonResult<Vec<CommentInfo>>
    {
        get_comments_impl( self, reader_id, comments_for, for_id, count, offset )
            .map_err( |e| fn_failed( "get_comments", e ) )
    }
    /// чтение кол-ва комментариев
    fn get_comments_count( &mut self,
                            comments_for: CommentFor,
                            for_id: Id ) -> CommonResult<Option<u32>>
    {
        get_comments_count_impl( self,
                                 comments_for,
                                 for_id )
            .map_err( |e| fn_failed( "get_comments_count", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbComments {} failed: {}", fn_name, e ) )
}

fn add_comment_impl( conn: &mut PooledConn,
                     user_id: Id,
                     time: u64,
                     comment_for: CommentFor,
                     for_id: Id,
                     text: &str ) -> mysql::Result<()>
{
    let mut stmt = try!( conn.prepare(
        "INSERT INTO comments (
            user_id,
            time,
            comment_for,
            for_id,
            text
        )
        VALUES( ?, ?, ?, ?, ? )"
    ));
    try!( stmt.execute( (user_id, time, comment_for, for_id, text) ) );
    Ok( () )
}

fn edit_comment_impl( conn: &mut PooledConn,
                      comment_id: Id,
                      time: u64,
                      text: &str ) -> mysql::Result<()>
{
    let mut stmt = try!( conn.prepare(
        "UPDATE comments
         SET edit_time=?, text=?
         WHERE id=?
         "
    ));
    try!( stmt.execute( (time, text, comment_id) ) );
    Ok( () )
}

const FIELDS: &'static str = "c.id,
                              c.time,
                              c.edit_time,
                              c.text,
                              c.user_id,
                              u.login,
                              v.id";

fn read_comment_fields<I: Iterator<Item = Value>>( reader_id: Id, mut values: I ) -> CommentInfo {
    let mut next = || values.next().expect( "invalid columns count on read DbComments" );

    let id = from_value( next() );
    let time = from_value( next() );
    let edit_time = from_value( next() );
    let text = from_value( next() );
    let user_id = from_value( next() );
    let user_name = from_value( next() );
    let visited_id: Option<Id> = from_value( next() );

    CommentInfo {
        id: id,
        creation_time: time,
        edit_time: edit_time,
        creator: ShortInfo {
            id: user_id,
            name: user_name
        },
        text: text,
        is_editable: reader_id == user_id,
        is_new: if let Some(_) = visited_id { false } else { true }
    }
}

fn get_comment_info_impl( conn: &mut PooledConn,
                          reader_id: Id,
                          comment_id: Id ) -> mysql::Result<Option<CommentInfo>>
{
    let query = format!(
        "SELECT {}
         FROM comments AS c
         LEFT JOIN users as u ON ( u.id = c.user_id )
         LEFT JOIN visited as v ON ( v.content_type='comment' AND v.content_id=c.id AND v.user_id=? )
         WHERE c.id = ?",
        FIELDS
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let mut sql_result = try!( stmt.execute( (reader_id, comment_id) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            let values = row.unwrap().into_iter();
            Some( read_comment_fields( reader_id, values ) )
        },
        None => None
    };
    Ok( result )
}

fn get_comments_impl( conn: &mut PooledConn,
                      reader_id: Id,
                      comments_for: CommentFor,
                      for_id: Id,
                      count: u32,
                      offset: u32 ) -> mysql::Result<Vec<CommentInfo>>
{
    let query = format!(
        "SELECT {}
         FROM comments AS c
         LEFT JOIN users as u ON ( u.id = c.user_id )
         LEFT JOIN visited as v ON ( v.content_type='comment' AND v.content_id=c.id AND v.user_id=? )
         WHERE comment_for=?
           AND for_id=?
         LIMIT ?
         OFFSET ?",
        FIELDS
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let sql_result = try!( stmt.execute( (reader_id, comments_for, for_id, count, offset) ) );
    let mut result = Vec::with_capacity( 10 );
    for row in sql_result {
        let row = try!( row );
        let values = row.unwrap().into_iter();
        result.push( read_comment_fields( reader_id, values ) );
    }
    Ok( result )
}

fn get_comments_count_impl( conn: &mut PooledConn,
                            comments_for: CommentFor,
                            for_id: Id ) -> mysql::Result<Option<u32>>
{
    let query = "SELECT COUNT(id)
                 FROM comments
                 WHERE comment_for=?
                   AND for_id=?";
    let mut stmt = try!( conn.prepare( query ) );
    let mut sql_result = try!( stmt.execute( (comments_for, for_id) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            let (count,) = from_row( row );
            Some( count )
        },
        None => None
    };
    Ok( result )
}

const EVENT_STR: &'static str = "event";
const PHOTO_STR: &'static str = "photo";

impl Into<Value> for CommentFor {
    fn into(self) -> Value {
        match self {
            CommentFor::Event => Value::Bytes( EVENT_STR.bytes().collect() ),
            CommentFor::Photo => Value::Bytes( PHOTO_STR.bytes().collect() )
        }
    }
}

//TODO: возможно можно подобные вещи делать макросом( уж больно много однотипных вещей )
pub struct CommentForIr {
    val: CommentFor,
    bytes: Vec<u8>
}

impl ConvIr<CommentFor> for CommentForIr {
    fn new(v: Value) -> mysql::Result<CommentForIr> {
        match v {
            Value::Bytes( bytes ) => {
                let value = match str::from_utf8( &bytes ) {
                    Ok( s ) => match s {
                        EVENT_STR => Some( CommentFor::Event ),
                        PHOTO_STR => Some( CommentFor::Photo ),
                        _ => None
                    },
                    _ => None
                };
                match value {
                    Some( t ) => Ok( CommentForIr{ val: t, bytes: bytes }),
                    None => Err( mysql::Error::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err(mysql::Error::FromValueError(v))
        }
    }
    fn commit(self) -> CommentFor {
        self.val
    }
    fn rollback(self) -> Value {
        Value::Bytes( self.bytes )
    }
}

impl FromValue for CommentFor {
    type Intermediate = CommentForIr;
}
