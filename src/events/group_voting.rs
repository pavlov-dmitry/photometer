/// Событие которое агрегирует в себя другое событие и выполняет его только если
/// за него проголосовало необходимое кол-во членов группы

use super::{
    Event,
    ScheduledEventInfo,
    FullEventInfo,
    events_collection,
    Description,
    UserAction,
};
use super::helpers;
use types::{ Id, EmptyResult, CommonResult, CommonError, common_error };
use rustc_serialize::json;
use database::{ Databaseable };
use stuff::{ Stuff };
use db::votes::{ DbVotes, Votes };
use db::groups::DbGroups;
use mail_writer::MailWriter;
use iron::prelude::*;
use answer::{ AnswerResult };
use std::cmp;

/// абстракция события которое применяется после того как группа проголосовала ЗА
pub trait ChangeByVoting {
    /// информация о событии
    //TODO: Сейчас информация о событии приходит в виде строки,
    // закодированного JSON объекта, придумать какой-нить более
    // элегантный способ, чтобы возвращался сразу объект
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description>;
    /// применить елси согласны
    fn apply( &self, stuff: &mut Stuff, group_id: Id, body: &ScheduledEventInfo ) -> EmptyResult;
    /// краткое имя события, будет в основном использоваться в рассылке
    fn name( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<String>;
}

/// создание нового голосования для группы
pub fn new( group_id: Id, success_coeff: f32, internal_event: &FullEventInfo ) -> FullEventInfo {
    let data = Data {
        internal_id: internal_event.id,
        group_id: group_id,
        success_coeff: success_coeff,
        internal_data: internal_event.data.clone()
    };
    FullEventInfo {
        id: ID,
        name: internal_event.name.clone(),
        start_time: internal_event.start_time.clone(),
        end_time: internal_event.end_time.clone(),
        data: json::encode( &data ).unwrap(),
        group: Some( group_id )
    }
}

#[derive(Clone)]
pub struct GroupVoting;
pub const ID : Id = 4;

impl GroupVoting {
    pub fn new() -> GroupVoting {
        GroupVoting
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Data {
    internal_id: Id,
    group_id: Id,
    success_coeff: f32,
    internal_data: String
}

impl Event for GroupVoting {
    /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( body ) );

        {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.add_rights_of_voting_for_group( body.scheduled_id, data.group_id ) );
        }

        let group_name = try!( get_group_name( stuff, &data ) );
        let internal_body = make_internal_body( &data, &body );
        let event_name = try!( get_event_name( stuff, &internal_body ) );

        try!( helpers::send_to_group( stuff, data.group_id, &mut |stuff, _user| {
            stuff.write_group_voiting_started_mail( &event_name,
                                                    &group_name,
                                                    body.scheduled_id )
        }));
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( body ) );
        let internal_body = make_internal_body( &data, &body );
        let event_name = try!( get_event_name( stuff, &internal_body ) );
        let group_name = try!( get_group_name( stuff, &data ) );

        let votes = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_votes( body.scheduled_id ) )
        };
        // подсчитываем голоса
        if is_success( &votes, data.success_coeff ) { // если набралось достаточно
            let change = try!( events_collection::get_change_by_voting( data.internal_id ) );

            // рассылаем всем что мол утверждено
            try!( helpers::send_to_group( stuff, data.group_id, &mut |stuff, _user| {
                stuff.write_group_voiting_accepted_mail( &event_name,
                                                         &group_name,
                                                         body.scheduled_id )
            }));

            // и применяем изменение
            try!( change.apply( stuff, data.group_id, &internal_body ) );
        }
        else { // если не хватило голосов
            // расслываем что мол увы.
            try!( helpers::send_to_group( stuff, data.group_id, &mut |stuff, _user| {
                stuff.write_group_voiting_denied_mail( &event_name,
                                                       &group_name,
                                                       body.scheduled_id )
            }));
        }
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
            info: event_info,
            all_count: votes.all_count,
            yes: votes.yes.len(),
            no: votes.no.len(),
            min_success_count: min_success_count( votes.all_count, data.success_coeff )
        } );
        Ok( desc )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let data = try!( get_data( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        let all_voted = votes.all_count == ( votes.yes.len() + votes.no.len() );
        Ok( all_voted || is_success( &votes, data.success_coeff ) )
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
    info: Description,
    all_count: usize,
    yes: usize,
    no: usize,
    min_success_count: usize
}

fn min_success_count( all: usize, success_coeff: f32 ) -> usize {
    let min_success_count = ( all as f32 * success_coeff ) as usize;
    //NOTE: нельзя допускать того что-бы действие принималось без чего бы то еще согласия
    cmp::max( 2, min_success_count )
}

fn is_success( votes: &Votes, success_coeff: f32 ) -> bool {
    min_success_count( votes.all_count, success_coeff ) <= votes.yes.len()
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
        group: body.group.clone()
    }
}

fn get_data( body: &ScheduledEventInfo ) -> CommonResult<Data> {
    json::decode( &body.data )
        .map_err( |e| CommonError( format!( "GroupVoting event data decode error: {}", e ) ) )
}

fn get_event_name( stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<String> {
    let event = try!( events_collection::get_change_by_voting( body.id ) );
    event.name( stuff, &body )
}

fn get_group_name( stuff: &mut Stuff, data: &Data ) -> CommonResult<String> {
    let db = try!( stuff.get_current_db_conn() );
    let group_info = try!( db.group_info( data.group_id ) );
    match group_info {
        Some( info ) => Ok( info.name ),
        None => common_error( "GroupVoting::get_group_name: invalid group id".to_owned() )
    }
}
