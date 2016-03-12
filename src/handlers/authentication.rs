use answer::{ Answer, AnswerResult, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use db::users::{ DbUsers };
use db::mailbox::{ DbMailbox };
use db::groups::{ DbGroups };
use db::group_feed::{ DbGroupFeed };
use authentication::{ User, SessionsStoreable, Userable };
use photo_store::{ PhotoStoreable };
use err_msg;
use iron::prelude::*;
use iron::status;
use rand::{ Rng, OsRng };
use mail_writer::MailWriter;
use mailer::Mailer;
use router_params::RouterParams;
use get_body::GetBody;
use answer_types::{ OkInfo, FieldErrorInfo };
use types::{ Id, ShortInfo };
use std::str::FromStr;
use time;
use parse_utils::GetMsecs;
use crypto;
use rustc_serialize::base64;
use rustc_serialize::base64::ToBase64;

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
        let regkey = try!( db.get_reg_key( &login_info.user ) );
        match regkey {
            Some( regkey ) => {
                let password_hash = make_password_hash( &regkey, &login_info.password );
                try!( db.get_user( &login_info.user, &password_hash ) )
            },
            None => None
        }
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

    //ряд простызх проверок
    if reg_info.user.is_empty() {
        return Ok( Answer::bad( FieldErrorInfo::empty( "user" ) ) );
    }
    if 24 < reg_info.user.len() {
        return Ok( Answer::bad( FieldErrorInfo::too_long( "user" ) ) );
    }
    if reg_info.email.is_empty() {
        return Ok( Answer::bad( FieldErrorInfo::empty( "email" ) ) );
    }
    if 128 < reg_info.email.len() {
        return Ok( Answer::bad( FieldErrorInfo::too_long( "email" ) ) );
    }

    //проверка на то что такого пользователя больше нет
    let user_exists = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.user_exists( &reg_info.user, &reg_info.email ) )
    };

    let answer = if user_exists == false { // нет такого пользователя
        let reg_key = gen_reg_key();
        let password_hash = make_password_hash( &reg_key, &reg_info.password );
        let new_user = {
            let db = try!( request.stuff().get_current_db_conn() );
            try!( db.add_user( &reg_info.user,
                               &password_hash,
                               &reg_info.email,
                               &reg_key,
                               time::get_time().msecs() ) )
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

fn make_password_hash( reg_key: &str, password: &str ) -> String {
    let reg_key_bytes: Vec<u8> = reg_key.bytes().take( 16 ).collect();
    let password_bytes: Vec<u8> = password.bytes().collect();
    let mut password_hash: [u8; 24] = [0; 24];
    crypto::bcrypt::bcrypt( 12, &reg_key_bytes, &password_bytes, &mut password_hash );
    let password_string = password_hash.to_base64( base64::STANDARD );
    password_string
}

fn registration_end_answer( req: &mut Request ) -> AnswerResult {
    let regkey = match req.param( "regkey" ) {
        Some( regkey ) => regkey.to_owned(),
        None => return Ok( Answer::not_found() )
    };
    let maybe_user = {
        let db = try!( req.stuff().get_current_db_conn() );
        try!( db.activate_user( &regkey ) )
    };
    if let Some( ref user ) = maybe_user {
        try!(
            req.photo_store().init_user_dir( &user.name )
                .map_err( |e| err_msg::fs_error( e ) )
        );

        // посылаем приветственное письмо( но только внутри фотометра )
        let stuff = req.stuff();
        let (subject, mail_body) = stuff.write_welcome_mail();
        try!( stuff.send_internal_mail( &user, &subject, &mail_body ) );
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
    id: Id,
    unreaded_messages_count: u32,
    groups: Vec<GroupInfo>
}

#[derive(RustcEncodable)]
struct GroupInfo {
    id: Id,
    name: String,
    unwatched_events: u32
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
    let stuff = req.stuff();
    let db = try!( stuff.get_current_db_conn() );

    let unreaded_count = try!( db.messages_count( user_id, true ) );
    let groups = try!( db.member_in_groups( user_id ) );
    let unwatched_by_groups = try!( db.get_unwatched_feed_elements_by_groups( user_id ) );

    let groups = groups.into_iter().map( |group| {
        let unwatched_group = unwatched_by_groups
            .iter()
            .find(|x| group.id == x.0 );

        let count = match unwatched_group {
            Some( &(_, count) ) => count,
            None => 0
        };
        GroupInfo {
            id: group.id,
            name: group.name,
            unwatched_events: count
        }
    }).collect();

    let user_info = UserInfo {
        name: user_name,
        id: user_id,
        unreaded_messages_count: unreaded_count,
        groups: groups
    };

    Ok( Answer::good( user_info ) )
}

#[derive(RustcEncodable)]
struct UserDescription {
    id: Id,
    name: String,
    join_time: u64,
    groups: Vec<ShortInfo>
}

pub fn user_desc( req: &mut Request ) -> IronResult<Response> {
    let id: u64 = match req.param( "id" ) {
        Some( id ) => match FromStr::from_str( id ).ok() {
            Some( id ) => id,
            None => return Ok( Response::with( status::NotFound ) )
        },
        None => return Ok( Response::with( status::NotFound ) )
    };

    let answer = AnswerResponse( user_desc_answer(req, id) );
    Ok( Response::with( answer ) )
}

fn user_desc_answer( req: &mut Request, id: Id ) -> AnswerResult {
    let db = try!( req.stuff().get_current_db_conn() );
    let user_info = try!( db.user_by_id( id ) );
    let answer = match user_info {
        Some( info ) => {
            let groups = try!( db.member_in_groups( id ) );
            let desc = UserDescription {
                id: id,
                name: info.name,
                join_time: info.join_time,
                groups: groups
            };
            Answer::good( desc )
        },
        None => Answer::not_found()
    };
    Ok( answer )
}
