use super::{
    Event,
    CreateFromTimetable,
    UserCreatedEvent,
    GroupCreatedEvent,
};
use std::boxed::{ Box };
use types::{ Id, CommonResult, common_error };
use super::publication::{ self, Publication };
use super::group_creation::{ self, GroupCreation };
use super::late_publication::{ self, LatePublication };
use super::group_voting::{ self, GroupVoting, ChangeByVoting };
use super::change_timetable::{ self, ChangeTimetable };


pub type EventPtr = Box<Event + Send + Sync>;
pub type TimetableEventPtr = Box<CreateFromTimetable + Send + Sync>;
pub type UserCreatedEventPtr = Box<UserCreatedEvent + Send + Sync>;
pub type ChangeByVotingPtr = Box<ChangeByVoting + Send + Sync>;
pub type GroupCreatedEventPtr = Box<GroupCreatedEvent + Send + Sync>;

pub fn get_event( id: Id ) -> CommonResult<EventPtr> {
    match id {
        publication::ID => Ok( Box::new( Publication::new() ) as EventPtr ),
        group_creation::ID => Ok( Box::new( GroupCreation::new() ) as EventPtr ),
        late_publication::ID => Ok( Box::new( LatePublication::new() ) as EventPtr ),
        group_voting::ID => Ok( Box::new( GroupVoting::new() ) as EventPtr ),
        _ => common_error( format!( "EventsCollection, event with id={} not found.", id ) )
    }
}

pub fn get_timetable_event( id: Id ) -> CommonResult<TimetableEventPtr> {
    match id {
        publication::ID => Ok( Box::new( Publication::new() ) as TimetableEventPtr ),
        _ => common_error( format!( "EventsCollection, timetable event with id={} not found.", id ) )
    }
}

pub fn get_user_created_event( id: Id ) -> CommonResult<UserCreatedEventPtr> {
    match id {
        group_creation::ID => Ok( Box::new( GroupCreation::new() ) as UserCreatedEventPtr ),
        _ => common_error( format!( "EventsCollection, user creating event with id={} not found.", id ) )
    }
}

pub fn get_change_by_voting( id: Id ) -> CommonResult<ChangeByVotingPtr> {
    match id {
        change_timetable::ID => Ok( Box::new( ChangeTimetable::new() ) as ChangeByVotingPtr ),
        _ => common_error( format!( "EventsCollection, change by voting event with id={} not found.", id ) )
    }
}

pub fn get_group_created_event( id: Id ) -> CommonResult<GroupCreatedEventPtr> {
    match id {
        change_timetable::ID => Ok( Box::new( ChangeTimetable::new() ) as GroupCreatedEventPtr ),
        _ => common_error( format!( "EventsCollection, group user creating event with id={} not found.", id ) )
    }
}
