use answer::{ AnswerResult, AnswerResponse };
use events::events_manager::{ EventsManagerRequest, EventsManagerStuff };
use events::{ EventId };
use stuff::Stuffable;
use types::{ Id };
use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use router_params::RouterParams;
use types::EmptyResult;

static ID: &'static str = "id";
static GROUP_ID: &'static str = "group_id";

pub fn path() -> &'static str {
    "/event/:id"
}

pub fn info( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => {
            let answer = AnswerResponse( request.event_info( id ) );
            Response::with( answer )
        },

        None => Response::with( status::NotFound )
    };
    Ok( response )
}

// TODO: Убрать совсем, если не нужно
// pub fn action_get( request: &mut Request ) -> IronResult<Response> {
//     let response = match get_id( ID, request ) {
//         Some( id ) => {
//             let answer = AnswerResponse( request.event_action_get( id ) );
//             Response::with( answer )
//         },

//         None => Response::with( status::NotFound )
//     };
//     Ok( response )
// }

pub fn action_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => {
            let answer = AnswerResponse( action_post_answer( id, request ) );
            Response::with( answer )
        },

        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn create_path() -> &'static str {
    "/events/create/:id"
}

pub fn create_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_event_id( ID, request ) {
        Some( id ) => {
            let answer = AnswerResponse( request.event_user_creation_get( id ) );
            Response::with( answer )
        },

        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn create_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_event_id( ID, request ) {
        Some( id ) => {
            let answer = AnswerResponse( create_post_answer( id, request ) );
            Response::with( answer )
        },
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

fn trigger_events( req: &mut Request ) -> EmptyResult {
    let started = try!( req.stuff().maybe_start_some_events() );
    let ended = try!( req.stuff().maybe_end_some_events() );
    if started || ended {
        try!( trigger_events( req ) );
    }
    Ok( () )
}

fn action_post_answer( id: Id, req: &mut Request ) -> AnswerResult {
    let result = req.event_action_post( id );
    try!( trigger_events( req ) );
    result
}

fn create_post_answer( event_id: EventId, req: &mut Request ) -> AnswerResult {
    let result = req.event_user_creation_post( event_id );
    try!( trigger_events( req ) );
    result
}

pub fn group_create_path() -> &'static str {
    "/events/group/:group_id/create/:id"
}

pub fn group_create_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_group_and_event_id( request ) {
        Some( (group_id, id) ) => {
            let answer = AnswerResponse( request.event_group_creation_get( group_id, id ) );
            Response::with( answer )
        },

        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn group_create_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_group_and_event_id( request ) {
        Some( (group_id, id) ) => {
            let answer = AnswerResponse(
                get_group_greation_post_answer( request, group_id, id )
            );
            Response::with( answer )
        },

        None => Response::with( status::NotFound )
    };
    Ok( response )
}

fn get_group_greation_post_answer( req: &mut Request, group_id: Id, event_id: EventId ) -> AnswerResult {
    let result = req.event_group_creation_post( group_id, event_id );
    try!( trigger_events( req ) );
    result
}


fn get_event_id( prm: &str, req: &Request ) -> Option<EventId> {
    match req.param( prm ) {
        Some( id ) => FromStr::from_str( id ).ok(),
        None => None
    }
}

fn get_id( prm: &str, req: &Request ) -> Option<Id> {
    match req.param( prm ) {
        Some( id ) => FromStr::from_str( id ).ok(),
        None => None
    }
}

fn get_group_and_event_id( request: &mut Request ) -> Option<(Id, EventId)> {
    get_id( GROUP_ID, request )
        .and_then( |group_id|
            get_event_id( ID, request )
                .map( |id| ( group_id, id ) )
        )
}
