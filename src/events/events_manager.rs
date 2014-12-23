use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use typemap::Assoc;
use plugin::Extensible;
use database::{ DbConnection };
use std::sync::{ Arc };
use super::{ Event, FullEventInfo, ScheduledEventInfo };
use super::time_store::{ TimeStore };
use super::events_collection::{ EventsCollection, EventPtr };
use db::events::{ DbEvents };
use db::timetable::DbTimetable;
use types::{ EmptyResult, CommonResult };
use time;
use time::{ Timespec };
use super::publication::Publication;
use super::group_creation::GroupCreation;
use answer::{ Answer, AnswerResult };
use types::{ Id };

#[deriving(Clone)]
struct EventsManager {
    time_store: Arc<TimeStore>,
    events: Arc<EventsCollection>
}

pub trait Eventsable {
    fn events_manager(&self) -> &EventsManager;
}

impl EventsManager {
    /// исполняет события на старт
    pub fn maybe_start_something( &self, db: &mut DbConnection ) -> EmptyResult {
        let (from, to) = try!( self.get_time_period() );
        try!( self.check_timetables( db, &from, &to ) );
        let events = try!( db.starting_events( &from, &to ) );
        for event_info in events.iter() {
            let event = try!( self.events.get_event( event_info.id ) );
            println!( "starting {}", event_info.id );
            try!( event.start( db, event_info ) );
        }
        Ok( () )
    }

    /// исполняет события на заверщение
    pub fn maybe_end_something( &self, db: &mut DbConnection ) -> EmptyResult {
        let events = try!( db.ending_events( &time::get_time() ) );
        for event_info in events.iter() {
            let event = try!( self.events.get_event( event_info.id ) );
            println!( "finishing {}", event_info.id );
            try!( event.finish( db, event_info ) );
            try!( db.mark_event_as_finished( event_info.scheduled_id ) );
        }
        Ok( () )
    }

    /// выдаёт информацию по событию
    pub fn info( &self, db: &mut DbConnection, scheduled_id: Id, req: &Request ) -> AnswerResult {
        self.if_has_event( db, scheduled_id, req, |event, event_info, db| {
            event.info_get( db, req, &event_info )
        })
    }

    /// инфа о действии
    pub fn action_get( &self, db: &mut DbConnection, scheduled_id: Id, req: &Request ) -> AnswerResult {
        self.if_has_event( db, scheduled_id, req, |event, event_info, db| {
            event.user_action_get( db, req, &event_info )
        })
    }

    /// применяем действие
    pub fn action_post( &self, db: &mut DbConnection, scheduled_id: Id, req: &Request ) -> AnswerResult {
        self.if_has_event( db, scheduled_id, req, |event, event_info, db| {
            let result = try!( event.user_action_post( db, req, &event_info ) );
            if try!( event.is_complete( db, &event_info ) ) {
                println!( "early finishing {}", event_info.id );
                try!( event.finish( db, &event_info ) );
                try!( db.mark_event_as_finished( event_info.scheduled_id ) );
            }
            Ok( result )
        })
    }

    /// проверяет расписания всех групп на новые события
    fn check_timetables( &self, db: &mut DbConnection, from: &Timespec, to: &Timespec ) -> EmptyResult {
        let timetable_events = try!( db.timetable_events( from, to ) );
        // создаём события
        let mut events : Vec<FullEventInfo> = Vec::new();
        for event_info in  timetable_events.iter() {
            let timetable_event = try!( self.events.get_timetable_event( event_info.event_id ) );
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
            println!( "add events from timetable: {}", events );
            try!( db.add_events( events.as_slice() ) );
        }
        Ok( () )
    }

    /// создание пользовательского события
    pub fn user_creation_get( &self, event_id: Id, req: &Request ) -> AnswerResult {
        let event = try!( self.events.get_user_event( event_id ) );
        event.user_creating_get( req )
    }

    pub fn user_creation_post( &self, event_id: Id, db: &mut DbConnection, req: &Request ) -> AnswerResult {
        let event = try!( self.events.get_user_event( event_id ) );
        match event.user_creating_post( db, req ) {
            Ok( event ) => {
                println!( "event created: {}", event.id );
                try!( db.add_events( &[ event ] ) );

                let mut answer = Answer::new();
                answer.add_record( "event", &"created".to_string() );
                Ok( answer )
            }
            Err( answer_result ) => answer_result
        }
    }

    // db приходится передавать по цепочке, иначе содается вторая mut ссылка в замыкании, что естественно делать нельзя
    fn if_has_event( &self, db: &mut DbConnection, scheduled_id: Id, _req: &Request, 
        do_this: |&EventPtr, ScheduledEventInfo, &mut DbConnection| -> AnswerResult 
    ) -> AnswerResult {
        match try!( db.event_info( scheduled_id ) ) {
            Some( event_info ) => {
                let event = try!( self.events.get_event( event_info.id ) ); 
                do_this( event, event_info, db )
            },
            None => {
                let mut answer = Answer::new();
                answer.add_error( "event", "not_found" );
                Ok( answer )
            }
        } 
    }

    fn get_time_period( &self ) -> CommonResult<( Timespec, Timespec )> {
        let from_time_tmp = try!( self.time_store.get_stored_time() ).unwrap_or( Timespec::new( 0, 0 ) );
        let from_time = Timespec::new( from_time_tmp.sec + 1, from_time_tmp.nsec );
        try!( self.time_store.remember_this_moment() );
        let to_time = try!( self.time_store.get_stored_time() ).unwrap();
        println!( "check_period from: {}   to: {}", from_time.sec, to_time.sec );
        Ok( ( from_time, to_time ) )
    }
}

pub fn middleware( time_store_file_path: &String ) -> EventsManager {
    let mut events_collection = EventsCollection::new();

    let publication = Publication::new();
    events_collection.add( publication.clone() );
    events_collection.add_timetable( publication.id(), publication.clone() );

    let group_creation = GroupCreation::new();
    events_collection.add( group_creation.clone() );
    events_collection.add_user_event( group_creation.id(), group_creation.clone() );

    EventsManager {
        time_store: Arc::new( TimeStore::new( Path::new( time_store_file_path.as_slice() ) ) ),
        events: Arc::new( events_collection )
    }
}

impl Assoc<EventsManager> for EventsManager {}

impl Middleware for EventsManager {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.extensions_mut().insert::<EventsManager, EventsManager>( self.clone() );
        Ok( Continue )
    } 
}

impl<'a, 'b> Eventsable for Request<'a, 'b> {
    fn events_manager(&self) -> &EventsManager {
        self.extensions().get::<EventsManager, EventsManager>().unwrap()
    }
}