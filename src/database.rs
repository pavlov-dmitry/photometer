/// модуль работы с БД

use mysql::conn::{MyOpts};
use mysql::conn::pool::{MyPool, MyPooledConn};
use mysql::error::{ MyResult };
//use mysql::value::{from_value};
use std::default::{Default};

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };

#[deriving(Clone)]
pub struct Database {
    pool: MyPool
}

pub struct DatabaseConn {
    connection: MyResult<MyPooledConn>
}

impl DatabaseConn {
    fn get_conn(&mut self) -> Result<&mut MyPooledConn, String> {
        self.connection.as_mut().map_err( |e| format!( "Database:: creating connection failed: {}", e ) )
    }
    pub fn has_user( &mut self, name: &str ) -> Result<bool, String> {
        /*self.get_conn().and_then( |connection|
            connection.prepare( "select login from users where login=?" )
                .and_then( |ref mut stmt| 
                    stmt.execute(&[&name])
                        .and_then( |ref mut sql_result| Ok( sql_result.count() == 1 ) )
                )
                .map_err( |e| format!( "Database:: func 'has_user' failed: {}", &e ) ),
        )*/
        Ok( false ) // пока так, т.к. internal compiler error =(
    }
}

impl Database {
    fn init(&self) -> Result<(), String> {
        let result = self.pool
            .query( "set names utf8;" );
        match result {
            Ok(_)=>Ok( () ),
            Err( e ) => Err( format!( "Database::init failed: {}", e ) )
        }
    }
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
    fn db(&self) -> DatabaseConn;
}

impl<'a, 'b> Databaseable for Request<'a, 'b> {
    fn db(&self) -> DatabaseConn {
        DatabaseConn{ connection: self.map.find::<Database>().unwrap().pool.get_conn() }
    }
}