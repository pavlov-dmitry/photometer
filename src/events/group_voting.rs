/// Событие которое агрегирует в себя другое событие и выполняет его только если 
/// за него проголосовало необходимое кол-во членов группы

use super::{ Event, ScheduledEventInfo, FullEventInfo, events_collection };
use types::{ Id, EmptyResult, CommonResult };
use rustc_serialize::json;
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use db::votes::{ DbVotes, Votes };
use iron::prelude::*;
use answer::{ AnswerResult, Answer };
use authentication::{ Userable };
use get_param::GetParamable;

/// абстракция события которое применяется после того как группа проголосовала ЗА
pub trait ChangeByVoting {
    /// информация о событии
    fn get_info( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// применить елси согласны
    fn apply( &self, stuff: &mut Stuff, group_id: Id, body: &ScheduledEventInfo ) -> EmptyResult;
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
        data: json::encode( &data ).unwrap()
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
        let data = try!( get_data( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        try!( db.add_rights_of_voting_for_group( body.scheduled_id, data.group_id ) );
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( &body.data ) );
        let votes = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_votes( body.scheduled_id ) )
        };
        if is_success( &votes, data.success_coeff ) {
            let change = try!( events_collection::get_change_by_voting( data.internal_id ) );
            let internal_body = make_internal_body( &data, &body );
            try!( change.apply( stuff, data.group_id, &internal_body ) );
        }
        Ok( () )
    }
    /// описание действия пользователя на это событие 
    fn user_action_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let data = try!( get_data( &body.data ) );
        let change = try!( events_collection::get_change_by_voting( data.internal_id ) );
        let internal_body = make_internal_body( &data, &body );
        let mut answer = try!( change.get_info( req, &internal_body ) );

        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );
        let is_need_vote = try!( db.is_need_user_vote( body.scheduled_id, user_id ) );
        if is_need_vote {
            answer.add_record( "user", &"no_need_vote".to_string() );
        }
        else {
            answer.add_record( "user", &"need_some_voting".to_string() );    
        }
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let vote: bool = try!( req.get_param( "vote" ) ) == "yes";
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );
        let is_need_vote = try!( db.is_need_user_vote( body.scheduled_id, user_id ) );

        let mut answer = Answer::new();
        if is_need_vote {
            try!( db.set_vote( body.scheduled_id, user_id, vote ) );
            answer.add_record( "vote", &"accepted".to_string() );
        }
        else {
            answer.add_error( "user", "no_need_vote" );
        }
        Ok( answer )
    }
    /// информация о состоянии события
    fn info_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let data = try!( get_data( &body.data ) );
        let change = try!( events_collection::get_change_by_voting( data.internal_id ) );
        let internal_body = make_internal_body( &data, &body );
        let mut answer = try!( change.get_info( req, &internal_body ) );

        let votes = { 
            let db = try!( req.stuff().get_current_db_conn() );
            try!( db.get_votes( body.scheduled_id ) )
        };

        answer.add_record( "all_count", &votes.all_count );
        answer.add_record( "yes", &votes.yes.len() );
        answer.add_record( "no", &votes.no.len() );
        Ok( answer )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let data = try!( get_data( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        let all_voted = votes.all_count == ( votes.yes.len() + votes.no.len() );
        Ok( all_voted || is_success( &votes, data.success_coeff ) )
    }
}

fn is_success( votes: &Votes, success_coeff: f32 ) -> bool {
    let min_success_count = ( votes.all_count as f32 * success_coeff ) as usize;
    //нельзя допускать того что-бы действие принималось без чего бы то еще согласия
    let min_success_count = if min_success_count < 2 { 2 } else { min_success_count };
    min_success_count <= votes.yes.len()
}

fn make_internal_body( data: &Data, body: &ScheduledEventInfo ) -> ScheduledEventInfo {
    ScheduledEventInfo {
        id: data.internal_id,
        scheduled_id: body.scheduled_id,
        name: body.name.clone(),
        state: body.state.clone(),
        data: data.internal_data.clone()
    }
}

fn get_data( str_body: &str ) -> CommonResult<Data> {
    json::decode( str_body ).map_err( |e| format!( "GroupVoting event data decode error: {}", e ) )   
}