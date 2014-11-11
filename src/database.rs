/// модуль работы с БД

use mysql::conn::{ MyOpts };
use mysql::conn::pool::{ MyPool, MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value };
use std::default::{ Default };

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };

#[deriving(Clone)]
pub struct Database {
    pool: MyPool
}

pub struct DatabaseConn {
    connection: MyResult<MyPooledConn>
}

pub type Id = i64;
pub type DBResult<T> = Result<T, String>;

impl DatabaseConn {
    fn get_conn(&mut self) -> DBResult<&mut MyPooledConn> {
        self.connection.as_mut().map_err( |e| format!( "Database:: creating connection failed: {}", e ) )
    }
    /// выбирает id пользователя по имени и паролю
    pub fn get_user( &mut self, name: &str, pass: &str ) -> DBResult<Option<Id>> {
        let name = name.to_string(); // помогает убрать internal compiler error
        let pass = pass.to_string();
        self.get_conn()
            .and_then( |connection| connection.prepare( "select id from users where login=? and password=?" )
                .and_then( |ref mut stmt| stmt.execute( &[ &name, &pass ] )
                    .and_then( |ref mut sql_result| 
                        sql_result.next().map_or( Ok( None ),
                            |row| row.and_then( |r| Ok( Some( from_value::<Id>( &r[0] ) ) ) )
                        )
                    )
                )
                .map_err( |e| format!( "Database:: func 'get_user' failed: {}", &e ) )
            )
    }
    /// добавляет нового пользователя в БД
    pub fn add_user( &mut self, name: &str, pass: &str ) -> DBResult<()> {
        let name = name.to_string();
        let pass = pass.to_string();
        self.get_conn()
            .and_then( |connection| connection.prepare( "INSERT INTO users (login, password) VALUES(?, ?);" )
                .and_then( |ref mut stmt| stmt.execute( &[ &name, &pass ] ).and( Ok( () ) ) )
                .map_err( |e| format!( "Database:: func 'add_user' failed: {}", e ) )
            )
    }
    /// проверяет наличие имени в БД
    pub fn user_exists(&mut self, name: &str,) -> DBResult<bool> {
        let name = name.to_string();
        self.get_conn()
            .and_then( |connection| connection.prepare( "select id from users where login=?" )
                .and_then( |ref mut stmt| stmt.execute( &[ &name ] )
                    .and_then( |ref mut sql_result|
                        Ok( sql_result.count() == 1 )
                    )
                )
                .map_err( |e| format!( "Database:: func 'user_exists' failed: {}", &e ) )
            )
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

pub fn create_db_connection( 
    db_name: String, 
    user: String, 
    pass: String,
    min_connections: uint,
    max_connections: uint
) -> Result<Database, String> {
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
    fn db(&self) -> DatabaseConn;
}

impl<'a, 'b> Databaseable for Request<'a, 'b> {
    fn db(&self) -> DatabaseConn {
        DatabaseConn{ connection: self.map.get::<Database>().unwrap().pool.get_conn() }
    }
}