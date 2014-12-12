/// модуль работы с БД
use mysql::conn::{ MyOpts };
use mysql::conn::pool::{ MyPool, MyPooledConn };
use std::default::{ Default };

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use types::{ CommonResult, EmptyResult };
use typemap::Assoc;
use plugin::Extensible;

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
        try!( self.create_users_table() );
        try!( self.create_images_table() );
        try!( self.create_mailbox_table() );
        try!( self.create_group_table() );
        try!( self.create_group_members_table() );
        try!( self.create_scheduled_events_table() );
        try!( self.init_names() );
        Ok( () )
    }

    fn init_names(&self) -> EmptyResult {
        self.execute( "set names utf8;", "init_names" )
    }

    fn create_users_table(&self) -> EmptyResult {
        self.execute( "
            CREATE TABLE IF NOT EXISTS `users` (
                `id` bigint(20) NOT NULL AUTO_INCREMENT,
                `login` varchar(16) NOT NULL DEFAULT '',
                `password` varchar(32) NOT NULL DEFAULT '',
                PRIMARY KEY (`id`),
                KEY `login_idx` (`login`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
            ", 
            "create_users_table" 
        )
    }

    fn create_images_table(&self) -> EmptyResult {
        self.execute( "
            CREATE TABLE IF NOT EXISTS `images` (
                `id` bigint(20) NOT NULL AUTO_INCREMENT,
                `owner_id` int(4) unsigned DEFAULT '0',
                `upload_time` int(11) NOT NULL DEFAULT '0',
                `type` enum( 'jpg', 'png' ) NOT NULL DEFAULT 'jpg',
                `width` int(4) unsigned DEFAULT '0',
                `height` int(4) unsigned DEFAULT '0',
                `name` varchar(64) NOT NULL DEFAULT '',
                `iso` int(11) unsigned DEFAULT '0',
                `shutter_speed` int(11) DEFAULT '0',
                `aperture` decimal(8,4) NOT NULL DEFAULT '0',
                `focal_length` int(4) unsigned DEFAULT '0',
                `focal_length_35mm` int(4) unsigned DEFAULT '0',
                `camera_model` varchar(64) NOT NULL DEFAULT '',
                PRIMARY KEY ( `id` ),
                KEY `owner_image` ( `owner_id`, `upload_time` )
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
            ", 
            "create_images_table" 
        )
    }

    fn create_mailbox_table(&self) -> EmptyResult {
        self.execute( "
            CREATE TABLE IF NOT EXISTS `mailbox` (
                `id` bigint(20) NOT NULL AUTO_INCREMENT,
                `creation_time` int(11) NOT NULL DEFAULT '0',
                `recipient_id` int(4) unsigned DEFAULT '0',
                `sender_name` varchar(128) NOT NULL DEFAULT '',
                `subject` varchar(128) NOT NULL DEFAULT '',
                `body` varchar(4096) NOT NULL DEFAULT '',
                `readed` BOOL NOT NULL DEFAULT false,
                PRIMARY KEY ( `id` ),
                KEY `unreaded_messages` ( `recipient_id`, `readed`, `creation_time` ) USING BTREE
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
            ", 
            "create_mailbox_table" 
        )
    }

    fn create_group_table(&self) -> EmptyResult {
        self.execute(
            "CREATE TABLE IF NOT EXISTS `groups` (
                `id` bigint(20) NOT NULL AUTO_INCREMENT,
                `name` varchar(128) NOT NULL DEFAULT '',
                `description` varchar(4096) NOT NULL DEFAULT '',
                `timetable` bigint(20) NOT NULL DEFAULT '0',
                `timetable_version` int(4) unsigned DEFAULT '0',
                PRIMARY KEY ( `id` )
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
            ",
            "create_group_table"
        )
    }

    fn create_group_members_table(&self) -> EmptyResult {
        self.execute(
            "CREATE TABLE IF NOT EXISTS `group_members` (
                `id` bigint(20) NOT NULL AUTO_INCREMENT,
                `user_id` bigint(20) NOT NULL DEFAULT '0',
                `group_id` bigint(20) NOT NULL DEFAULT '0',
                PRIMARY KEY ( `id` ),
                KEY `users` ( `user_id` ),
                KEY `groups` ( `group_id` )
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
            ",
            "create_group_members_table"
        )
    }

    fn create_scheduled_events_table(&self) -> EmptyResult {
        self.execute(
            "CREATE TABLE IF NOT EXISTS `scheduled_events` (
                `id` bigint(20) NOT NULL AUTO_INCREMENT,
                `event_id` int(4) NOT NULL DEFAULT '0',
                `start_time` int(11) NOT NULL DEFAULT '0',
                `end_time` int(11) NOT NULL DEFAULT '0',
                `data` varchar(16384) NOT NULL DEFAULT '',
                PRIMARY KEY ( `id` ),
                KEY `start_time_idx` ( `start_time` ),
                KEY `end_time_idx` ( `end_time` ),
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
            ",
            "create_events_table"
        )
    }

    fn execute( &self, query: &str, name: &str ) -> EmptyResult {
        match self.pool.query( query ) {
            Ok(_)=> Ok( () ),
            Err( e ) => Err( format!( "Database {} failed: {}", name, e ) )
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