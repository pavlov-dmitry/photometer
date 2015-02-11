use iron::typemap::Key;
use database::{ Databaseable };
use stuff::{ Stuff, StuffInstallable, Stuffable };
use std::sync::{ Arc };
use super::{ Event, FullEventInfo, ScheduledEventInfo, EventState };
use super::events_collection;
use super::events_collection::{ EventPtr };
use db::events::{ DbEvents };
use db::timetable::DbTimetable;
use types::{ EmptyResult, CommonResult };
use time;
use time::{ Timespec };
use answer::{ Answer, AnswerResult };
use types::{ Id };
use super::time_store::TimeStore;
use iron::prelude::*;

#[derive(Clone)]
struct EventsManagerBody {
    time_store: Arc<TimeStore>
}

pub trait EventsManagerStuff {
    fn maybe_start_some_events(&mut self) -> EmptyResult;
    fn maybe_end_some_events(&mut self) -> EmptyResult;
}

pub trait EventsManagerRequest {
    fn event_info( &mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_action_get( &mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_action_post( &mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_user_creation_get(&mut self, scheduled_id: Id ) -> AnswerResult;
    fn event_user_creation_post(&mut self, scheduled_id: Id ) -> AnswerResult;
}

impl EventsManagerStuff for Stuff {
    
    /// исполняет события на старт
    fn maybe_start_some_events( &mut self ) -> EmptyResult {
        let (from, to) = try!( self.get_time_period() );
        try!( self.check_timetables( &from, &to ) );
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


impl<'a> EventsManagerRequest for Request<'a> {
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
            Ok( event ) => {
                info!( "event created: '{}':{}", event.name, event.id );
                let db = try!( self.stuff().get_current_db_conn() );
                try!( db.add_events( &[ event ] ) );

                let mut answer = Answer::new();
                answer.add_record( "event", &"created".to_string() );
                Ok( answer )
            }
            Err( answer_result ) => answer_result
        }
    }
}

trait EventsManagerStuffPrivate {
    fn get_body( &self ) -> &EventsManagerBody;
    fn get_time_period( &self ) -> CommonResult<( Timespec, Timespec )>;
    fn set_event_state( &mut self, scheduled_id: Id, state: EventState ) -> EmptyResult;
    fn check_timetables( &mut self, from: &Timespec, to: &Timespec ) -> EmptyResult;
    fn finish_him( &mut self, event: EventPtr, info: &ScheduledEventInfo ) -> EmptyResult;
}

trait EventsManagerPrivate {
    fn if_has_event<F: Fn(EventPtr, ScheduledEventInfo, &mut Request) -> AnswerResult>( 
        &mut self, scheduled_id: Id, do_this: F
    ) -> AnswerResult;
}

impl EventsManagerStuffPrivate for Stuff {
    fn get_body( &self ) -> &EventsManagerBody {
        self.extensions.get::<EventsManagerBody>().unwrap()  
    }
    fn get_time_period( &self ) -> CommonResult<( Timespec, Timespec )> {
        let body = self.get_body();
        let from_time = try!( body.time_store.get_stored_time() ).unwrap_or( Timespec::new( 0, 0 ) );
        try!( body.time_store.remember_this_moment() );
        let to_time = try!( body.time_store.get_stored_time() ).unwrap();
        debug!( "check_period from: {}  to: {}", from_time.sec, to_time.sec );
        Ok( ( from_time, to_time ) )
    }
    /// проверяет расписания всех групп на новые события
    fn check_timetables( &mut self, from: &Timespec, to: &Timespec ) -> EmptyResult {
        let timetable_events = {
            let db = try!( self.get_current_db_conn() );
            try!( db.timetable_events( from, to ) )
        };
        // создаём события
        let mut events : Vec<FullEventInfo> = Vec::new();
        for event_info in  timetable_events.iter() {
            let timetable_event = try!( events_collection::get_timetable_event( event_info.event_id ) );
            let data = timetable_event.from_timetable( event_info.group_id, &event_info.params );
            let data = try!( data.ok_or( 
                format!( "Creating event id = {} group_id = {} from timetable failed", 
                    event_info.event_id, 
                    event_info.group_id 
                ) 
            ));
            events.push( FullEventInfo {
                id: event_info.event_id,
                name: event_info.event_name.clone(),
                start_time: event_info.start_time,
                end_time: event_info.end_time,
                data: data
            });
        }
        // елси хоть что нить создали, то записываем их в запланированные события
        if events.is_empty() == false {
            debug!( "add events from timetable: {:?}", events );
            let db = try!( self.get_current_db_conn() );
            try!( db.add_events( events.as_slice() ) );
        }
        Ok( () )
    }
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

impl<'a> EventsManagerPrivate for Request<'a> {
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
                let mut answer = Answer::new();
                answer.add_error( "event", "not_found" );
                Ok( answer )
            }
        } 
    }
}

pub fn body( time_store_file_path: &String ) -> EventsManagerBody {
    EventsManagerBody {
        time_store: Arc::new( TimeStore::new( Path::new( time_store_file_path ) ) )
    }
}

impl Key for EventsManagerBody { type Value = EventsManagerBody; }

impl StuffInstallable for EventsManagerBody {
    fn install_to( &self, stuff: &mut Stuff ) {
        stuff.extensions.insert::<EventsManagerBody>( self.clone() );   
    }
}