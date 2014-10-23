use nickel::{ Request, Response };
use answer;
use answer::{ Answer, AnswerSendable };
use database::{ Databaseable, DatabaseConn };
use authentication::{ SessionsStoreable, SessionsStoreMiddleware };
use params_body_parser::{ ParamsBody };

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";

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

fn not_found_param_msg( prm : &str ) -> String {
    format!( "can`t find '{}' param", prm )
}