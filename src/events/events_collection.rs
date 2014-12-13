use super::{ Event };
use std::boxed::{ Box };

pub struct EventsCollection {
	events: Vec<Box<Event + Send + Sync>>,
}

impl EventsCollection {
	pub fn new() -> EventsCollection {
		EventsCollection {
			events: Vec::new(),
		}
	}

	pub fn add<E: Event + Send + Sync>( &mut self, event: E ) {
		self.events.push( box event );
	}
}