use super::{ Event, CreateFromTimetable, UserEvent };
use std::collections::HashMap;
use std::boxed::{ Box };
use types::{ Id, CommonResult };

pub type EventPtr = Box<Event + Send + Sync>;
pub type TimetableEventPtr = Box<CreateFromTimetable + Send + Sync>;
pub type UserEventPtr = Box<UserEvent + Send + Sync>;

pub struct EventsCollection {
    events: HashMap<Id, EventPtr>,
    timetable: HashMap<Id, TimetableEventPtr>,
    user_events: HashMap<Id, UserEventPtr>
}

impl EventsCollection {
    pub fn new() -> EventsCollection {
        EventsCollection {
            events: HashMap::new(),
            timetable: HashMap::new(),
            user_events: HashMap::new()
        }
    }

    pub fn add<E: Event + Send + Sync>( &mut self, event: E ) {
        self.events.insert( event.id(), box event );
    }

    pub fn add_timetable<E: CreateFromTimetable + Send + Sync>( &mut self, id: Id, event: E ) {
        self.timetable.insert( id, box event );
    }

    pub fn add_user_event<E: UserEvent + Send + Sync>( &mut self, id: Id, event: E ) {
        self.user_events.insert( id, box event );
    }

    pub fn get_event( &self, id: Id ) -> CommonResult<&EventPtr> {
        self.events.get( &id ).ok_or( format!( "EventsCollection, event with id={} not found.", id ) )
    }

    pub fn get_timetable_event( &self, id: Id ) -> CommonResult<&TimetableEventPtr> {
        self.timetable.get( &id ).ok_or( format!( "EventsCollection, timetable event with id={} not found.", id ) )
    }

    pub fn get_user_event( &self, id: Id ) -> CommonResult<&UserEventPtr> {
        self.user_events.get( &id ).ok_or( format!( "EventsCollection, user creating event with id={} not found.", id ) )
    }
}