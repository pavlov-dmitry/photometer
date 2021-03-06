/// Событие которое агрегирует в себя другое событие и выполняет его только если
/// за него проголосовало необходимое кол-во членов группы

use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    FullEventInfo,
    events_collection,
    Description,
    UserAction,
};
use super::helpers;
use types::{
    Id,
    EmptyResult,
    CommonResult,
    CommonError,
    ShortInfo,
};
use rustc_serialize::json;
use database::{ Databaseable };
use stuff::{ Stuff };
use db::votes::{ DbVotes, Votes };
use iron::prelude::*;
use answer::{ AnswerResult };
use std::cmp;
use time;
use db::group_feed::DbGroupFeed;
use super::feed_types::FeedEventState;
use parse_utils::{ GetMsecs };

/// абстракция события которое применяется после того как группа проголосовала ЗА
pub trait ChangeByVoting {
    /// информация о событии
    //TODO: Сейчас информация о событии приходит в виде строки,
    // закодированного JSON объекта, придумать какой-нить более
    // элегантный способ, чтобы возвращался сразу объект
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description>;
    // действие на старт голосования
    fn start( &self, stuff: &mut Stuff, group: &ShortInfo, body: &ScheduledEventInfo ) -> EmptyResult;
    /// применить елси согласны
    fn apply( &self, stuff: &mut Stuff, group: &ShortInfo, body: &ScheduledEventInfo ) -> EmptyResult;
    /// краткое имя события, будет в основном использоваться в рассылке
    fn name( &self, stuff: &mut Stuff, group: &ShortInfo, body: &ScheduledEventInfo ) -> CommonResult<String>;
}

/// создание нового голосования для группы
pub fn new( group_id: Id, success_coeff: f32, internal_event: &FullEventInfo ) -> FullEventInfo {
    let data = Data {
        internal_id: internal_event.id,
        success_coeff: success_coeff,
        internal_data: internal_event.data.clone()
    };
    FullEventInfo {
        id: ID,
        name: internal_event.name.clone(),
        start_time: internal_event.start_time.clone(),
        end_time: internal_event.end_time.clone(),
        data: json::encode( &data ).unwrap(),
        group: Some( group_id ),
        creator: internal_event.creator.clone()
    }
}

#[derive(Clone)]
pub struct GroupVoting;
pub const ID : EventId = EventId::GroupVoting;

impl GroupVoting {
    pub fn new() -> GroupVoting {
        GroupVoting
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Data {
    internal_id: EventId,
    success_coeff: f32,
    internal_data: String
}

#[derive(Debug, RustcEncodable)]
struct FeedData {
    internal_id: EventId,
    is_success: bool
}

impl Event for GroupVoting {
    /// идентификатор события
    fn id( &self ) -> EventId {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( body ) );
        let group = body.group.as_ref().unwrap();

        {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.add_rights_of_voting_for_group( body.scheduled_id, group.id ) );

            // Добавляем запись в ленту группы
            let feed_data = Description::new( FeedData {
                internal_id: data.internal_id,
                is_success: true
            }).to_string();
            try!( db.add_to_group_feed( time::get_time().msecs(),
                                        group.id,
                                        body.scheduled_id,
                                        FeedEventState::Start,
                                        &feed_data ) );
        }

        let event = try!( events_collection::get_change_by_voting( data.internal_id ) );
        let internal_body = make_internal_body( &data, &body );
        try!( event.start( stuff, group, &internal_body ) );

        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( body ) );
        let group_id = body.group.as_ref().unwrap().id;
        let internal_body = make_internal_body( &data, &body );

        let votes = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_votes( body.scheduled_id ) )
        };
        // подсчитываем голоса
        let is_success = is_finished_success( &votes, data.success_coeff );
        if  is_success { // если набралось достаточно
            let change = try!( events_collection::get_change_by_voting( data.internal_id ) );
            // и применяем изменение
            let group = body.group.as_ref().unwrap();
            try!( change.apply( stuff, group, &internal_body ) );
        }

        // Добавляем запись в ленту группы
        let db = try!( stuff.get_current_db_conn() );
        let feed_data = Description::new( FeedData {
            internal_id: data.internal_id,
            is_success: is_success
        }).to_string();
        try!( db.add_to_group_feed( time::get_time().msecs(),
                                    group_id,
                                    body.scheduled_id,
                                    FeedEventState::Finish,
                                    &feed_data ) );
        // удаляем стартовое за ненадобностью из ленты группы
        try!( db.remove_start_event_from_feed( body.scheduled_id ) );

        Ok( () )
    }
    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let data = try!( get_data( body ) );
        let change = try!( events_collection::get_change_by_voting( data.internal_id ) );
        let internal_body = make_internal_body( &data, &body );
        let event_info = try!( change.info( stuff, &internal_body ) );

        let votes = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_votes( body.scheduled_id ) )
        };

        let desc = Description::new( GroupVoitingInfo {
            internal_id: data.internal_id,
            info: event_info,
            all_count: votes.all_count,
            yes: votes.yes.len(),
            no: votes.no.len(),
            success_coeff: data.success_coeff
        } );
        Ok( desc )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let data = try!( get_data( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        Ok( is_fail( &votes, data.success_coeff ) || is_success( &votes, data.success_coeff ) )
    }

    /// действие которое должен осуществить пользователь
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction> {
        helpers::get_action_by_vote( stuff, body.scheduled_id, user_id )
    }

    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        helpers::set_user_vote( req, body )
    }
}

#[derive(RustcEncodable, Debug)]
struct GroupVoitingInfo {
    internal_id: EventId,
    info: Description,
    all_count: usize,
    yes: usize,
    no: usize,
    success_coeff: f32
}

fn min_success_count( all: usize, success_coeff: f32 ) -> usize {
    let min_success_count = ( all as f32 * success_coeff ) as usize;
    //NOTE: нельзя допускать того что-бы действие принималось без чего бы то еще согласия
    cmp::max( 2, min_success_count )
}

fn is_success( votes: &Votes, success_coeff: f32 ) -> bool {
    min_success_count( votes.all_count, success_coeff ) <= votes.yes.len()
}

//NOTE: разница с функцией is_success в том что эта считает только
// голоса проголосовавших. Чтобы по окончанию голосвания подсчитать
// результат только по тем кто проголосовал. Иначе при отсутсвии
// нескольких человек ни одно голосование не пройдет
fn is_finished_success( votes: &Votes, success_coeff: f32 ) -> bool {
    min_success_count( votes.yes.len() + votes.no.len(), success_coeff ) <= votes.yes.len()
}

fn is_fail( votes: &Votes, success_coeff: f32 ) -> bool {
    let yes_count = votes.yes.len();
    let no_count = votes.no.len();
    let not_voited_count = votes.all_count - no_count - yes_count;
    (not_voited_count + yes_count) < min_success_count( votes.all_count, success_coeff )
}

fn make_internal_body( data: &Data, body: &ScheduledEventInfo ) -> ScheduledEventInfo {
    ScheduledEventInfo {
        id: data.internal_id,
        start_time: body.start_time,
        end_time: body.end_time,
        scheduled_id: body.scheduled_id,
        name: body.name.clone(),
        state: body.state.clone(),
        data: data.internal_data.clone(),
        group: body.group.clone(),
        creator: body.creator.clone()
    }
}

fn get_data( body: &ScheduledEventInfo ) -> CommonResult<Data> {
    json::decode( &body.data )
        .map_err( |e| CommonError( format!( "GroupVoting event data decode error: {}", e ) ) )
}

// fn get_event_name( stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<String> {
//     let event = try!( events_collection::get_change_by_voting( body.id ) );
//     event.name( stuff, &body )
// }

// fn get_group_name( info: &ScheduledEventInfo ) -> String {
//     match info.group {
//         Some( ref group_info ) => group_info.name.clone(),
//         None => panic!( "GroupVoting invalid event body, without group info." )
//     }
// }
