use answer::{ Answer, AnswerResult, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use db::users::{ DbUsers };
use db::mailbox::{ DbMailbox };
use authentication::{ User, SessionsStoreable, Userable };
use photo_store::{ PhotoStoreable };
use err_msg;
use iron::prelude::*;
use rand::{ Rng, OsRng };
use mail_writer::MailWriter;
use mailer::Mailer;
use router_params::RouterParams;
use get_body::GetBody;
use answer_types::{ OkInfo, FieldErrorInfo };

/// авторизация пользователя
pub fn login( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( AnswerResponse( login_answer( request )) ) )
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
    Ok( Response::with( AnswerResponse( join_us_answer( request )) ) )
}

pub fn registration_end_path() -> &'static str {
    "/registration/:regkey"
}

//окончание регистрации
pub fn registration_end( req: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( AnswerResponse( registration_end_answer( req )) ) )
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

    let answer = if user_exists == false { // нет такого пользователя
        let reg_key = gen_reg_key();
        let new_user = {
            let db = try!( request.stuff().get_current_db_conn() );
            try!( db.add_user( &reg_info.user, &reg_info.password, &reg_info.email, &reg_key ) )
        };
        info!( "add user {} reg_key = {}", &reg_info.user, &reg_key );
        let stuff = request.stuff();
        let (subject, mail_body) = stuff.write_registration_accept_mail( &reg_key );
        try!( stuff.send_external_mail( &new_user, &subject, &mail_body ) );
        Answer::good( OkInfo::new( "added" ) )
    }
    else {
        Answer::bad( FieldErrorInfo::new( "user", "exists" ) )
    };
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


#[derive(RustcEncodable)]
struct SidInfo {
    sid: String
}

fn make_login( req: &mut Request, maybe_user: Option<User> ) -> AnswerResult
{
    let answer = match maybe_user {
        Some( user ) => {
            info!( "user detected: '{}':{}", user.name, user.id );
            Answer::good( SidInfo {
                sid: req.sessions_store().add_new_session( &user )
            })
        },

        None => Answer::bad( FieldErrorInfo::new( "user", "not_found" ) )
    };
    Ok( answer )
}

fn gen_reg_key() -> String {
    let mut rng = OsRng::new().unwrap();
    rng.gen_ascii_chars()
        .take( 50 )
        .collect()
}

#[derive(RustcEncodable)]
struct UserInfo {
    name: String,
    unreaded_messages_count: u32
}

pub fn user_info_path() -> &'static str {
    "/user_info"
}

pub fn user_info( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( user_info_answer(req) );
    Ok( Response::with( answer ) )
}

fn user_info_answer( req: &mut Request ) -> AnswerResult {
    let user_id = req.user().id;
    let user_name = req.user().name.clone();

    let unreaded_count = {
        let db = try!( req.stuff().get_current_db_conn() );
        try!( db.messages_count( user_id, true ) )
    };

    let user_info = UserInfo {
        name: user_name,
        unreaded_messages_count: unreaded_count
    };

    Ok( Answer::good( user_info ) )
}
