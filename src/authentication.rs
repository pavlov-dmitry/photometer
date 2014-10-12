extern crate nickel;

use self::nickel::{ Request, Response, Continue, Halt, MiddlewareResult, Middleware };
use params_body_parser::{ ParamsBody };
use std::collections::HashMap;
use stuff::{ Stuffable };

#[deriving(Clone)]
pub struct User {
	pub name : String,
	password : String
}

type SessionsHash = HashMap<String, User>;

struct SessionIdGenerator {
	state : int
}

impl SessionIdGenerator {
	fn new() -> SessionIdGenerator {
		SessionIdGenerator { state : 0i }
	}
	fn gen(&mut self) -> String {
		self.state += 1;
		format!( "{}", self.state )
	}
}


pub struct SessionsStore {
	sessions : SessionsHash,
	session_id_generator : SessionIdGenerator
}

impl SessionsStore {
	pub fn new() -> SessionsStore {
		SessionsStore { 
			sessions : HashMap::new(),  
			session_id_generator : SessionIdGenerator::new()
		}
	}
	fn user_by_session_id(&self, session_id: &String ) -> Option<User> {
		self.sessions.find( session_id ).map( | ref user | { (*user).clone() } )
	}

	fn add_new_session( &mut self, user: &String, password: &String ) -> String {
		let sess_id = self.session_id_generator.gen();
    	let new_user = User { name : user.clone(), password : password.clone() };
    	self.sessions.insert( sess_id.clone(), new_user );
    	sess_id
	}
}

#[deriving(Clone)]
pub struct Autentication;

fn make_login ( response: &mut Response) { 
    match response.send_file( &Path::new( "../www/login.html" ) ) {
    	Ok(_) => {}
    	Err( e ) => { response.send( e.desc ); }
    }
}

impl Middleware for Autentication {
	fn invoke(&self, req: &mut Request, res: &mut Response) -> MiddlewareResult {

		let found = req.parameter( "session_id" ).map_or( None, |session| {
			let session_store = req.stuff().sessions_store_for.read();
			session_store.user_by_session_id( session )
		} );

		match found {
			None => { 
				make_login( res ); 
				Ok( Halt )
			}
			Some( user ) => {
				req.map.insert( user );
				Ok( Continue )
			}
		}
    } 
}

pub fn login( request: &Request, response: &mut Response ) {
	let answer_str = request.parameter( "user" ).map_or( "can`t find user param".to_string(), |ref user| { 
        request.parameter( "password" ).map_or( "can`t find password param".to_string() , |ref password| { 
        	let mut session_store = request.stuff().sessions_store_for.write();
        	let sess_id = session_store.add_new_session( user.clone(), password.clone() );
            format!( "logging as {} with password {} session_id = {}", user, password, sess_id )
        } ) 
    } );

	response.send( answer_str.as_slice() );
}

pub fn middleware() -> Autentication {
	Autentication
}

pub trait Userable {
	fn user( &self ) -> &User;
}

impl<'a, 'b> Userable for Request<'a, 'b> {
	fn user( &self ) -> &User {
		self.map.find::<User>().unwrap()
	}
}