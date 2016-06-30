/// модуль работы с БД
use mysql;
use mysql::conn::pool::{ Pool, PooledConn };

use types::{ CommonResult, EmptyResult, CommonError, common_error };
use iron::typemap::Key;
use iron::prelude::*;
use db;
use err_msg;
use stuff::{ Stuff, StuffInstallable };

pub trait Databaseable {
    //создаёт новое подключение
    fn get_new_db_conn(&self) -> CommonResult<PooledConn>;
    //реализует ленивую отдачу текущего подключения
    fn get_current_db_conn(&mut self) -> CommonResult<&mut PooledConn>;
}

#[derive(Clone)]
pub struct Database {
    pool: Pool
}

pub type DbConnection = PooledConn;

impl Database {
    fn init(&self) -> EmptyResult {
        try!( db::users::create_tables( self ) );
        try!( db::photos::create_tables( self ) );
        try!( db::mailbox::create_tables( self ) );
        try!( db::groups::create_tables( self ) );
        try!( db::events::create_tables( self ) );
        try!( db::publication::create_tables( self ) );
        try!( db::votes::create_tables( self ) );
        try!( db::group_feed::create_tables( self ) );
        try!( db::visited::create_tables( self ) );
        try!( db::comments::create_tables( self ) );
        try!( self.init_names() );
        Ok( () )
    }

    fn init_names(&self) -> EmptyResult {
        self.execute( "set names utf8;", "init_names" )
    }

    pub fn execute( &self, query: &str , fn_name: &str ) -> EmptyResult {
        match self.pool.prep_exec( query, () ) {
            Ok( _ ) => Ok( () ),
            Err( e ) => Err( err_msg::fn_failed( fn_name, e ) )
        }
    }

}

pub fn create_db_connection(
    db_name: String,
    user: String,
    pass: String,
    min_connections: usize,
    max_connections: usize
) -> CommonResult<Database> {
    let mut opts = mysql::OptsBuilder::new();
    opts.db_name( Some( db_name ) )
        .user( Some( user ) )
        .pass( Some( pass ) );

    let pool = Pool::new_manual( min_connections, max_connections, opts );
    match pool {
        Ok( pool ) => {
            let db = Database{ pool: pool };
            match db.init() { // тут я что-то подупрел с fn map, скопилировалось только с match
                Ok(_) => Ok( db ),
                Err( e ) => Err( e )
            }
        },
        Err( e ) => common_error( format!( "Connection to db failed: {}", e ) )
    }
}

impl Key for Database { type Value = Database; }

impl StuffInstallable for Database {
    fn install_to(&self, stuff: &mut Stuff ) {
        stuff.extensions.insert::<Database>( self.clone() );
    }
}

struct ConnectionKey;
impl Key for ConnectionKey { type Value = PooledConn; }

impl Databaseable for Stuff {
    fn get_new_db_conn(&self) -> CommonResult<PooledConn> {
        self.extensions.get::<Database>().unwrap()
            .pool.get_conn()
            .map_err( |e| CommonError( format!( "Can't create db connection: {}", e ) ) )
    }
    fn get_current_db_conn(&mut self) -> CommonResult<&mut PooledConn> {
        if self.extensions.contains::<ConnectionKey>() == false {
            let conn = try!( self.get_new_db_conn() );
            self.extensions.insert::<ConnectionKey>( conn );
        }
        Ok( self.extensions.get_mut::<ConnectionKey>().unwrap() )
    }
}
