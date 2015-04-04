/// изменение расписания группы

use super::{ GroupEvent, FullEventInfo, ScheduledEventInfo, EventState };
use super::group_voting::{ self, ChangeByVoting };
use super::events_collection;
use stuff::{ Stuff, Stuffable };
use database::{ Databaseable };
use db::events::DbEvents;
use answer::{ Answer, AnswerResult };
use types::{ Id, CommonResult, EmptyResult };
use time::{ self, Timespec };
use std::time::duration::Duration;
use rustc_serialize::json;
use iron::prelude::*;
use get_body::GetBody;
use parse_utils;
use err_msg;
use answer_types::{ OkInfo, FieldErrorInfo };

#[derive(Clone)]
pub struct ChangeTimetable;
pub const ID : Id = 5;

impl ChangeTimetable {
    pub fn new() -> ChangeTimetable {
        ChangeTimetable
    }
}


#[derive(Clone, RustcDecodable)]
struct TimetableDiffInfoStr {
    days_for_voting: u32,
    remove: Vec<Id>,
    add: Vec<AddEventInfoStr>
}

#[derive(Clone, RustcDecodable)]
struct AddEventInfoStr {
    id: Id,
    name: String,
    start_time: String,
    end_time: String,
    params: String
}

struct TimetableDiffInfo {
    days_for_voting: u32,
    remove: Vec<Id>,
    add: Vec<AddEventInfo>
}

struct AddEventInfo {
    id: Id,
    name: String,
    start_time: Timespec,
    end_time: Timespec,
    params: String
}

impl GroupEvent for ChangeTimetable {
    /// описание создания
    fn user_creating_get( &self, _req: &mut Request, _group_id: Id ) -> AnswerResult {
        let answer = Answer::good( OkInfo::new( "lets_add_some_events" ) );
        Ok( answer )
    }
    /// применение создания
    fn user_creating_post( &self, req: &mut Request, group_id: Id ) -> Result<FullEventInfo, AnswerResult> {
        let diff_info_str = try!( req.get_body::<TimetableDiffInfoStr>() );

        // парсим время
        let diff_info = try!( parse_times( diff_info_str ) );

        let self_start_time = time::get_time();
        let self_end_time = self_start_time + Duration::days( diff_info.days_for_voting as i64 );

        // проверка корректности добаляемых событий
        try!( check_for_add( &diff_info.add, &self_end_time ) );

        // проверка всех на отключение
        try!( check_for_remove( &diff_info.remove, &self_end_time, &mut req.stuff() ) );

        //создаём выключенными события которые должны будут добавиться
        let db = try!( req.stuff().get_current_db_conn() );
        let mut added_ids : Vec<Id> = Vec::new();
        for add in &diff_info.add {
            // так как check_for_add пройдено то событие точно существует, потому исползуем unwrap
            let event = events_collection::get_timetable_event( add.id ).unwrap();
            let event_data = event.from_timetable( group_id, &add.params ).unwrap();
            let new_event_info = FullEventInfo {
                id: add.id,
                name: add.name.clone(),
                start_time: add.start_time,
                end_time: add.end_time,
                data: event_data
            };
            let new_event_id = try!( db.add_disabled_event( &new_event_info ) );
            added_ids.push( new_event_id );
        }

        //добавляемся сами
        let data = Data {
            disable: diff_info.remove,
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
    }
}

fn parse_times( diff_str: TimetableDiffInfoStr ) -> Result<TimetableDiffInfo, AnswerResult> {
    let mut parsed_time = Vec::new();
    parsed_time.reserve( diff_str.add.len() );
    for add in diff_str.add {
        let parsed_start = match parse_utils::parse_timespec( &add.start_time ) {
            Ok( tm ) => tm,
            Err( _ ) => return Err( Err( err_msg::parsing_error_param( "start_time" ) ) )
        };
        let parsed_end = match parse_utils::parse_timespec( &add.end_time ) {
            Ok( tm ) => tm,
            Err( _ ) => return Err( Err( err_msg::parsing_error_param( "end_time" ) ) )
        };
        parsed_time.push( AddEventInfo {
            id: add.id,
            name: add.name,
            start_time: parsed_start,
            end_time: parsed_end,
            params: add.params
        });
    }

    Ok( TimetableDiffInfo {
        days_for_voting: diff_str.days_for_voting,
        remove: diff_str.remove,
        add: parsed_time
    })
}

fn check_for_add( for_add: &Vec<AddEventInfo>, self_end_time: &Timespec ) -> EmptyResult {
    // проверка диапазонов времен
    for add in for_add {
        if add.end_time.sec < add.start_time.sec {
            return Err( String::from_str( "invalid time diapasons" ) );
        }
        if add.start_time.sec < self_end_time.sec {
            return Err( String::from_str( "start time must after end of voting" ) );
        }
        // проверка что такие события существуют
        match events_collection::get_timetable_event( add.id ) {
            Ok( event ) => {
                // проверка что параметры подходят под переданные события
                if event.is_valid_params( &add.params ) == false {
                    return Err( format!( "invalid params for event id={}", add.id ) );
                }
            }
            Err( _ ) => return Err( String::from_str( "invalid event id" ) )
        }
    }
    Ok( () )
}

fn check_for_remove( for_remove: &Vec<Id>, self_end_time: &Timespec, stuff: &mut Stuff ) -> Result<(), AnswerResult> {
    for &id in for_remove {
        let maybe_event_start_time = {
            let db  = try!( stuff.get_current_db_conn() );
            try!( db.event_start_time( id ) )
        };
        match maybe_event_start_time {
            // проверяем что оно не началось или не начнётся за время голосования
            Some( remove_start_time ) => {
                if remove_start_time < *self_end_time {
                    let answer = Answer::bad( FieldErrorInfo::new(
                        "remove_id",
                        "start_before_end_of_voting" ) );
                    return Err( Ok( answer ) );
                }
            }
            // событие для отключения не найдено
            None => {
                let answer = Answer::bad( FieldErrorInfo::new( "remove_id", "not_found" ) );
                return Err( Ok( answer ) );
            }
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
    fn get_info( &self, _req: &mut Request, body: &ScheduledEventInfo ) -> CommonResult<String> {
        let data = try!( get_data( &body.data ) );
        let info_as_string = json::encode( &data ).unwrap();
        Ok( info_as_string )
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
