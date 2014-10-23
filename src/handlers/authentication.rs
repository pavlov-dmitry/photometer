use nickel::{ Request, Response };
use answer;
use answer::{ Answer, AnswerSendable };
use database::{ Databaseable, DatabaseConn };
use authentication::{ SessionsStoreable, SessionsStoreMiddleware };
use params_body_parser::{ ParamsBody };
use photo_store::{ PhotoStoreable };
use super::err_msg;

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";

/// авторизация пользователя
pub fn login( request: &Request, response: &mut Response ) {
    let answer_result = request.parameter( USER ).ok_or( err_msg::param_not_found( USER ) )
        .and_then( |user| request.parameter( PASSWORD ).ok_or( err_msg::param_not_found( PASSWORD ) )
            .and_then( |password| { 
                let mut db = request.db();
                let session_store = request.sessions_store();
                make_login( &mut db, session_store, user, password )
            } ) 
        );

    response.send_answer( &answer_result );
}

// регистрация пользователя
pub fn join_us( request: &Request, response: &mut Response ) {
    let answer_result = request.parameter( LOGIN ).ok_or( err_msg::param_not_found( LOGIN ) )
        .and_then( |login| request.parameter( PASSWORD ).ok_or( err_msg::param_not_found( PASSWORD ) )
            .and_then( |password| { 
                let mut db = request.db();
                db.user_password( login.as_slice() )
                    .and_then( |maybe_password| { 
                        if maybe_password.is_none() { // нет такого пользователя
                            db.add_user( login.as_slice(), password.as_slice() )
                                .and_then( |_| 
                                    request.photo_store().init_user_dir( login ).map_err( |e| err_msg::fs_error( e ) ) 
                                )
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