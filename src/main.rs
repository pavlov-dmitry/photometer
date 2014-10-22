extern crate nickel;
extern crate serialize;
extern crate sync;
extern crate mysql;

use nickel::{ Nickel, Request, Response, HttpRouter, StaticFilesHandler };
use authentication::{ Userable };
use sync::Arc;

use std::io::{ File };
use params_body_parser::{ ParamsBody };

mod params_body_parser;
mod authentication;
mod config;
mod cookies_parser;
mod database;
mod answer;
mod parse_utils;

fn hello ( request: &Request, response: &mut Response) { 
    let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
    response.send( answer );
}

fn test_upload( request: &Request, response: &mut Response ) {
    match request.bin_parameter( "upload_img" ) {
        Some( data ) => {
            let path = Path::new( "../data/img.jpg" );
            let mut writed = false;
            {
                File::create( &path )
                    .and_then( |mut file| file.write( data ) )
                    .map( |_| writed = true )
                    .unwrap_or_else( |e| response.send( format!( "{}", e ) ) )
            }
            if writed == true {
                response.send_file( &path );
            }
        }
        None => response.send( "no upload_img" )
    }

}

fn main() {
    let cfg = Arc::new( config::load_or_default( &Path::new( "../etc/photometer.cfg" ) ) );
    let mut server = Nickel::new();
    let mut authentication_router = Nickel::router();
    let mut router = Nickel::router();

    let db = database::create_db_connection(  
        cfg.db_name.clone(),
        cfg.db_user.clone(),
        cfg.db_password.clone(),
        cfg.db_min_connections,
        cfg.db_max_connections
    ).unwrap_or_else( |e| { fail!( e ) } );


    router.get( "/hello", hello );
    router.post( "/upload", test_upload );

    authentication_router.post( "/login", authentication::login ) ;
    authentication_router.post( "/join_us", authentication::join_us ) ;

    server.utilize( authentication::create_session_store() );
    server.utilize( db );
    server.utilize( params_body_parser::middleware() );
    server.utilize( cookies_parser::middleware() );
    server.utilize( StaticFilesHandler::new( cfg.static_files_path.as_slice() ) );
    server.utilize( authentication_router );
    server.utilize( authentication::middleware( &cfg.login_page_path ) );
    server.utilize( router );

    server.listen( cfg.server_ip(), cfg.server_port );
}
