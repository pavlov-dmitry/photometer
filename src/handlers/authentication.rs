use nickel::{ Request, Response };
use answer;
use answer::{ Answer, AnswerSendable, AnswerResult };
use database::{ Databaseable, DbConnection };
use db::users::{ DbUsers };
use authentication::{ SessionsStoreable, SessionsStoreMiddleware, User };
use super::get_param::{ GetParamable };
use photo_store::{ PhotoStoreable };
use super::err_msg;
use types::{ CommonResult };

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";

/// авторизация пользователя
pub fn login( request: &Request, response: &mut Response ) {
    response.send_answer( &login_answer( request ) );
}

fn login_answer( request: &Request ) -> CommonResult<Answer> {
    let user = try!( request.get_param( USER ) );
    let password = try!( request.get_param( PASSWORD ) );
    let session_store = request.sessions_store();
    let mut db_conn = try!( request.get_db_conn() );
    make_login( &mut db_conn, session_store, user, password )
}

// регистрация пользователя
pub fn join_us( request: &Request, response: &mut Response ) {
    response.send_answer( &join_us_answer( request ) );
}

fn join_us_answer( request: &Request ) -> AnswerResult {
    let login = try!( request.get_param( LOGIN ) );
    let password = try!( request.get_param( PASSWORD ) );
    let mut db = try!( request.get_db_conn() );
    let user_exists = try!( db.user_exists( login ) );
    if !user_exists { // нет такого пользователя
        try!( db.add_user( login, password ) );
        try!( request.photo_store().init_user_dir( login ).map_err( |e| err_msg::fs_error( e ) ) );
        make_login( &mut db, request.sessions_store(), login, password )
    } 
    else {
        let mut answer = answer::new();
        answer.add_error( "user", "exists" );
        Ok( answer )
    }
}

fn make_login( db: &mut DbConnection, session_store: &SessionsStoreMiddleware, name: &str, pass: &str ) -> AnswerResult
{
    let maybe_id = try!( db.get_user( name, pass ) );
    let mut answer = answer::new();
    match maybe_id {
        Some( id ) => {
            let sess_id = session_store.add_new_session( &User::new( name, id ) );
            answer.add_record( "sid", &sess_id );
        },
        None => answer.add_error( "user_pass", "not_found" )
    }
    Ok( answer )
}