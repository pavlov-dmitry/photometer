use std::collections::HashMap;
use std::sync::{ Arc, RwLock };
use cookies::{ Cookieable };
use iron::middleware::{ BeforeMiddleware, AroundMiddleware };
use iron::prelude::*;
use iron::typemap::Key;
use iron::{ Handler, status, Url };
use iron::modifiers::Redirect;
use types::Id;

static SESSION_ID : &'static str = "sid";

/// информация о пользователе
#[derive(Clone)]
pub struct User {
    pub name : String,
    pub id : Id,
    pub mail: String
}

/*impl User {
    pub fn new( name: &str, id: Id ) -> User {
        User { 
            name: name.to_string(), 
            id: id 
        }
    }
}*/

/// хранилице информации о активных пользователях
pub type SessionId = String;
type SessionsHash = HashMap<SessionId, User>;
type UserIdSessionsHash = HashMap<Id, SessionId>;

struct SessionIdGenerator {
    state : Id
}

/// примитивный генератор идентификаторов сессий
impl SessionIdGenerator {
    fn new() -> SessionIdGenerator {
        SessionIdGenerator { state : 0 }
    }
    fn gen(&mut self) -> SessionId {
        self.state += 1;
        format!( "{}", self.state )
    }
}

pub struct SessionsStore {
    sessions : SessionsHash,
    sessions_by_user : UserIdSessionsHash,
    session_id_generator : SessionIdGenerator
}

impl SessionsStore {
    pub fn new() -> SessionsStore {
        SessionsStore { 
            sessions : HashMap::new(), 
            sessions_by_user : HashMap::new(), 
            session_id_generator : SessionIdGenerator::new()
        }
    }
}

#[derive(Clone)]
pub struct SessionsStoreMiddleware {
    store : Arc<RwLock<SessionsStore>>
}

impl SessionsStoreMiddleware {
    pub fn user_by_session_id(&self, session_id: &SessionId ) -> Option<User> {
        let store = self.store.read().unwrap();
        store.sessions.get( session_id ).map( |user| { (*user).clone() } )
    }

    pub fn add_new_session( &self, user: &User ) -> SessionId {
        let mut store = self.store.write().unwrap();
        // удаляем старую сессию этого пользователя
        let sess_id = store.session_id_generator.gen();
        if let Some( old_session_id ) = store.sessions_by_user.insert( user.id, sess_id.clone() ) {
            debug!( "remove old session id={} for user='{}'", old_session_id, user.name );
            store.sessions.remove( &old_session_id );
        }
        // создаём новую
        debug!( "create session id={} for user='{}'", sess_id, user.name );
        store.sessions.insert( sess_id.clone(), (*user).clone() );
        sess_id
    }
}

impl Key for SessionsStoreMiddleware { type Value = SessionsStoreMiddleware; }

impl BeforeMiddleware for SessionsStoreMiddleware {
    fn before( &self, req: &mut Request ) -> IronResult<()> {
        req.extensions.insert::<SessionsStoreMiddleware>( (*self).clone() );
        Ok( () )
    } 
}

pub trait SessionsStoreable {
    fn sessions_store( &self ) -> &SessionsStoreMiddleware;
}

impl<'a> SessionsStoreable for Request<'a> {
    fn sessions_store( &self ) -> &SessionsStoreMiddleware {
        self.extensions.get::<SessionsStoreMiddleware>().unwrap()
    }
}

pub fn create_session_store() -> SessionsStoreMiddleware {
    SessionsStoreMiddleware { store: Arc::new( RwLock::new( SessionsStore::new() ) ) }
}

/// аутентификация пользователя
#[derive(Clone)]
pub struct Autentication {
    login_url : Arc<Url>
}

pub struct AuthenticationHandler<H: Handler> {
    handler: H,
    authentication: Autentication
}

impl Autentication {
    fn make_login ( &self ) -> IronResult<Response> { 
        Ok( Response::with(( status::Found, Redirect( (*self.login_url).clone() ) )) )
    }
}

impl Key for User { type Value = User; }

impl AroundMiddleware for Autentication {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new( AuthenticationHandler {
            handler: handler,
            authentication: self
        })
    }
}

impl<H: Handler> Handler for AuthenticationHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        debug!( "session_id_cookie={:?}", req.cookie( SESSION_ID ) );
        let some_user = req.cookie( SESSION_ID ).map_or( None, |session| {
            req.sessions_store().user_by_session_id( session )
        } );

        match some_user {
            None => { 
                self.authentication.make_login()
            }
            Some( user ) => {
                req.extensions.insert::<User>( user );
                self.handler.handle( req )
            }
        }
    } 
}

/// простой доступ из Request-a к информации о пользователе
pub fn middleware( c: &Url ) -> Autentication {
    Autentication{ login_url : Arc::new( c.clone() ) }
}

pub trait Userable {
    fn user( &self ) -> &User;
}

impl<'a> Userable for Request<'a> {
    fn user( &self ) -> &User {
        self.extensions.get::<User>().unwrap()
    }
}