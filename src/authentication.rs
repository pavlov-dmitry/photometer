extern crate nickel;

use self::nickel::{ Request, Response, Continue, Halt, MiddlewareResult, Middleware };
use params_body_parser::{ ParamsBody };
use std::collections::HashMap;
use sync::{ Arc, RWLock };
use cookies_parser::{ Cookieable };
use database::{ Databaseable, DatabaseConn };
use answer;
use answer::{ Answer, AnswerSendable };

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";
static SESSION_ID : &'static str = "sid";

/// информация о пользователе
#[deriving(Clone)]
pub struct User {
    pub name : String
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
        store.sessions.find( session_id ).map( | ref user | { (*user).clone() } )
    }

    pub fn add_new_session( &self, user: &String ) -> String {
        let mut store = self.store.write();
        let sess_id = store.session_id_generator.gen();
        let new_user = User { name : user.clone()};
        store.sessions.insert( sess_id.clone(), new_user );
        sess_id
    }
}

impl Middleware for SessionsStoreMiddleware {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.map.insert( self.clone() );
        Ok( Continue )
    } 
}

trait SessionsStoreable {
    fn sessions_store( &self ) -> &SessionsStoreMiddleware;
}

impl<'a, 'b> SessionsStoreable for Request<'a, 'b> {
    fn sessions_store( &self ) -> &SessionsStoreMiddleware {
        self.map.find::<SessionsStoreMiddleware>().unwrap()
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

fn make_login( db: &mut DatabaseConn, session_store: &SessionsStoreMiddleware, user: &String, pass: &String ) -> Result<Answer, String>
{
    db.user_password( user.as_slice() )
        .and_then( |maybe_password| {
            let mut answer = answer::new();
            match maybe_password {
                None => answer.add_error( "user", "not_found" ), 
                Some( password ) => {
                    if password == *pass {
                        let sess_id = session_store.add_new_session( user );
                        answer.add_record( "sid", sess_id.as_slice() );
                    } else {
                        answer.add_error( "password", "incorrect" );
                    }
                }
            }
            Ok( answer )
        })  
}

/// авторизация пользователя
pub fn login( request: &Request, response: &mut Response ) {
    let answer_result = request.parameter( USER ).map_or( Err( not_found_param_msg( USER ) ), |user|
        request.parameter( PASSWORD ).map_or( Err( not_found_param_msg( PASSWORD ) ) , |password| { 
            let mut db = request.db();
            let session_store = request.sessions_store();
            make_login( &mut db, session_store, user, password )
        } ) 
    );

    response.send_answer( &answer_result );
}

// регистрация пользователя
pub fn join_us( request: &Request, response: &mut Response ) {
    let answer_result = request.parameter( LOGIN ).map_or( Err( not_found_param_msg( LOGIN ) ), |login| 
        request.parameter( PASSWORD ).map_or( Err( not_found_param_msg( PASSWORD ) ) , |password| { 
            let mut db = request.db();
            db.user_password( login.as_slice() )
                .and_then( |maybe_password| { 
                    if maybe_password.is_none() { // нет такого пользователя
                        db.add_user( login.as_slice(), password.as_slice() )
                            .and_then( |_| make_login( &mut db, request.sessions_store(), login, password ) )
                    } else {
                        let mut answer = answer::new();
                        answer.add_error( "user", "exists" );
                        Ok( answer )
                    }
                })
        })
    );

    response.send_answer( &answer_result );
}

fn not_found_param_msg( prm : &str ) -> String {
    format!( "can`t find '{}' param", prm )
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
        self.map.find::<User>().unwrap()
    }
}