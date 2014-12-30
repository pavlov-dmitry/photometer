use super::{ Event, CreateFromTimetable, UserEvent };
use std::boxed::{ Box };
use types::{ Id, CommonResult };
use super::publication;
use super::publication::Publication;
use super::group_creation;
use super::group_creation::GroupCreation;
use super::late_publication;
use super::late_publication::LatePublication;


pub type EventPtr = Box<Event + Send + Sync>;
pub type TimetableEventPtr = Box<CreateFromTimetable + Send + Sync>;
pub type UserEventPtr = Box<UserEvent + Send + Sync>;

pub fn get_event( id: Id ) -> CommonResult<EventPtr> {
    match id {
        publication::ID => Ok( box Publication::new() as EventPtr ),
        group_creation::ID => Ok( box GroupCreation::new() as EventPtr ),
        late_publication::ID => Ok( box LatePublication::new() as EventPtr ),
        _ => Err( format!( "EventsCollection, event with id={} not found.", id ) )
    }
}

pub fn get_timetable_event( id: Id ) -> CommonResult<TimetableEventPtr> {
    match id {
        publication::ID => Ok( box Publication::new() as TimetableEventPtr ),
        _ => Err( format!( "EventsCollection, timetable event with id={} not found.", id ) )
    }
}

pub fn get_user_event( id: Id ) -> CommonResult<UserEventPtr> {
    match id {
        group_creation::ID => Ok( box GroupCreation::new() as UserEventPtr ),
        _ => Err( format!( "EventsCollection, user creating event with id={} not found.", id ) )
    }
}
