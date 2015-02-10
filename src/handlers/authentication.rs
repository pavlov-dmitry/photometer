use answer::{ Answer, AnswerResult };
use database::{ Databaseable };
use stuff::Stuffable;
use db::users::{ DbUsers };
use authentication::{ SessionsStoreable };
use get_param::{ GetParamable };
use photo_store::{ PhotoStoreable };
use err_msg;
use iron::prelude::*;
use iron::status;

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";

/// авторизация пользователя
pub fn login( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, login_answer( request )) ) )
}

fn login_answer( request: &mut Request ) -> AnswerResult {
    let user = try!( request.get_param( USER ) ).to_string();
    let password = try!( request.get_param( PASSWORD ) ).to_string();
    make_login( request, user.as_slice(), password.as_slice() )
}

// регистрация пользователя
pub fn join_us( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, join_us_answer( request )) ) )
}

fn join_us_answer( request: &mut Request ) -> AnswerResult {
    let login = try!( request.get_param( LOGIN ) ).to_string();
    let login = login.as_slice();
    let password = try!( request.get_param( PASSWORD ) ).to_string();
    let password = password.as_slice();
    let user_exists = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.user_exists( login ) )
    };
    if !user_exists { // нет такого пользователя
        {
            let db = try!( request.stuff().get_current_db_conn() );
            try!( db.add_user( login, password ) );
        }
        try!( request.photo_store().init_user_dir( login ).map_err( |e| err_msg::fs_error( e ) ) );
        make_login( request, login, password )
    } 
    else {
        let mut answer = Answer::new();
        answer.add_error( "user", "exists" );
        Ok( answer )
    }
}

fn make_login( req: &mut Request, name: &str, pass: &str ) -> AnswerResult
{
    let maybe_user = {
        let db = try!( req.stuff().get_current_db_conn() );
        try!( db.get_user( name, pass ) )
    };
    let mut answer = Answer::new();
    match maybe_user {
        Some( user ) => {
            info!( "user detected: '{}':{}", user.name, user.id );
            let sess_id = req.sessions_store().add_new_session( &user );
            answer.add_record( "sid", &sess_id );
        },
        None => answer.add_error( "user_pass", "not_found" )
    }
    Ok( answer )
}
