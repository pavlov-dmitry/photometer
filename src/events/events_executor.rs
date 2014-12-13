use database::{ DbConnection };
use std::sync::{ Arc };
use super::time_store::{ TimeStore };
use super::events_collection::{ EventsCollection };
use db::events::{ DbEvents };
use types::{ EmptyResult };
use time;

#[deriving(Clone)]
struct EventsExecutor {
    time_store: Arc<TimeStore>,
    events: Arc<EventsCollection>
}

impl EventsExecutor {
    /// исполняет события на старт
    fn maybe_start_something( &self, db: &mut DbConnection ) -> EmptyResult {
        let from_time = try!( self.time_store.get_stored_time() ).unwrap_or( time::get_time() );
        try!( self.time_store.remember_this_moment() );
        let to_time = try!( self.time_store.get_stored_time() ).unwrap();
        /*try!( db.starting_events( from_time, to_time, |event_info| {

        } ));*/

        Ok( () )
    }

    /// исполняет события на заверщение
    fn maybe_end_something() -> EmptyResult {
        Ok( () )
    }
}

fn midleware<'a>( time_store_file_path: Path ) -> EventsExecutor {
    EventsExecutor {
        time_store: Arc::new( TimeStore::new( time_store_file_path ) ),
        events: Arc::new( EventsCollection::new() )
    }
}

