/// модуль работы с БД
use mysql::conn::{ MyOpts };
use mysql::conn::pool::{ MyPool, MyPooledConn };
use std::default::{ Default };

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use types::{ CommonResult };

#[deriving(Clone)]
pub struct Database {
    pool: MyPool
}

pub type DbConnection = MyPooledConn;

impl Database {
    fn init(&self) -> CommonResult<()> {
        let result = self.pool
            .query( "set names utf8;" );
        match result {
            Ok(_)=>Ok( () ),
            Err( e ) => Err( format!( "Database::init failed: {}", e ) )
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

impl Middleware for Database {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.map.insert( self.clone() );
        Ok( Continue )
    }
}

pub trait Databaseable {
    fn get_db_conn(&self) -> CommonResult<MyPooledConn>;
}

impl<'a, 'b> Databaseable for Request<'a, 'b> {
    fn get_db_conn(&self) -> CommonResult<MyPooledConn> {
        self.map.get::<Database>().unwrap()
            .pool.get_conn()
            .map_err( |e| format!( "Can't create db connection: {}", e ) )
    }
}