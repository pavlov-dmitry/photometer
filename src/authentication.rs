extern crate nickel;

use self::nickel::{ Request, Response, Continue, Halt, MiddlewareResult, Middleware };
use std::collections::HashMap;
use sync::{ Arc, RWLock };
use cookies_parser::{ Cookieable };

static SESSION_ID : &'static str = "sid";

/// информация о пользователе
#[deriving(Clone)]
pub struct User {
    pub name : String,
    pub id : i64
}

impl User {
    pub fn new( name: &str, id: i64 ) -> User {
        User { 
            name: name.to_string(), 
            id: id 
        }
    }
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
}

#[deriving(Clone)]
pub struct SessionsStoreMiddleware {
    store : Arc<RWLock<SessionsStore>>
}

impl SessionsStoreMiddleware {
    pub fn user_by_session_id(&self, session_id: &String ) -> Option<User> {
        let store = self.store.read();
        store.sessions.get( session_id ).map( | ref user | { (*user).clone() } )
    }

    pub fn add_new_session( &self, user: &User ) -> String {
        let mut store = self.store.write();
        let sess_id = store.session_id_generator.gen();
        store.sessions.insert( sess_id.clone(), (*user).clone() );
        sess_id
    }
}

impl Middleware for SessionsStoreMiddleware {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.map.insert( self.clone() );
        Ok( Continue )
    } 
}

pub trait SessionsStoreable {
    fn sessions_store( &self ) -> &SessionsStoreMiddleware;
}

impl<'a, 'b> SessionsStoreable for Request<'a, 'b> {
    fn sessions_store( &self ) -> &SessionsStoreMiddleware {
        self.map.get::<SessionsStoreMiddleware>().unwrap()
    }
}

pub fn create_session_store() -> SessionsStoreMiddleware {
    SessionsStoreMiddleware { store: Arc::new( RWLock::new( SessionsStore::new() ) ) }
}

/// аутентификация пользователя
#[deriving(Clone)]
pub struct Autentication {
    login_page_path : Arc<String>
}

impl Autentication {
    fn make_login ( &self, response: &mut Response) { 
        match response.send_file( &Path::new( self.login_page_path.as_slice() ) ) {
            Ok(_) => {}
            Err( e ) => { response.send( e.desc ); }
        }
    }
}


impl Middleware for Autentication {
    fn invoke(&self, req: &mut Request, res: &mut Response) -> MiddlewareResult {

        let found = req.cookie( SESSION_ID ).map_or( None, |session| {
            req.sessions_store().user_by_session_id( session )
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

/// простой доступ из Request-a к информации о пользователе
pub fn middleware( c: &String ) -> Autentication {
    Autentication{ login_page_path : Arc::new( (*c).clone() ) }
}

pub trait Userable {
    fn user( &self ) -> &User;
}

impl<'a, 'b> Userable for Request<'a, 'b> {
    fn user( &self ) -> &User {
        self.map.get::<User>().unwrap()
    }
}