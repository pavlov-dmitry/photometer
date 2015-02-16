use answer::{ Answer, AnswerResult };
use database::{ Databaseable };
use stuff::Stuffable;
use db::users::{ DbUsers };
use authentication::{ User, SessionsStoreable };
use get_param::{ GetParamable };
use photo_store::{ PhotoStoreable };
use err_msg;
use iron::prelude::*;
use iron::status;
use rand::{ Rng, OsRng };
use mail_writer::MailWriter;
use mailer::Mailer;
use router_params::RouterParams;

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";
static MAIL : &'static str = "mail";

/// авторизация пользователя
pub fn login( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, login_answer( request )) ) )
}

fn login_answer( request: &mut Request ) -> AnswerResult {
    let user = try!( request.get_param( USER ) ).to_string();
    let password = try!( request.get_param( PASSWORD ) ).to_string();
    let maybe_user = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.get_user( &user[], &password[] ) )
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

fn join_us_answer( request: &mut Request ) -> AnswerResult {
    //считывание параметров
    let login = try!( request.get_param( LOGIN ) ).to_string();
    let login = login.as_slice();
    let password = try!( request.get_param( PASSWORD ) ).to_string();
    let password = password.as_slice();
    let mail = try!( request.get_param( MAIL ) ).to_string();
    let mail = mail.as_slice();
    //проверка на то что такого пользователя больше нет
    let user_exists = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.user_exists( login, mail ) )
    };

    let mut answer = Answer::new();

    if user_exists == false { // нет такого пользователя
        let reg_key = gen_reg_key();
        let reg_key = reg_key.as_slice();
        let new_user = {
            let db = try!( request.stuff().get_current_db_conn() );
            try!( db.add_user( login, password, mail, reg_key ) )
        };
        info!( "add user {} reg_key = {}", login, reg_key );
        let stuff = request.stuff();
        let mail_body = stuff.write_registration_accept_mail( reg_key );
        try!( stuff.send_external_mail( &new_user, "Фотометр", "Регистрация", mail_body.as_slice() ) );
        //try!( request.photo_store().init_user_dir( login ).map_err( |e| err_msg::fs_error( e ) ) );
        //make_login( request, login, password )
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
        try!( db.activate_user( regkey.as_slice() ) )
    };
    if let Some( ref user ) = maybe_user {
        try!( 
            req.photo_store().init_user_dir( user.name.as_slice() )
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
