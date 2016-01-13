/// изменение расписания группы

use super::{
    GroupCreatedEvent,
    FullEventInfo,
    ScheduledEventInfo,
    EventState,
    Description
};
use super::group_voting::{ self, ChangeByVoting };
use super::events_collection;
use stuff::{ Stuff, Stuffable };
use database::{ Databaseable };
use db::events::DbEvents;
use db::groups::DbGroups;
use answer::{ Answer, AnswerResult };
use types::{ Id, CommonResult, EmptyResult, CommonError };
use time::{ self, Timespec };
use rustc_serialize::json;
use iron::prelude::*;
use get_body::GetBody;
use parse_utils;
use std::convert::From;

#[derive(Clone)]
pub struct ChangeTimetable;
pub const ID : Id = 5;

const DAYS_FOR_VOTE : i64 = 5;
const MAX_NAME_LENGTH: usize = 64;
const MAX_PARAMS_LENGTH: usize = 2048;
const MAX_DESCRIPTION_LENGTH: usize = 2048;

impl ChangeTimetable {
    pub fn new() -> ChangeTimetable {
        ChangeTimetable
    }
}


#[derive(Clone, RustcDecodable)]
struct TimetableDiffInfoStr {
    description: String,
    remove: Vec<Id>,
    add: Vec<AddEventInfoStr>
}

#[derive(Clone, RustcDecodable)]
struct AddEventInfoStr {
    event_id: Id,
    name: String,
    time: String,
    params: String
}

struct TimetableDiffInfo {
    description: String,
    remove: Vec<Id>,
    add: Vec<AddEventInfo>
}

struct AddEventInfo {
    event_id: Id,
    name: String,
    time: Timespec,
    params: String
}

#[derive(RustcEncodable)]
struct ChangeTimetableInfo {
    group_name: String,
    days_for_voting: i64
}

#[derive(RustcEncodable)]
enum FieldClass {
    ForAdd = 0,
    ForRemove = 1,
    Common = 2
}

#[derive(RustcEncodable)]
enum FieldType {
    Name = 0,
    Datetime = 1,
    Event = 2,
    Params = 3,
    Description = 4,
}

#[derive(RustcEncodable)]
enum ErrorReason {
    TooLong = 0,
    Invalid = 1,
    TimeInPast = 2,
    NotFound = 3,
    Empty = 4
}

#[derive(RustcEncodable)]
struct FieldErrorInfo {
    field_class: FieldClass,
    field_type: FieldType,
    idx: usize,
    reason: ErrorReason
}

type FieldErrors = Vec<FieldErrorInfo>;

impl GroupCreatedEvent for ChangeTimetable {
    /// описание создания
    fn user_creating_get( &self, req: &mut Request, group_id: Id ) -> AnswerResult {
        let db = try!( req.stuff().get_current_db_conn() );
        let answer = match try!( db.group_info( group_id ) ) {
            Some( group_info ) => Answer::good( ChangeTimetableInfo{
                group_name: group_info.name,
                days_for_voting: DAYS_FOR_VOTE
            } ),
            None => Answer::bad( "group_not_found" )
        };
        Ok( answer )
    }
    /// применение создания
    fn user_creating_post( &self, req: &mut Request, group_id: Id ) -> Result<FullEventInfo, AnswerResult> {
        let diff_info_str = try!( req.get_body::<TimetableDiffInfoStr>() );

        // парсим время
        let diff_info = try!( parse_times( diff_info_str ) );

        let self_start_time = time::get_time();
        let self_end_time = self_start_time + time::Duration::days( DAYS_FOR_VOTE );
        let current_time = time::now().to_timespec();

        // проверка корректности добаляемых событий
        let mut errors = Vec::new();
        check_for_add( &diff_info.add, &current_time, &mut errors );
        // проверка всех на отключение
        try!( check_for_remove( &diff_info.remove, &current_time, &mut req.stuff(), &mut errors ) );

        if errors.is_empty() == false {
            return Err( Ok( Answer::bad( errors ) ) );
        }


        //создаём выключенными события которые должны будут добавиться
        let db = try!( req.stuff().get_current_db_conn() );
        let mut added_ids : Vec<Id> = Vec::new();
        for add in &diff_info.add {
            // так как check_for_add пройдено то событие точно существует, потому исползуем unwrap
            let event = events_collection::get_timetable_event( add.event_id ).unwrap();
            let event_data = event.from_timetable( group_id, &add.params ).unwrap();
            let (start_time, end_time) = event.time_gate( &add.time );
            let new_event_info = FullEventInfo {
                id: add.event_id,
                name: add.name.clone(),
                start_time: start_time,
                end_time: end_time,
                data: event_data,
                group: Some( group_id )
            };
            let new_event_id = try!( db.add_disabled_event( &new_event_info ) );
            added_ids.push( new_event_id );
        }

        //добавляемся сами
        let data = Data {
            description: diff_info.description,
            disable: diff_info.remove,
            enable: added_ids
        };
        let self_info = FullEventInfo {
            id: ID,
            name: String::from( "Изменения расписания" ),
            start_time: self_start_time,
            end_time: self_end_time,
            data: json::encode( &data ).unwrap(),
            group: Some( group_id )
        };
        // создаём голосование и хотим что бы за изменение расписания проголосовала хотя бы половина
        Ok( group_voting::new( group_id, 0.5, &self_info ) )
    }
}

fn parse_times( diff_str: TimetableDiffInfoStr ) -> Result<TimetableDiffInfo, AnswerResult> {
    let mut parsed_time = Vec::new();
    let mut errors = Vec::new();

    if diff_str.description.is_empty() {
        errors.push( FieldErrorInfo {
            field_class: FieldClass::Common,
            field_type: FieldType::Description,
            idx: 0,
            reason: ErrorReason::Empty
        });
    }

    if MAX_DESCRIPTION_LENGTH < diff_str.description.len() {
        errors.push( FieldErrorInfo {
            field_class: FieldClass::Common,
            field_type: FieldType::Description,
            idx: 0,
            reason: ErrorReason::TooLong
        });
    }

    parsed_time.reserve( diff_str.add.len() );
    for (idx, add) in diff_str.add.into_iter().enumerate() {
        match parse_utils::parse_timespec( &add.time ) {
            Ok( tm ) => {
                parsed_time.push( AddEventInfo {
                    event_id: add.event_id,
                    name: add.name,
                    time: tm,
                    params: add.params
                });
            },
            Err( _ ) => {
                // return Err( Err( err_msg::parsing_error_param( "time" ) ) )
                errors.push( FieldErrorInfo {
                    field_class: FieldClass::ForAdd,
                    field_type: FieldType::Datetime,
                    idx: idx,
                    reason: ErrorReason::Invalid
                });
            }
        };
    }

    match errors.is_empty() {
        true => Ok( TimetableDiffInfo {
            description: diff_str.description,
            remove: diff_str.remove,
            add: parsed_time
        }),
        false => Err( Ok ( Answer::bad( errors ) ) )
    }
}

fn check_for_add( for_add: &Vec<AddEventInfo>, current_time: &Timespec, errors: &mut FieldErrors ) {
    // проверка диапазонов времен
    for (idx, add) in for_add.iter().enumerate() {
        if add.name.is_empty() {
            errors.push( FieldErrorInfo {
                field_class: FieldClass::ForAdd,
                field_type: FieldType::Name,
                idx: idx,
                reason: ErrorReason::Empty
            });
        }
        if MAX_NAME_LENGTH < add.name.len() {
            errors.push( FieldErrorInfo {
                field_class: FieldClass::ForAdd,
                field_type: FieldType::Name,
                idx: idx,
                reason: ErrorReason::TooLong
            });
        }
        if MAX_PARAMS_LENGTH < add.params.len() {
            errors.push( FieldErrorInfo {
                field_class: FieldClass::ForAdd,
                field_type: FieldType::Params,
                idx: idx,
                reason: ErrorReason::TooLong
            });
        }
        if add.time.sec < current_time.sec {
            // return common_error( String::from( "start time must after end of voting" ) );
            errors.push( FieldErrorInfo{
                field_class: FieldClass::ForAdd,
                field_type: FieldType::Datetime,
                idx: idx,
                reason: ErrorReason::TimeInPast
            });
        }
        // проверка что такие события существуют
        match events_collection::get_timetable_event( add.event_id ) {
            Ok( event ) => {
                // проверка что параметры подходят под переданные события
                if event.is_valid_params( &add.params ) == false {
                    // return common_error( format!( "invalid params for event id={}", add.event_id ) );
                    errors.push( FieldErrorInfo {
                        field_class: FieldClass::ForAdd,
                        field_type: FieldType::Params,
                        idx: idx,
                        reason: ErrorReason::Invalid
                    });
                }
            }
            Err( _ ) => {
                // return common_error( String::from( "invalid event id" ) )
                errors.push( FieldErrorInfo{
                    field_class: FieldClass::ForAdd,
                    field_type: FieldType::Event,
                    idx: idx,
                    reason: ErrorReason::Invalid
                });
            }
        }
    }
}

fn check_for_remove( for_remove: &Vec<Id>,
                     current_time: &Timespec,
                     stuff: &mut Stuff,
                     errors: &mut FieldErrors
                     ) -> EmptyResult
{
    for (idx, &id) in for_remove.iter().enumerate() {
        let maybe_event_start_time = {
            let db  = try!( stuff.get_current_db_conn() );
            try!( db.event_start_time( id ) )
        };
        match maybe_event_start_time {
            // проверяем что оно не началось или не начнётся за время голосования
            Some( remove_start_time ) => {
                if remove_start_time < *current_time {
                    // let answer = Answer::bad( FieldErrorInfo::new(
                    //     "remove_id",
                    //     "start_before_end_of_voting" ) );
                    // return Err( Ok( answer ) );
                    errors.push( FieldErrorInfo {
                        field_class: FieldClass::ForRemove,
                        field_type: FieldType::Datetime,
                        idx: idx,
                        reason: ErrorReason::TimeInPast
                    });
                }
            }
            // событие для отключения не найдено
            None => {
                // let answer = Answer::bad( FieldErrorInfo::new( "remove_id", "not_found" ) );
                // return Err( Ok( answer ) );
                errors.push( FieldErrorInfo {
                    field_class: FieldClass::ForRemove,
                    field_type: FieldType::Event,
                    idx: idx,
                    reason: ErrorReason::NotFound
                });
            }
        }
    }
    Ok( () )
}

#[derive(RustcEncodable, RustcDecodable)]
struct Data {
    description: String,
    disable: Vec<Id>,
    enable: Vec<Id>
}

fn get_data( body: &ScheduledEventInfo ) -> CommonResult<Data> {
    json::decode( &body.data )
        .map_err( |e| CommonError( format!( "ChangeTimetable event data decode error: {}", e ) ) )
}
impl ChangeByVoting for ChangeTimetable {
    /// информация о событии
    fn info( &self, _stuff: &mut Stuff, _body: &ScheduledEventInfo ) -> CommonResult<Description> {
        // let data = try!( get_data( body ) );
        // let info_as_string = json::encode( &data ).unwrap();
        // Ok( info_as_string )
        unimplemented!();
    }

    /// применить елси согласны
    fn apply( &self, stuff: &mut Stuff, _group_id: Id, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( body ) );
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
