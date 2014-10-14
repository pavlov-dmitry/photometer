extern crate nickel;
extern crate serialize;
extern crate sync;
extern crate mysql;

use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter, StaticFilesHandler };
use authentication::{ Userable };
use sync::Arc;

mod params_body_parser;
mod stuff;
mod authentication;
mod config;
mod cookies_parser;
mod database;

fn hello ( request: &Request, response: &mut Response) { 
	let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
	response.send( answer );
}

fn main() {
    let cfg = Arc::new( config::load_or_default( &Path::new( "../etc/photometer.cfg" ) ) );
    let mut server = Nickel::new();
    let mut authentication_router = Nickel::router();
    let mut router = Nickel::router();

    let db = database::create_db_connection(  
        cfg.db_name.clone(),
        cfg.db_user_name.clone(),
        cfg.db_user_password.clone()
    ).unwrap_or_else( |e| { fail!( e ) } );


    let stuff = stuff::new( authentication::SessionsStore::new() );


    router.get( "/hello", hello );

    authentication_router.post( "/login", authentication::login ) ;

    server.utilize( stuff );
    server.utilize( db );
    server.utilize( params_body_parser::middleware() );
    server.utilize( cookies_parser::middleware() );
    server.utilize( StaticFilesHandler::new( cfg.static_files_path.as_slice() ) );
	server.utilize( authentication_router );
    server.utilize( authentication::middleware( cfg.clone() ) );
    server.utilize( router );

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
