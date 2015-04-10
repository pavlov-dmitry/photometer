#![feature(
    core,
    collections,
    std_misc,
    libc,
    net,
    old_io,
    old_path,
    fs_walk,
    path_ext
)]

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
extern crate rand;
extern crate bodyparser;
extern crate persistent;

use iron::prelude::*;
use iron::Url;
use router::Router;
use not_found_switcher::NotFoundSwitcher;
use mount::Mount;
use static_file::Static;
use std::net::SocketAddr;
use std::path::Path;
use std::io;
use std::fs::{ self, PathExt };
//use std::time::Duration;
use persistent::Read;

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
mod simple_time_profiler;
mod request_logger;
mod not_found_switcher;
mod router_params;
mod stuff;
mod trigger;
mod mailer;
mod mail_writer;
mod get_body;
mod answer_types;

use stuff::{ StuffCollection, StuffMiddleware };

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 5;

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
    router.get( handlers::gallery::photo_info_path(), handlers::gallery::photo_info );

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
    router.get( handlers::events::group_create_path(), handlers::events::group_create_get );
    router.post( handlers::events::group_create_path(), handlers::events::group_create_post );

    router.get( handlers::authentication::user_info_path(), handlers::authentication::user_info );

    let mut auth_chain = Chain::new( router );
    auth_chain.around( authentication::middleware( &Url::parse( cfg.login_page_url.as_slice() ).unwrap() ) );

    let not_found_switch_to_auth = NotFoundSwitcher::new( auth_chain );

    let mut no_auth_router = Router::new();

    no_auth_router.post( "/login", handlers::login );
    no_auth_router.post( "/join_us", handlers::join_us );
    no_auth_router.get(
        handlers::authentication::registration_end_path(),
        handlers::authentication::registration_end
    );

    no_auth_router.get( "/*", add_static_path( "../www" ) );

    let mut stuff = StuffCollection::new();
    stuff.add( db );
    let postman = mailer::create( mailer::MailContext::new(
        cfg.mail_smtp_address.as_slice(),
        cfg.mail_from_address.as_slice(),
        cfg.mail_from_pass.as_slice(),
        cfg.mail_tmp_file_path.as_slice()
    ) );
    stuff.add( postman );
    stuff.add( mail_writer::create( &cfg.root_url ) );

    let stuff_middleware = StuffMiddleware::new( stuff );
    trigger::start( cfg.events_trigger_period_sec, stuff_middleware.clone() );

    let mut chain = Chain::new( no_auth_router );
    chain.link_before( Read::<bodyparser::MaxBodyLength>::one( MAX_BODY_LENGTH ) );
    chain.link_before( authentication::create_session_store() );
    chain.link_before( stuff_middleware );
    chain.link_before(
        photo_store::middleware(
            &cfg.photo_store_path,
            cfg.photo_store_preview_size
        )
    );
    chain.around( not_found_switch_to_auth );
    chain.around( request_logger::middleware() );

    //let addr = format!( "{}:{}", cfg.server_ip(), cfg.server_port );
    let addr = SocketAddr::new( cfg.server_ip(), cfg.server_port );
    println!( "starting listen on {}", addr );
    Iron::new( chain ).http( addr ).unwrap();
}

fn add_static_path( root_path: &str ) -> Mount {
    let mut mount = Mount::new();
    { // этот блок нужен что-бы замыкание make_mount разрушилось перед возвращением mount
        let mut make_mount = |path: &str, mounted_path: &str| {
            let handler = Static::new( Path::new( mounted_path ) );
            //let handler = handler.cache( Duration::days( 365 ) );
            mount.mount( path, handler );
        };

        make_mount( "/", root_path );

        let root_path_len = root_path.len();
        let _ = visit_dirs( root_path, |p| {
            let path_str = p.to_str().unwrap();
            let mounted_path = &path_str[root_path_len ..];
            make_mount( &path_str, mounted_path );
        });
    }
    mount
}

fn visit_dirs<F: FnMut(&Path)>( root: &str, mut callback: F ) -> io::Result<()> {
    for dir in try!( fs::walk_dir( root ) ) {
        let dir = try!( dir );
        let path = dir.path();
        if path.is_dir() {
            callback( &path );
        }
    }
    Ok( () )
}
