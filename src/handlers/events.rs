use answer::{ AnswerResult, Answer };
use events::events_manager::EventsManager;
use types::{ Id };
use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use router_params::RouterParams;

static ID: &'static str = "id";

pub fn info_path() -> &'static str {
    "/events/info/:id"
}

pub fn info( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( request ) {
        Some( id ) => Response::with( (status::Ok, request.event_info( id )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn action_path() -> &'static str {
    "/events/action/:id"
}

pub fn trigger_path() -> &'static str {
    "/events/trigger"
}

pub fn action_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( request ) {
        Some( id ) => Response::with( (status::Ok, request.event_action_get( id )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn action_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( request ) {
        Some( id ) => Response::with( (status::Ok, action_post_answer( id, request )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn trigger( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, trigger_impl( request )) ) )
}

pub fn create_path() -> &'static str {
    "/events/create/:id"
}

pub fn create_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( request ) {
        Some( id ) => Response::with( (status::Ok, request.event_user_creation_get( id )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn create_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( request ) {
        Some( id ) => Response::with( (status::Ok, create_post_answer( id, request )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
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

fn get_id( req: &Request ) -> Option<Id> {
    let id = req.param( ID );
    FromStr::from_str( id ).ok()
}