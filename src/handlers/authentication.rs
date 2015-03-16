use answer::{ Answer, AnswerResult };
use database::{ Databaseable };
use stuff::Stuffable;
use db::users::{ DbUsers };
use authentication::{ User, SessionsStoreable };
use photo_store::{ PhotoStoreable };
use err_msg;
use iron::prelude::*;
use iron::status;
use rand::{ Rng, OsRng };
use mail_writer::MailWriter;
use mailer::Mailer;
use router_params::RouterParams;
use get_body::GetBody;

/// авторизация пользователя
pub fn login( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, login_answer( request )) ) )
}

#[derive(Clone, RustcDecodable)]
struct LoginInfo {
    user: String,
    password: String
}

fn login_answer( request: &mut Request ) -> AnswerResult {
    let login_info = try!( request.get_body::<LoginInfo>() );
    let maybe_user = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.get_user( &login_info.user, &login_info.password ) )
    };
    make_login( request, maybe_user )
}

// регистрация пользователя
pub fn join_us( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, join_us_answer( request )) ) )
}

pub fn registration_end_path() -> &'static str {
    "/registration/:regkey"
}

//окончание регистрации
pub fn registration_end( req: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, registration_end_answer( req )) ) )
}

#[derive(Clone, RustcDecodable)]
struct RegInfo {
    user: String,
    password: String,
    email: String
}

fn join_us_answer( request: &mut Request ) -> AnswerResult {
    //считывание параметров
    let reg_info = try!( request.get_body::<RegInfo>() );
    //проверка на то что такого пользователя больше нет
    let user_exists = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.user_exists( &reg_info.user, &reg_info.email ) )
    };

    let mut answer = Answer::new();

    if user_exists == false { // нет такого пользователя
        let reg_key = gen_reg_key();
        let new_user = {
            let db = try!( request.stuff().get_current_db_conn() );
            try!( db.add_user( &reg_info.user, &reg_info.password, &reg_info.email, &reg_key ) )
        };
        info!( "add user {} reg_key = {}", &reg_info.user, &reg_key );
        let stuff = request.stuff();
        let (subject, mail_body) = stuff.write_registration_accept_mail( &reg_key );
        try!( stuff.send_external_mail( &new_user, &subject, &mail_body ) );
        answer.add_record( "user", &String::from_str( "added" ) );
    } 
    else {
        answer.add_error( "user", "exists" );
    }
    Ok( answer )
}

fn registration_end_answer( req: &mut Request ) -> AnswerResult {
    let regkey = req.param( "regkey" ).to_string();
    let maybe_user = {
        let db = try!( req.stuff().get_current_db_conn() );
        try!( db.activate_user( &regkey ) )
    };
    if let Some( ref user ) = maybe_user {
        try!( 
            req.photo_store().init_user_dir( &user.name )
                .map_err( |e| err_msg::fs_error( e ) ) 
        );
    }
    make_login( req, maybe_user )
}

fn make_login( req: &mut Request, maybe_user: Option<User> ) -> AnswerResult
{
    /*let maybe_user = {
        let db = try!( req.stuff().get_current_db_conn() );
        try!( db.get_user( name, pass ) )
    };*/
    let mut answer = Answer::new();
    match maybe_user {
        Some( user ) => {
            info!( "user detected: '{}':{}", user.name, user.id );
            let sess_id = req.sessions_store().add_new_session( &user );
            answer.add_record( "sid", &sess_id );
        },
        None => answer.add_error( "user", "not_found" )
    }
    Ok( answer )
}

fn gen_reg_key() -> String {
    let mut rng = OsRng::new().unwrap();
    rng.gen_ascii_chars()
        .take( 50 )
        .collect()
}
