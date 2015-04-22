use database::{ Databaseable };
use stuff::{ Stuff, Stuffable };
use super::{ Event, ScheduledEventInfo, EventState, FullEventInfo };
use super::events_collection;
use super::events_collection::{ EventPtr };
use db::events::{ DbEvents };
use types::{ EmptyResult };
use time;
use answer::{ Answer, AnswerResult };
use types::{ Id };
use iron::prelude::*;
use authentication::Userable;
use db::groups::DbGroups;
use answer_types::{ OkInfo, FieldErrorInfo };

pub trait EventsManagerStuff {
    fn maybe_start_some_events(&mut self) -> EmptyResult;
    fn maybe_end_some_events(&mut self) -> EmptyResult;
}

pub trait EventsManagerRequest {
    fn event_info( &mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_action_get( &mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_action_post( &mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_user_creation_get(&mut self, event_id: Id ) -> AnswerResult;
    fn event_user_creation_post(&mut self, event_id: Id ) -> AnswerResult;
    fn event_group_creation_get(&mut self, group_id: Id, event_id: Id ) -> AnswerResult;
    fn event_group_creation_post(&mut self, group_id: Id, event_id: Id ) -> AnswerResult;
}

impl EventsManagerStuff for Stuff {

    /// исполняет события на старт
    fn maybe_start_some_events( &mut self ) -> EmptyResult {
        let events = {
            let db = try!( self.get_current_db_conn() );
            try!( db.starting_events( &time::get_time() ) )
        };
        for event_info in events.iter() {
            let event = try!( events_collection::get_event( event_info.id ) );
            info!( "starting '{}':{}", event_info.name, event_info.id );
            try!( event.start( self, event_info ) );
            try!( self.set_event_state( event_info.scheduled_id, EventState::Active ) );
        }
        Ok( () )
    }

    /// исполняет события на заверщение
    fn maybe_end_some_events( &mut self ) -> EmptyResult {
        let events = {
            let db = try!( self.get_current_db_conn() );
            try!( db.ending_events( &time::get_time() ) )
        };
        for event_info in events.iter() {
            let event = try!( events_collection::get_event( event_info.id ) );
            info!( "finishing '{}':{}", event_info.name, event_info.id );
            try!( self.finish_him( event, event_info ) );
        }
        Ok( () )
    }
}


impl<'a, 'b> EventsManagerRequest for Request<'a, 'b> {
    /// выдаёт информацию по событию
    fn event_info( &mut self, scheduled_id: Id ) -> AnswerResult {
        self.if_has_event( scheduled_id, |event, event_info, req| {
            event.info_get( req, &event_info )
        })
    }

    /// инфа о действии
    fn event_action_get( &mut self, scheduled_id: Id ) -> AnswerResult {
        self.if_has_event( scheduled_id, |event, event_info, req| {
            event.user_action_get( req, &event_info )
        })
    }

    /// применяем действие
    fn event_action_post( &mut self, scheduled_id: Id ) -> AnswerResult {
        self.if_has_event( scheduled_id, |event, event_info, req| {
            let result = try!( event.user_action_post( req, &event_info ) );
            let stuff = req.stuff();
            if try!( event.is_complete( stuff, &event_info ) ) {
                info!( "early finishing '{}':{}", event_info.name, event_info.id );
                try!( stuff.finish_him( event, &event_info ) );
            }
            Ok( result )
        })
    }

    /// создание пользовательского события
    fn event_user_creation_get( &mut self, event_id: Id ) -> AnswerResult {
        let event = try!( events_collection::get_user_event( event_id ) );
        event.user_creating_get( self )
    }

    fn event_user_creation_post( &mut self, event_id: Id ) -> AnswerResult {
        let event = try!( events_collection::get_user_event( event_id ) );
        match event.user_creating_post( self ) {
            Ok( event ) => apply_event( event, self ),
            Err( answer_result ) => answer_result
        }
    }

    fn event_group_creation_get(&mut self, group_id: Id, event_id: Id ) -> AnswerResult {
        let event = try!( events_collection::get_group_event( event_id ) );
        event.user_creating_get( self, group_id )
    }

    fn event_group_creation_post(&mut self, group_id: Id, event_id: Id ) -> AnswerResult {
        let event = try!( events_collection::get_group_event( event_id ) );

        let user_id = self.user().id;
        let member_of_group = {
            let db = try!( self.stuff().get_current_db_conn() );
            try!( db.is_member( user_id, group_id ) )
        };

        // проверка на то что пользователь в группе
        if member_of_group {
            match event.user_creating_post( self, group_id ) {
                Ok( event ) => apply_event( event, self ),
                Err( answer_result ) => answer_result
            }
        } else {
            let answer = Answer::bad( FieldErrorInfo::new( "user", "not_in_group" ) );
            Ok( answer )
        }
    }
}

fn apply_event( event: FullEventInfo, req: &mut Request ) -> AnswerResult {
    info!( "event created: '{}':{}", event.name, event.id );
    let db = try!( req.stuff().get_current_db_conn() );
    try!( db.add_events( &[ event ] ) );

    let answer = Answer::good( OkInfo::new( "created" ) );
    Ok( answer )
}

trait EventsManagerStuffPrivate {
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult;
    fn finish_him( &mut self, event: EventPtr, info: &ScheduledEventInfo ) -> EmptyResult;
}

trait EventsManagerPrivate {
    fn if_has_event<F: Fn(EventPtr, ScheduledEventInfo, &mut Request) -> AnswerResult>(
        &mut self, scheduled_id: Id, do_this: F
    ) -> AnswerResult;
}

impl EventsManagerStuffPrivate for Stuff {
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult {
        let db = try!( self.get_current_db_conn() );
        db.set_event_state( scheduled_id, state )
    }
    fn finish_him( &mut self, event: EventPtr, info: &ScheduledEventInfo ) -> EmptyResult {
        try!( event.finish( self, info ) );
        let db = try!( self.get_current_db_conn() );
        try!( db.set_event_state( info.scheduled_id, EventState::Finished ) );
        Ok( () )
    }
}

impl<'a, 'b> EventsManagerPrivate for Request<'a, 'b> {
    // db приходится передавать по цепочке, иначе содается вторая mut ссылка в замыкании, что естественно делать нельзя
    fn if_has_event<F: Fn(EventPtr, ScheduledEventInfo, &mut Request) -> AnswerResult>(
        &mut self,  scheduled_id: Id, do_this: F
    ) -> AnswerResult {
        let event_info = {
            let db = try!( self.stuff().get_current_db_conn() );
            try!( db.event_info( scheduled_id ) )
        };
        match event_info {
            Some( event_info ) => {
                let event = try!( events_collection::get_event( event_info.id ) );
                do_this( event, event_info, self )
            },
            None => {
                let answer = Answer::bad( FieldErrorInfo::new( "event", "not_found" ) );
                Ok( answer )
            }
        }
    }
}
