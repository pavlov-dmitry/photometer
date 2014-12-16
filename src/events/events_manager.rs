use database::{ DbConnection };
use std::sync::{ Arc };
use super::{ Event };
use super::time_store::{ TimeStore };
use super::events_collection::{ EventsCollection };
use db::events::{ DbEvents };
use types::{ EmptyResult, CommonResult, EventInfo };
use time;
use time::{ Timespec };
use super::publication::Publication;

#[deriving(Clone)]
struct EventsManager {
    time_store: Arc<TimeStore>,
    events: Arc<EventsCollection>
}

impl EventsManager {
    /// исполняет события на старт
    pub fn maybe_start_something( &self, db: &mut DbConnection ) -> EmptyResult {
        let (from, to) = try!( self.get_time_period() );
        let events = try!( db.starting_events( &from, &to ) );
        for event_info in events.iter() {
            let event = try!( self.events.get_event_by_id( event_info.id ) );
            try!( event.start( db, event_info ) );
        }
        Ok( () )
    }

    /// исполняет события на заверщение
    pub fn maybe_end_something( &self, db: &mut DbConnection ) -> EmptyResult {
        let (from, to) = try!( self.get_time_period() );
        let events = try!( db.ending_events( &from, &to ) );
        for event_info in events.iter() {
            let event = try!( self.events.get_event_by_id( event_info.id ) );
            try!( event.finish( db, event_info ) );
        }
        Ok( () )
    }

    fn get_time_period( &self ) -> CommonResult<( Timespec, Timespec )> {
        let from_time = try!( self.time_store.get_stored_time() ).unwrap_or( time::get_time() );
        try!( self.time_store.remember_this_moment() );
        let to_time = try!( self.time_store.get_stored_time() ).unwrap();
        Ok( ( from_time, to_time ) )
    }
}

fn midleware<'a>( time_store_file_path: Path ) -> EventsManager {
    let mut events_collection = EventsCollection::new();
    events_collection.add( Publication::new() );

    EventsManager {
        time_store: Arc::new( TimeStore::new( time_store_file_path ) ),
        events: Arc::new( events_collection )
    }
}

