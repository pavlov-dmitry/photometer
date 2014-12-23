use nickel::{ Request, Response, NickelError, NickelErrorKind, Halt, MiddlewareResult };
use answer::{ AnswerResult, AnswerSendable, Answer };
use database::Databaseable;
use events::events_manager::Eventsable;
use types::{ Id };
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

pub fn trigger_path() -> &'static str {
    "/events/trigger"
}

pub fn action_get( request: &Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &action_get_answer( id, request ) );
    Ok( Halt )
}
pub fn action_post( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &action_post_answer( id, request ) );
    Ok( Halt )
}

pub fn trigger( request: &Request, response: &mut Response ) {
    response.send_answer( &trigger_impl( request ) );
}

pub fn create_path() -> &'static str {
    "/events/create/:id"
}

pub fn create_get( request: &Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &create_get_answer( id, request ) );
    Ok( Halt )
}

pub fn create_post( request: &mut Request, response: &mut Response ) -> MiddlewareResult {
    let id = try!( get_id( request ) );
    response.send_answer( &create_post_answer( id, request ) );
    Ok( Halt )
}

fn trigger_impl( req: &Request ) -> AnswerResult {
    let events = req.events_manager();
    let mut db = try!( req.get_db_conn() );
    try!( events.maybe_start_something( &mut db ) );
    try!( events.maybe_end_something( &mut db ) );
    // стартуем после стопа еще раз, потому-что некоторые события по стопу создают новые
    try!( events.maybe_start_something( &mut db ) );

    let mut answer = Answer::new();
    answer.add_record( "trigger", &"activated".to_string() );
    Ok( answer )
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
    let result = req.events_manager().action_post( &mut db, id, req );
    try!( req.events_manager().maybe_start_something( &mut db ) );
    result
}

fn create_get_answer( event_id: Id, req: &Request ) -> AnswerResult {
    req.events_manager().user_creation_get( event_id, req )
}

fn create_post_answer( event_id: Id, req: &mut Request ) -> AnswerResult {
    let mut db = try!( req.get_db_conn() );

    let result = req.events_manager().user_creation_post( event_id, &mut db, req );
    try!( req.events_manager().maybe_start_something( &mut db ) );
    result
}

fn get_id( req: &Request ) -> Result<Id, NickelError> {
    let id = req.param( ID );
    ::std::str::from_str::<Id>( id )
        .ok_or( NickelError::new("Error parsing request path", NickelErrorKind::ErrorWithStatusCode(Status::NotFound)) )
}