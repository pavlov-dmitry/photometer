use nickel::{ Request, Response };
use answer::{ AnswerResult, AnswerSendable, Answer };
use database::Databaseable;
use db::events::DbEvents;
use events::events_manager::Eventsable;
use types::{ Id, CommonResult };
use err_msg;

static ID: &'static str = "id";

pub fn info_path() -> &'static str {
    "/events/info/:id"
}

pub fn info( request: &Request, response: &mut Response ) {
    response.send_answer( &info_answer( request ) );
}

pub fn action_path() -> &'static str {
    "/events/action/:id"
}

pub fn action_get( request: &Request, response: &mut Response ) {
    response.send_answer( &action_get_answer( request ) );
}
pub fn action_post( request: &Request, response: &mut Response ) {
    response.send_answer( &action_post_answer( request ) );
}

fn info_answer( req: &Request ) -> AnswerResult {
    let id = try!( get_id( req ) );
    let mut db = try!( req.get_db_conn() );
    req.events_manager().info( &mut db, id, req )
}

fn action_get_answer( req: &Request ) -> AnswerResult {
    let id = try!( get_id( req ) );
    let mut db = try!( req.get_db_conn() );
    req.events_manager().action_get( &mut db, id, req )
}

fn action_post_answer( req: &Request ) -> AnswerResult {
    let id = try!( get_id( req ) );
    let mut db = try!( req.get_db_conn() );
    req.events_manager().action_post( &mut db, id, req )
}

fn get_id( req: &Request ) -> CommonResult<Id> {
    let id = req.param( ID );
    ::std::str::from_str::<Id>( id )
        .ok_or( String::from_str( "invalid type of paramter `id` in path ( expected i64 )" ) )
}