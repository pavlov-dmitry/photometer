/// модуль работы с БД
use mysql::conn::{ MyOpts };
use mysql::conn::pool::{ MyPool, MyPooledConn };
use std::default::{ Default };

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use types::{ CommonResult, EmptyResult };
use typemap::Assoc;
use plugin::Extensible;
use db;
use err_msg;

pub trait Databaseable {
    fn get_db_conn(&self) -> CommonResult<MyPooledConn>;
}

#[deriving(Clone)]
pub struct Database {
    pool: MyPool
}

pub type DbConnection = MyPooledConn;

impl Database {
    fn init(&self) -> EmptyResult {
        try!( db::users::create_tables( self ) );
        try!( db::photos::create_tables( self ) );
        try!( db::mailbox::create_tables( self ) );
        try!( db::groups::create_tables( self ) );
        try!( db::events::create_tables( self ) );
        try!( db::publication::create_tables( self ) );
        try!( db::timetable::create_tables( self ) );
        try!( db::votes::create_tables( self ) );
        try!( self.init_names() );
        Ok( () )
    }

    fn init_names(&self) -> EmptyResult {
        self.execute( "set names utf8;", "init_names" )
    }

    pub fn execute( &self, query: &str , fn_name: &str ) -> EmptyResult {
        match self.pool.query( query ) {
            Ok(_) => Ok( () ),
            Err( e ) => Err( err_msg::fn_failed( fn_name, e ) )
        }
    }

}

pub fn create_db_connection( 
    db_name: String, 
    user: String, 
    pass: String,
    min_connections: uint,
    max_connections: uint
) -> CommonResult<Database> {
    let opts = MyOpts{
        db_name: Some( db_name ),
        user: Some( user ), 
        pass: Some( pass ),
        ..Default::default()
    };

    let pool = MyPool::new_manual( min_connections, max_connections, opts );
    match pool {
        Ok( pool ) => {
            let db = Database{ pool: pool };
            match db.init() { // тут я что-то подупрел с fn map, скопилировалось только с match
                Ok(_) => Ok( db ),
                Err( e ) => Err( e )
            }
        },
        Err( e ) => Err( format!( "Connection to db failed: {}", e ) )
    }
}

impl Assoc<Database> for Database {}

impl Middleware for Database {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.extensions_mut().insert::<Database, Database>( self.clone() );
        Ok( Continue )
    }
}

impl<'a, 'b> Databaseable for Request<'a, 'b> {
    fn get_db_conn(&self) -> CommonResult<MyPooledConn> {
        self.extensions().get::<Database, Database>().unwrap()
            .pool.get_conn()
            .map_err( |e| format!( "Can't create db connection: {}", e ) )
    }
}