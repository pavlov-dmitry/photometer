extern crate nickel;

use std::io::net::ip::Ipv4Addr;

use nickel::{ Nickel, Request, Response, HttpRouter };

use authentication::{ Userable };

mod params_body_parser;
mod stuff;
mod authentication;

fn hello ( request: &Request, response: &mut Response) { 
	let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
	response.send( answer );
}

fn main() {
    let mut server = Nickel::new();
    let mut authentication_router = Nickel::router();
    let mut router = Nickel::router();

    let stuff = stuff::new( authentication::SessionsStore::new() );


    router.get( "/hello", hello );

    authentication_router.post( "/login", authentication::login ) ;

    server.utilize( stuff );
    server.utilize( params_body_parser::middleware() );
	server.utilize( authentication_router );
    server.utilize( authentication::middleware() );
    server.utilize( router );

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
