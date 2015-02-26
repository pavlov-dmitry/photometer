/// изменение расписания группы

use super::{ GroupEvent, FullEventInfo, ScheduledEventInfo, EventState };
use super::group_voting::{ self, ChangeByVoting };
use super::events_collection;
use stuff::{ Stuff, Stuffable };
use database::{ Databaseable };
use authentication::Userable;
use db::groups::DbGroups;
use db::events::DbEvents;
use answer::{ Answer, AnswerResult };
use types::{ Id, CommonResult, EmptyResult };
use time::{ self, Timespec };
use std::time::duration::Duration;
use rustc_serialize::json;
use iron::prelude::*;
use get_param::GetParamable;

#[derive(Clone)]
pub struct ChangeTimetable;
pub const ID : Id = 5;

impl ChangeTimetable {
    pub fn new() -> ChangeTimetable {
        ChangeTimetable
    }
}

impl GroupEvent for ChangeTimetable {
    /// описание создания
    fn user_creating_get( &self, _req: &mut Request, _group_id: Id ) -> AnswerResult {
        let mut answer = Answer::new();
        answer.add_record( "timetable", &String::from_str( "lets_add_some_events" ) );
        Ok( answer )
    }
    /// применение создания
    fn user_creating_post( &self, req: &mut Request, group_id: Id ) -> Result<FullEventInfo, AnswerResult> {
        let user_id = req.user().id;
        let member_of_group = {
            let db = try!( req.stuff().get_current_db_conn() );
            try!( db.is_member( user_id, group_id ) )
        };
        let mut answer = Answer::new();
        // проверка на то что пользователь в группе
        if member_of_group {
            let days_for_voting = try!( req.get_param_uint( "days" ) );
            let ids_for_remove = try!( req.get_params_id( "remove" ) );

            let event_ids_for_add = try!( req.get_params_id( "add" ) );
            let names = try!( req.get_params( "name" ) ).clone();
            let start_times = try!( req.get_params_time( "start_time" ) );
            let end_times = try!( req.get_params_time( "end_time" ) );
            let params = try!( req.get_params( "params" ) ).clone();

            let self_start_time = time::get_time();
            let self_end_time = self_start_time + Duration::days( days_for_voting as i64 );

            try!( validate( 
                &event_ids_for_add, 
                &names, 
                &start_times, 
                &end_times,
                &params,
                &self_end_time 
            ));

            // просматриваем всех на отключение
            for &id in &ids_for_remove {
                let maybe_event_start_time = {
                    let db  = try!( req.stuff().get_current_db_conn() );
                    try!( db.event_start_time( id ) )
                };
                match maybe_event_start_time {
                    // проверяем что оно не началось или не начнётся за время голосования
                    Some( remove_start_time ) => {
                        if remove_start_time < self_end_time {
                            answer.add_error( "remove_id", "start_before_end_of_voting" );
                            return Err( Ok( answer ) );
                        }
                    }
                    // событие для отключения не найдено
                    None => {
                        answer.add_error( "remove_id", "not_found" );
                        return Err( Ok( answer ) );
                    }
                }
            }

            //создаём выключенными события которые должны будут добавиться
            let db = try!( req.stuff().get_current_db_conn() );
            let mut added_ids : Vec<Id> = Vec::new();
            for i in ( 0 .. event_ids_for_add.len() ) {
                // так как validate пройдено то событие точно существует, потому исползуем unwrap
                let id = event_ids_for_add[ i ];
                let event = events_collection::get_timetable_event( id ).unwrap();
                let event_data = event.from_timetable( group_id, &params[ i ] ).unwrap();
                let new_event_info = FullEventInfo {
                    id: id,
                    name: names[ i ].clone(),
                    start_time: start_times[ i ],
                    end_time: end_times[ i ],
                    data: event_data
                };
                let new_event_id = try!( db.add_disabled_event( &new_event_info ) );
                added_ids.push( new_event_id );
            }
            
            //добавляемся сами
            let data = Data {
                disable: ids_for_remove,
                enable: added_ids
            };
            let self_info = FullEventInfo {
                id: ID,
                name: String::from_str( "Изменения расписания" ),
                start_time: self_start_time,
                end_time: self_end_time,
                data: json::encode( &data ).unwrap()
            };
            // создаём голосование и хотим что бы за изменение расписания проголосовала хотя бы половина
            Ok( group_voting::new( group_id, 0.5, &self_info ) )
            
        } else {
            answer.add_error( "user", "not_in_group" );
            Err( Ok( answer ) )
        }
    }
}

fn validate( 
    events_ids: &Vec<Id>, 
    names: &Vec<String>, 
    begin: &Vec<Timespec>, 
    end: &Vec<Timespec>,
    params: &Vec<String>,
    self_end_time: &Timespec
) -> EmptyResult {
    // проверка на соответствие кол-ва
    if events_ids.len() != names.len() ||
        events_ids.len() != begin.len() ||
        events_ids.len() != end.len() ||
        events_ids.len() != params.len() {
        //FIXME: написать сто нить более понятное
        return Err( String::from_str( "invalid params count" ) );
    }
    // проверка диапазонов времен
    for (b, e) in begin.iter().zip( end.iter() ) {
        if e.sec < b.sec {
            return Err( String::from_str( "invalid time diapasons" ) );
        }
        if b.sec < self_end_time.sec {
            return Err( String::from_str( "start time must after end of voting" ) );
        }
    }
    // проверка что такие события существуют
    for (&id, p) in events_ids.iter().zip( params.iter() ) {
        match events_collection::get_timetable_event( id ) {
            Ok( event ) => {
                // проверка что параметры подходят под переданные события
                if event.is_valid_params( &p ) == false {
                    return Err( format!( "invalid params for event id={}", id ) );
                }
            }
            Err( _ ) => return Err( String::from_str( "invalid event id" ) )
        }
    } 
    Ok( () )
}

#[derive(RustcEncodable, RustcDecodable)]
struct Data {
    disable: Vec<Id>,
    enable: Vec<Id>
}

fn get_data( str_body: &str ) -> CommonResult<Data> {
    json::decode( str_body ).map_err( |e| format!( "ChangeTimetable event data decode error: {}", e ) )   
}
impl ChangeByVoting for ChangeTimetable {
    /// информация о событии
    fn get_info( &self, _req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let data = try!( get_data( &body.data ) );
        let mut answer = Answer::new();
        answer.add_record( "disable", &data.disable );
        answer.add_record( "enable", &data.enable );
        Ok( answer )
    }

    /// применить елси согласны
    fn apply( &self, stuff: &mut Stuff, _group_id: Id, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        for id in data.disable {
            try!( db.set_event_state( id, EventState::Disabled ) );
        }
        for id in data.enable {
            try!( db.set_event_state( id, EventState::NotStartedYet ) );
        }
        Ok( () )
    }
}