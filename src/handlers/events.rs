use nickel::{ Request, Response, NickelError, NickelErrorKind, Halt, MiddlewareResult };
use answer::{ AnswerResult, AnswerSendable };
use database::Databaseable;
use events::events_manager::Eventsable;
use types::{ Id, CommonResult };
use http::status::Status;

static ID: &'static str = "id";

pub fn info_path() -> &'static str {
    "/events/info/:id"
}

pub fn info( request: &Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &info_answer( id, request ) );
    Ok( Halt )
}

pub fn action_path() -> &'static str {
    "/events/action/:id"
}

pub fn action_get( request: &Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &action_get_answer( id, request ) );
    Ok( Halt )
}
pub fn action_post( request: &Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &action_post_answer( id, request ) );
    Ok( Halt )
}

fn info_answer( id: Id, req: &Request ) -> AnswerResult {
    let mut db = try!( req.get_db_conn() );
    req.events_manager().info( &mut db, id, req )
}

fn action_get_answer( id: Id, req: &Request ) -> AnswerResult {
    let mut db = try!( req.get_db_conn() );
    req.events_manager().action_get( &mut db, id, req )
}

fn action_post_answer( id: Id, req: &Request ) -> AnswerResult {
    let mut db = try!( req.get_db_conn() );
    req.events_manager().action_post( &mut db, id, req )
}

fn get_id( req: &Request ) -> Result<Id, NickelError> {
    let id = req.param( ID );
    ::std::str::from_str::<Id>( id )
        .ok_or( NickelError::new("Error parsing request path", NickelErrorKind::ErrorWithStatusCode(Status::NotFound)) )
}