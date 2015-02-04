extern crate iron;
extern crate mysql;
extern crate image;
extern crate time;
extern crate url;
#[macro_use] 
extern crate log;
extern crate env_logger;
extern crate "rustc-serialize" as rustc_serialize;
extern crate router;
extern crate mount;
extern crate "static" as static_file;

use iron::prelude::*;
use iron::Url;
use router::Router;
use not_found_switcher::NotFoundSwitcher;
use mount::Mount;
use static_file::Static;

mod params_body_parser;
mod authentication;
mod config;
mod cookies;
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
mod not_found_switcher;
mod router_params;

fn main() {
    env_logger::init().unwrap();
    
    let cfg = config::load_or_default( &Path::new( "../etc/photometer.cfg" ) );

    let db = database::create_db_connection(  
        cfg.db_name.clone(),
        cfg.db_user.clone(),
        cfg.db_password.clone(),
        cfg.db_min_connections,
        cfg.db_max_connections
    ).unwrap_or_else( |e| { panic!( e ) } );

    let mut router = Router::new();

    router.get( "/hello", handlers::hello );
    router.post( "/upload", handlers::upload_photo );
    router.post( "/crop", handlers::crop_photo );
    router.post( "/rename", handlers::rename_photo );

    router.get( handlers::images::photos_path(), handlers::images::get_photo );
    router.get( handlers::images::preview_path(), handlers::images::get_preview );
    
    router.get( handlers::gallery::current_year_count_path(), handlers::gallery::current_year_count );
    router.get( handlers::gallery::by_year_count_path(), handlers::gallery::by_year_count );
    router.get( handlers::gallery::current_year_path(), handlers::gallery::current_year );
    router.get( handlers::gallery::by_year_path(), handlers::gallery::by_year );

    router.get( "/mailbox", handlers::mailbox::get );
    router.get( "/mailbox/unreaded", handlers::mailbox::get_unreaded );
    router.get( "/mailbox/count", handlers::mailbox::count );
    router.get( "/mailbox/unreaded/count", handlers::mailbox::count_unreaded );
    router.post( "/mailbox/mark_as_readed", handlers::mailbox::mark_as_readed );

    router.get( handlers::events::info_path(), handlers::events::info );
    router.get( handlers::events::action_path(), handlers::events::action_get );
    router.post( handlers::events::action_path(), handlers::events::action_post );
    router.get( handlers::events::create_path(), handlers::events::create_get );
    router.post( handlers::events::create_path(), handlers::events::create_post );

    router.post( handlers::timetable::timetable_path(), handlers::timetable::set_timetable );

    let mut auth_chain = Chain::new( router );
    auth_chain.around( authentication::middleware( &Url::parse( &cfg.login_page_path[] ).unwrap() ) );
    auth_chain.link_before(
        photo_store::middleware( 
            &cfg.photo_store_path, 
            cfg.photo_store_max_photo_size_bytes,
            cfg.photo_store_preview_size
        )
    );

    let not_found_switch_to_auth = NotFoundSwitcher::new( auth_chain );

    let mut no_auth_router = Router::new();

    no_auth_router.post( "/login", handlers::login );
    no_auth_router.post( "/join_us", handlers::join_us );
    no_auth_router.get( handlers::events::trigger_path(), handlers::events::trigger );

    let mut static_mount = Mount::new();
    static_mount.mount( "/static/", Static::new( Path::new( "../www/" ) ) );
    no_auth_router.get( "/static/*", static_mount );
    
    let mut chain = Chain::new( no_auth_router );
    chain.link_before( authentication::create_session_store() );
    //chain.link_before( request_logger::middleware() );
    chain.link_before( db );
    chain.link_before( params_body_parser::middleware() );
    chain.link_before( events::events_manager::middleware( &cfg.time_store_file_path ) );
    chain.around( not_found_switch_to_auth );
    chain.around( request_logger::middleware() );

    let addr = format!( "{}:{}", cfg.server_ip(), cfg.server_port );
    println!( "starting listen on {}", addr );
    Iron::new( chain ).listen( addr.as_slice() ).unwrap();
}
