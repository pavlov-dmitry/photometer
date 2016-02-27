use rustc_serialize::json;
use iron::prelude::*;

use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    GroupCreatedEvent,
    FullEventInfo,
    Description,
    UserAction,
};
use super::helpers;
use types::{ Id, ShortInfo, EmptyResult, CommonResult, CommonError };
use answer::{ Answer, AnswerResult };
use answer_types::{ FieldErrorInfo };
use database::{ Databaseable };
use db::users::DbUsers;
use db::groups::DbGroups;
use db::votes::DbVotes;
use time;
use stuff::{ Stuff, Stuffable };
use get_body::{ GetBody };
use authentication::{ Userable };
use mail_writer::{ MailWriter };
use mailer::{ Mailer };

#[derive(Clone)]
pub struct UserInviteToGroup;
pub const ID: EventId = EventId::UserInvite;

impl UserInviteToGroup {
    pub fn new() -> UserInviteToGroup {
        UserInviteToGroup
    }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
struct UserInviteQuery {
    user_id: Id,
    text: String
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
struct UserInviteInfo {
    group_id: Id,
    user_id: Id,
    text: String
}

#[derive(Debug, RustcEncodable)]
struct InviteInfo {
    group: ShortInfo,
    group_description: String,
    user: ShortInfo,
    is_voted: bool,
    success: bool
}

impl GroupCreatedEvent for UserInviteToGroup {
    /// описание создания
    fn user_creating_get( &self, req: &mut Request, group_id: Id ) -> AnswerResult {
        let db = try!( req.stuff().get_current_db_conn() );

        let group_info = match try!( db.group_info( group_id ) ) {
            Some( group_info ) => ShortInfo {
                id: group_info.id,
                name: group_info.name
            },
            None => return Ok( Answer::not_found() )
        };

        Ok( Answer::good( group_info ) )
    }
    /// применение создания
    fn user_creating_post( &self, req: &mut Request, group_id: Id ) -> Result<FullEventInfo, AnswerResult> {
        let invite_query = try!( req.get_body::<UserInviteQuery>() );
        let user_id = req.user().id;

        // леса проверок
        if invite_query.text.is_empty() {
            return Err( Ok( Answer::bad( FieldErrorInfo::empty( "text" ) ) ) );
        }
        if 2048 < invite_query.text.len() {
            return Err( Ok( Answer::bad( FieldErrorInfo::too_long( "text" ) ) ) );
        }
        let db = try!( req.stuff().get_current_db_conn() );
        if try!( db.user_id_exists( invite_query.user_id ) ) == false {
            return Err( Ok( Answer::bad( FieldErrorInfo::not_found( "user" ) ) ) );
        }
        if try!( db.is_member( invite_query.user_id, group_id ) ) == true {
            return Err( Ok( Answer::bad( FieldErrorInfo::new( "user", "in_group" ) ) ) );
        }

        //NOTE: тут делаем unwrap так как сверху проверяли на наличие
        let user_info = try!( db.user_by_id( invite_query.user_id ) ).unwrap();
        let group_info = try!( db.group_info( group_id ) ).unwrap();

        let start_time = time::get_time();
        let end_time = start_time + time::Duration::days( 7 );

        let invite_info = UserInviteInfo {
            group_id: group_id,
            user_id: invite_query.user_id,
            text: invite_query.text
        };

        Ok( FullEventInfo{
            id: ID,
            name: make_invite_name( &group_info.name, &user_info.name ),
            start_time: start_time,
            end_time: end_time,
            data: json::encode( &invite_info ).unwrap(),
            group: None,
            creator: Some( user_id )
        })
    }
}

fn make_invite_name( group_name: &str, user_name: &str ) -> String {
    format!( "Приглашение пользователя '{}' в группу '{}'", user_name, group_name )
}

impl Event for UserInviteToGroup {
    /// идентификатор события
    fn id( &self ) -> EventId {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let inviter = body.creator.as_ref().unwrap();

        let ( user_info, group_info ) = {
            // ставим что пользователь может проголовать, будет ли он присоединяться
            let db = try!( stuff.get_current_db_conn() );
            try!( db.add_rights_of_voting( body.scheduled_id, &[ info.user_id ] ) );

            let user_info = try!( db.user_by_id( info.user_id ) ).unwrap();
            let group_info = try!( db.group_info( info.group_id ) ).unwrap();
            ( user_info, group_info )
        };

        // пишем и отсылаем письмо с приглашением в группу
        let (subject, mail) = stuff.write_invite_to_group_mail( &inviter.name,
                                                                &group_info.name,
                                                                body.scheduled_id );
        try!( stuff.send_mail( &user_info, &subject, &mail ) );
        // пишем письмо тому кто пригласил
        let inviter_info = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.user_by_id( inviter.id ) ).unwrap()
        };
        let (subject, mail) = stuff.write_user_invited_to_group_mail( &inviter.name,
                                                                      &group_info.name,
                                                                      body.scheduled_id );
        try!( stuff.send_mail( &inviter_info, &subject, &mail ) );
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        if 0 < votes.yes.len() {
            //TODO: создать событие для голосвания всей группой о приглашении нового пользователя
            println!( "need create join to group event with {:?}", info );
        }
        Ok( () )
    }
    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        let group_id = info.group_id;
        let group_info = try!( db.group_info( group_id ) ).unwrap();
        let user_info = try!( db.user_by_id( info.user_id ) ).unwrap();
        let invite_info = InviteInfo {
            group: ShortInfo {
                id: group_info.id,
                name: group_info.name
            },
            group_description: group_info.description,
            user: ShortInfo {
                id: user_info.id,
                name: user_info.name
            },
            is_voted: 0 < ( votes.yes.len() + votes.no.len() ),
            success: 0 < votes.yes.len()
        };
        let desc = Description::new( invite_info );
        Ok( desc )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let db = try!( stuff.get_current_db_conn() );
        db.is_all_voted( body.scheduled_id )
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

fn get_info( str_body: &String ) -> CommonResult<UserInviteInfo> {
    json::decode( &str_body )
        .map_err( |e| CommonError( format!( "UserInvite event data decode error: {}", e ) ) )
}
