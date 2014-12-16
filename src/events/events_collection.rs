use super::{ Event };
use std::collections::HashMap;
use std::boxed::{ Box };
use types::{ Id, CommonResult };

pub type EventPtr = Box<Event + Send + Sync>;

pub struct EventsCollection {
	events: HashMap<Id, EventPtr>,
}

impl EventsCollection {
	pub fn new() -> EventsCollection {
		EventsCollection {
			events: HashMap::new(),
		}
	}

	pub fn add<E: Event + Send + Sync>( &mut self, event: E ) {
		self.events.insert( event.id(), box event );
	}

	pub fn get_event_by_id( &self, id: Id ) -> CommonResult<&EventPtr> {
		self.events.get( &id ).ok_or( format!( "EventsCollection, event with id={} not found.", id ) )
	}
}