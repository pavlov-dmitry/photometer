use std::str;
use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult, MyError };
use mysql::value::{
    IntoValue,
    Value,
    ToValue,
    FromValue,
    ConvIr
};
use database::Database;
use std::fmt::Display;
use types::{
    Id,
    EmptyResult,
    CommonError,
};

pub enum VisitedContent {
    Feed
}

pub trait DbVisited {
    /// выставляет что данный контент уже посёщен
    fn set_visited( &mut self, user_id: Id, content: VisitedContent, items: &[ Id ] ) -> EmptyResult;
    // /// проверяет видел ли данный контент пользователь
    // fn is_visited( &mut self, user_id: Id, content: VisitedContent, item: Id ) -> CommonResult<bool>;
    // /// проверяет видел ли список контента пользователь
    // fn check_visited( &mut self, user_id: Id, content: VisitedContent, items: &[ Id ] ) -> CommonResult<Vec<(Id, bool)>>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `visited` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `user_id` bigint(20) NOT NULL DEFAULT '0',
            `content_type` ENUM(
                'feed'
            ) NOT NULL DEFAULT 'feed',
            `content_id` bigint(20) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::visited::create_tables"
    )
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbVisited func '{}' failed: {}", fn_name, e ) )
}

impl DbVisited for MyPooledConn {
    /// выставляет что данный контент уже посёщен
    fn set_visited( &mut self, user_id: Id, content: VisitedContent, items: &[ Id ] ) -> EmptyResult {
        set_visited_impl( self, user_id, content, items )
            .map_err( |e| fn_failed( "set_visited", e ) )
    }
    // /// проверяет видел ли данный контент пользователь
    // fn is_visited( &mut self, user_id: Id, content: VisitedContent, item: Id ) -> CommonResult<bool> {
    //     is_visited_impl( self, user_id, content, item )
    //         .map_err( |e| fn_failed( "is_visited_impl", e) )
    // }
    // /// проверяет видел ли список контента пользователь
    // fn check_visited( &mut self, user_id: Id, content: VisitedContent, items: &[ Id ] ) -> CommonResult<Vec<(Id, bool)>> {
    //     check_visited_impl( self, user_id, content, items )
    //         .map_err( |e| fn_failed( "check_visited", e ) )
    // }
}

fn set_visited_impl( conn: &mut MyPooledConn,
                     user_id: Id,
                     content: VisitedContent,
                     items: &[ Id ] ) -> MyResult<()>
{
    if items.is_empty() {
        return Ok( () )
    }

    let mut query = format!(
        "INSERT INTO visited (
            user_id,
            content_type,
            content_id
        )
        VALUES( ?, ?, ? )"
    );
    for _ in 1 .. items.len() {
        query.push_str( ", (?, ?, ?)");
    }

    let mut stmt = try!( conn.prepare( &query ) );
    let mut values: Vec<Value> = Vec::with_capacity( items.len() * 3 );
    for id in items {
        values.push( user_id.into_value() );
        values.push( content.into_value() );
        values.push( id.into_value() );
    }
    try!( stmt.execute( values ) );
    Ok( () )
}

const FEED_STR: &'static str = "feed";

impl ToValue for VisitedContent {
    fn to_value(&self) -> Value {
        match self {
            &VisitedContent::Feed => Value::Bytes( FEED_STR.bytes().collect() )
        }
    }
}

pub struct VisitedContentIr
{
    val: VisitedContent,
    bytes: Vec<u8>
}

impl ConvIr<VisitedContent> for VisitedContentIr {
    fn new(v: Value) -> MyResult<VisitedContentIr> {
        match v {
            Value::Bytes( bytes ) => {
                let value = match str::from_utf8( &bytes ) {
                    Ok( s ) => match s {
                        FEED_STR => Some( VisitedContent::Feed ),
                        _ => None
                    },
                    _ => None
                };
                match value {
                    Some( t ) => Ok( VisitedContentIr{ val: t, bytes: bytes } ),
                    None => Err( MyError::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err(MyError::FromValueError(v))
        }
    }
    fn commit(self) -> VisitedContent {
        self.val
    }
    fn rollback(self) -> Value {
        Value::Bytes( self.bytes )
    }
}

impl FromValue for VisitedContent {
    type Intermediate = VisitedContentIr;
}
