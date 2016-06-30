use iron::prelude::*;
use router_params::RouterParams;
use std::str::FromStr;
use types::{ Id };
use events::{ EventId };

pub fn get_event_id( prm: &str, req: &Request ) -> Option<EventId> {
    match req.param( prm ) {
        Some( id ) => FromStr::from_str( id ).ok(),
        None => None
    }
}

pub fn get_id( prm: &str, req: &Request ) -> Option<Id> {
    match req.param( prm ) {
        Some( id ) => FromStr::from_str( id ).ok(),
        None => None
    }
}
