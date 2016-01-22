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
use rand::{ Rng, OsRng };
use mail_writer::MailWriter;
use mailer::Mailer;
use router_params::RouterParams;
use get_body::GetBody;
use answer_types::{ OkInfo, FieldErrorInfo };
use types::Id;

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

    let groups = groups.into_iter().map( |(id, name)| {
        let unwatched_group = unwatched_by_groups
            .iter()
            // .find(|&&(unwatched_id,_)| id == unwatched_id );
            .find(|x| id == x.0 );

        let count = match unwatched_group {
            Some( &(_, count) ) => count,
            None => 0
        };
        GroupInfo {
            id: id,
            name: name,
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
