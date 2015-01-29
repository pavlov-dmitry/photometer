extern crate nickel;
extern crate mysql;
extern crate image;
extern crate time;
extern crate typemap;
extern crate plugin;
extern crate http;
extern crate url;
#[macro_use] 
extern crate log;
extern crate env_logger;
extern crate "rustc-serialize" as rustc_serialize;

use nickel::{ Nickel, HttpRouter, StaticFilesHandler, Request, Response, MiddlewareResult, ResponseFinalizer };

mod params_body_parser;
mod authentication;
mod config;
mod cookies_parser;
mod database;
mod answer;
mod parse_utils;
mod handlers;
mod photo_store;
mod exif_reader;
mod types;
mod db;
mod events;
mod err_msg;
mod get_param;
mod simple_time_profiler;
mod request_logger;

fn main() {
    env_logger::init().unwrap();
    
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

    // надеюсь этот кошмар скоро поправят (as fn(&mut Request, &mut Response))
    router.get( "/hello", handlers::hello as fn(&Request, &mut Response) );
    router.post( "/upload", handlers::upload_photo as fn(&mut Request, &mut Response) );
    router.post( "/crop", handlers::crop_photo as fn(&mut Request, &mut Response) );
    router.post( "/rename", handlers::rename_photo as fn(&mut Request, &mut Response) );

    router.get( handlers::images::photos_path(), handlers::images::get_photo as fn(&mut Request, &mut Response) );
    router.get( handlers::images::preview_path(), handlers::images::get_preview as fn(&mut Request, &mut Response) );
    
    router.get( handlers::gallery::current_year_count_path(), handlers::gallery::current_year_count as fn(&mut Request, &mut Response) );
    router.get( handlers::gallery::by_year_count_path(), handlers::gallery::by_year_count as fn(&mut Request, &mut Response) );
    router.get( handlers::gallery::current_year_path(), handlers::gallery::current_year as fn(&mut Request, &mut Response) );
    router.get( handlers::gallery::by_year_path(), handlers::gallery::by_year as fn(&mut Request, &mut Response) );

    router.get( "/mailbox", handlers::mailbox::get as fn(&mut Request, &mut Response) );
    router.get( "/mailbox/unreaded", handlers::mailbox::get_unreaded as fn(&mut Request, &mut Response) );
    router.get( "/mailbox/count", handlers::mailbox::count as fn(&mut Request, &mut Response) );
    router.get( "/mailbox/unreaded/count", handlers::mailbox::count_unreaded as fn(&mut Request, &mut Response) );
    router.post( "/mailbox/mark_as_readed", handlers::mailbox::mark_as_readed as fn(&mut Request, &mut Response) );

    router.get( handlers::events::info_path(), handlers::events::info as fn(&mut Request, &mut Response) -> MiddlewareResult );
    router.get( handlers::events::action_path(), handlers::events::action_get as fn(&mut Request, &mut Response ) -> MiddlewareResult );
    router.post( handlers::events::action_path(), handlers::events::action_post as fn(&mut Request, &mut Response) -> MiddlewareResult );
    router.get( handlers::events::create_path(), handlers::events::create_get as fn(&mut Request, &mut Response) -> MiddlewareResult );
    router.post( handlers::events::create_path(), handlers::events::create_post as fn(&mut Request, &mut Response) -> MiddlewareResult );

    router.post( handlers::timetable::timetable_path(), handlers::timetable::set_timetable as fn(&mut Request, &mut Response) -> MiddlewareResult );

    authentication_router.post( "/login", handlers::login as fn(&mut Request, &mut Response) ) ;
    authentication_router.post( "/join_us", handlers::join_us as fn(&mut Request, &mut Response) ) ;
    authentication_router.get( handlers::events::trigger_path(), handlers::events::trigger as fn(&mut Request, &mut Response) );
    
    server.utilize( request_logger::middleware() );
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
