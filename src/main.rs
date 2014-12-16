#![feature(macro_rules)]

extern crate nickel;
extern crate serialize;
extern crate mysql;
extern crate image;
extern crate time;
extern crate typemap;
extern crate plugin;

use nickel::{ Nickel, HttpRouter, StaticFilesHandler };

mod params_body_parser;
mod authentication;
mod config;
mod cookies_parser;
mod database;
mod answer;
mod parse_utils;
mod handlers;
mod photo_store;
//mod photo_event;
mod exif_reader;
mod types;
mod db;
mod events;
mod err_msg;
mod get_param;

fn main() {
    let cfg = config::load_or_default( &Path::new( "../etc/photometer.cfg" ) );
    let mut server = Nickel::new();
    let mut authentication_router = Nickel::router();
    let mut router = Nickel::router();

    let db = database::create_db_connection(  
        cfg.db_name.clone(),
        cfg.db_user.clone(),
        cfg.db_password.clone(),
        cfg.db_min_connections,
        cfg.db_max_connections
    ).unwrap_or_else( |e| { panic!( e ) } );


    router.get( "/hello", handlers::hello );
    router.post( "/upload", handlers::upload_photo );
    router.post( "/crop", handlers::crop_photo );
    router.post( "/rename", handlers::rename_photo );

    router.get( handlers::images::photos_path(), handlers::images::get_photo ) ;
    router.get( handlers::images::preview_path(), handlers::images::get_preview ) ;
    
    router.get( handlers::gallery::current_year_count_path(), handlers::gallery::current_year_count );
    router.get( handlers::gallery::by_year_count_path(), handlers::gallery::by_year_count );
    router.get( handlers::gallery::current_year_path(), handlers::gallery::current_year );
    router.get( handlers::gallery::by_year_path(), handlers::gallery::by_year );

    router.get( "/mailbox", handlers::mailbox::get );
    router.get( "/mailbox/unreaded", handlers::mailbox::get_unreaded );
    router.get( "/mailbox/count", handlers::mailbox::count );
    router.get( "/mailbox/unreaded/count", handlers::mailbox::count_unreaded );
    router.post( "/mailbox/mark_as_readed", handlers::mailbox::mark_as_readed );

    authentication_router.post( "/login", handlers::login ) ;
    authentication_router.post( "/join_us", handlers::join_us ) ;
    
    server.utilize( authentication::create_session_store() );
    server.utilize( db );
    server.utilize( StaticFilesHandler::new( cfg.static_files_path.as_slice() ) );
    server.utilize( 
        photo_store::middleware( 
            &cfg.photo_store_path, 
            cfg.photo_store_max_photo_size_bytes,
            cfg.photo_store_preview_size
        ) 
    );
    server.utilize( cookies_parser::middleware() );
    server.utilize( params_body_parser::middleware() );
    server.utilize( events::events_manager::middleware( &cfg.time_store_file_path ) );
    server.utilize( authentication_router );
    server.utilize( authentication::middleware( &cfg.login_page_path ) );
    server.utilize( router );

    server.listen( cfg.server_ip(), cfg.server_port );
}
