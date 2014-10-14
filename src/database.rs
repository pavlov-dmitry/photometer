/// модуль работы с БД

use mysql::conn::{MyOpts};
use mysql::conn::pool::{MyPool};
//use mysql::value::{from_value};
use std::default::{Default};

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };

#[deriving(Clone)]
pub struct Database {
    pool: MyPool
}

pub fn create_db_connection( db_name: String, user: String, pass: String ) -> Result<Database, String> {
    let opts = MyOpts{
        db_name: Some( db_name ),
        user: Some( user ), 
        pass: Some( pass ),
        ..Default::default()
    };

    let pool = MyPool::new( opts );
    match pool {
        Ok( pool ) => Ok( Database{ pool: pool } ),
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
    fn db(&self) -> &Database;
}

impl<'a, 'b> Databaseable for Request<'a, 'b> {
    fn db(&self) -> &Database {
        self.map.find::<Database>().unwrap()
    }
}