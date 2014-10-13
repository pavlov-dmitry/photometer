extern crate nickel;

use self::nickel::{ Request, Response, Continue, Halt, MiddlewareResult, Middleware };
use params_body_parser::{ ParamsBody };
use std::collections::HashMap;
use stuff::{ Stuffable };
use sync::Arc;
use cookies_parser::{ Cookieable };

use config::Config;

use serialize::json;

static USER : &'static str = "user";
static PASSWORD : &'static str = "password";
static SESSION_ID : &'static str = "sid";

/// информация о пользователе
#[deriving(Clone)]
pub struct User {
	pub name : String,
	password : String
}

/// хранилице информации о автиынх пользователях
type SessionsHash = HashMap<String, User>;

struct SessionIdGenerator {
	state : int
}

/// примитивный генератор идентификаторов сессий
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


/// аутентификация пользователя
#[deriving(Clone)]
pub struct Autentication {
	cfg : Arc<Config>
}

impl Autentication {
	fn make_login ( &self, response: &mut Response) { 
	    match response.send_file( &Path::new( self.cfg.login_page_path.as_slice() ) ) {
	    	Ok(_) => {}
	    	Err( e ) => { response.send( e.desc ); }
	    }
	}
}


impl Middleware for Autentication {
	fn invoke(&self, req: &mut Request, res: &mut Response) -> MiddlewareResult {

		let found = req.cookie( SESSION_ID ).map_or( None, |session| {
			let session_store = req.stuff().sessions_store_for.read();
			session_store.user_by_session_id( session )
		} );

		match found {
			None => { 
				self.make_login( res ); 
				Ok( Halt )
			}
			Some( user ) => {
				req.map.insert( user );
				Ok( Continue )
			}
		}
    } 
}

/// авторизация пользователя
pub fn login( request: &Request, response: &mut Response ) {
	let answer_result = request.parameter( USER ).map_or( Err( not_found_param_msg( USER ) ), |ref user| { 
    	request.parameter( PASSWORD ).map_or( Err( not_found_param_msg( PASSWORD ) ) , |ref password| { 
			let mut session_store = request.stuff().sessions_store_for.write();
        	let sess_id = session_store.add_new_session( user.clone(), password.clone() );
            Ok( json::encode( &AuthAnswer{ sid : sess_id } ) )
        } ) 
    } );

	match answer_result {
		Err( err_desc ) => response.send( err_desc.as_slice() ),
		Ok( answer ) => {
			response.content_type( "application/json" );
			response.send( answer.as_slice() );
		}
	}
}

fn not_found_param_msg( prm : &str ) -> String {
    format!( "can`t find '{}' param", prm )
}

#[deriving(Encodable)]
struct AuthAnswer {
	sid : String
}


/// простой доступ из Request-a к информации о пользователе
pub fn middleware( c: Arc<Config> ) -> Autentication {
	Autentication{ cfg : c }
}

pub trait Userable {
	fn user( &self ) -> &User;
}

impl<'a, 'b> Userable for Request<'a, 'b> {
	fn user( &self ) -> &User {
		self.map.find::<User>().unwrap()
	}
}