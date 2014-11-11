use nickel::{ Request, Response };
use answer;
use answer::{ Answer, AnswerSendable };
use database::{ Databaseable, DatabaseConn };
use authentication::{ SessionsStoreable, SessionsStoreMiddleware, User };
use super::get_param::{ GetParamable };
use photo_store::{ PhotoStoreable };
use super::err_msg;

static USER : &'static str = "user";
static LOGIN : &'static str = "login";
static PASSWORD : &'static str = "password";

/// авторизация пользователя
pub fn login( request: &Request, response: &mut Response ) {
    let answer_result = 
        request.get_param( USER )
            .and_then( |user| request.get_param( PASSWORD )
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
    let answer_result = 
        request.get_param( LOGIN )
            .and_then( |login| request.get_param( PASSWORD )
                .and_then( |password| { 
                    let mut db = request.db();
                    db.user_exists( login )
                        .and_then( |exists| { 
                            if !exists { // нет такого пользователя
                                db.add_user( login, password )
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

fn make_login( db: &mut DatabaseConn, session_store: &SessionsStoreMiddleware, name: &str, pass: &str ) -> Result<Answer, String>
{
    db.get_user( name, pass )
        .and_then( |maybe_id| {
            let mut answer = answer::new();
            match maybe_id {
                None => answer.add_error( "user_pass", "not_found" ), 
                Some( id ) => {
                    let sess_id = session_store.add_new_session( &User::new( name, id ) );
                    answer.add_record( "sid", sess_id.as_slice() );
                }
            }
            Ok( answer )
        })  
}