use answer::{ AnswerResult };
use events::events_manager::{ EventsManagerRequest, EventsManagerStuff };
use stuff::Stuffable;
use types::{ Id };
use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use router_params::RouterParams;

static ID: &'static str = "id";
static GROUP_ID: &'static str = "group_id";

pub fn info_path() -> &'static str {
    "/events/info/:id"
}

pub fn info( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => Response::with( (status::Ok, request.event_info( id )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn action_path() -> &'static str {
    "/events/action/:id"
}

pub fn action_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => Response::with( (status::Ok, request.event_action_get( id )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn action_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => Response::with( (status::Ok, action_post_answer( id, request )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn create_path() -> &'static str {
    "/events/create/:id"
}

pub fn create_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => Response::with( (status::Ok, request.event_user_creation_get( id )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn create_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_id( ID, request ) {
        Some( id ) => Response::with( (status::Ok, create_post_answer( id, request )) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

fn action_post_answer( id: Id, req: &mut Request ) -> AnswerResult {
    let result = req.event_action_post( id );
    try!( req.stuff().maybe_start_some_events() );
    result
}

fn create_post_answer( event_id: Id, req: &mut Request ) -> AnswerResult {
    let result = req.event_user_creation_get( event_id );
    try!( req.stuff().maybe_start_some_events() );
    result
}

pub fn group_create_path() -> &'static str {
    "/events/group/:group_id/create/:id"
}

pub fn group_create_get( request: &mut Request ) -> IronResult<Response> {
    let response = match get_group_and_event_id( request ) {
        Some( (group_id, id) ) => Response::with( (
            status::Ok, 
            request.event_group_creation_get( group_id, id ) 
        ) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}

pub fn group_create_post( request: &mut Request ) -> IronResult<Response> {
    let response = match get_group_and_event_id( request ) {
        Some( (group_id, id) ) => Response::with( (
            status::Ok, 
            request.event_group_creation_post( group_id, id )
        ) ),
        None => Response::with( status::NotFound )
    };
    Ok( response )
}


fn get_id( prm: &str, req: &Request ) -> Option<Id> {
    let id = req.param( prm );
    FromStr::from_str( id ).ok()
}

fn get_group_and_event_id( request: &mut Request ) -> Option<(Id, Id)> {
    get_id( GROUP_ID, request )
        .and_then( |group_id| 
            get_id( ID, request )
                .map( |id| ( group_id, id ) )
        )
}