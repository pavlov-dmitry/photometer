use answer::{ AnswerResult, AnswerSendable, Answer };
use events::events_manager::EventsManager;
use types::{ Id };
use iron::status::Status;
use std::str::FromStr;

static ID: &'static str = "id";

pub fn info_path() -> &'static str {
    "/events/info/:id"
}

pub fn info( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &request.event_info( id ) );
    Ok( Halt )
}

pub fn action_path() -> &'static str {
    "/events/action/:id"
}

pub fn trigger_path() -> &'static str {
    "/events/trigger"
}

pub fn action_get( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &request.event_action_get( id ) );
    Ok( Halt )
}
pub fn action_post( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &action_post_answer( id, request ) );
    Ok( Halt )
}

pub fn trigger( request: &mut Request, response: &mut Response ) {
    response.send_answer( &trigger_impl( request ) );
}

pub fn create_path() -> &'static str {
    "/events/create/:id"
}

pub fn create_get( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &request.event_user_creation_get( id ) );
    Ok( Halt )
}

pub fn create_post( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &create_post_answer( id, request ) );
    Ok( Halt )
}

fn trigger_impl( req: &mut Request ) -> AnswerResult {
    try!( req.maybe_start_some_events() );
    try!( req.maybe_end_some_events() );
    // стартуем после стопа еще раз, потому-что некоторые события по стопу создают новые
    try!( req.maybe_start_some_events() );

    let mut answer = Answer::new();
    answer.add_record( "trigger", &"activated".to_string() );
    Ok( answer )
}

fn action_post_answer( id: Id, req: &mut Request ) -> AnswerResult {
    let result = req.event_action_post( id );
    try!( req.maybe_start_some_events() );
    result
}

fn create_post_answer( event_id: Id, req: &mut Request ) -> AnswerResult {
    let result = req.event_user_creation_get( event_id );
    try!( req.maybe_start_some_events() );
    result
}

fn get_id( req: &Request ) -> Result<Id, NickelError> {
    let id = req.param( ID );
    FromStr::from_str( id )
        .ok_or( NickelError::new("Error parsing request path", NickelErrorKind::ErrorWithStatusCode(Status::NotFound)) )
}