use rustc_serialize::json;
use iron::prelude::*;

use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    UserCreatedEvent,
    FullEventInfo,
    Description,
    UserAction,
};
use types::{ Id, EmptyResult, CommonResult, CommonError };
use db::users::DbUsers;
use db::groups::DbGroups;

#[derive(Clone)]
pub struct UserInviteToGroup;
pub ID: EventId = EventId::UserInvite;

impl UserInviteToGroup {
    pub fn new() -> UserInviteToGroup {
        UserInviteToGroup
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct UserInviteInfo {
    group_id: Id,
    user_id: Id,
    text: String
}

#[derive(RustcEncodable)]
struct InviteInfo {
    group: ShortInfo,
    group_description: String
    user: ShortInfo
}

impl GroupCreatedEvent for UserInviteToGroup {
    /// описание создания
    fn user_creating_get( &self, _req: &mut Request, group_id: Id ) -> AnswerResult {
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
        let invite_info = try!( req.get_body::<UserInviteInfo>() );
        let user_id = req.user().id;

        // леса проверок
        if invite_info.text.is_empty() {
            return Err( Ok( Answer::bad( FieldErrorInfo::empty( "text" ) ) ) );
        }
        if 2048 < invite_info.text.len() {
            return Err( Ok( Answer::bad( FieldErrorInfo::too_long( "text" ) ) ) );
        }
        let db = try!( req.stuff().get_current_db_conn() );
        if try!( db.user_id_exists( invite_info.group_id ) ) == false {
            return Err( Ok( Answer::bad( FieldErrorInfo::not_found( "user" ) ) ) );
        }
        if try!( db.is_member( invite_info.user_id, invite_info.group_id ) ) == true {
            return Err( Ok( Answer::bad( FieldErrorInfo::new( "user", "in_group" ) ) ) );
        }

        //NOTE: тут делаем unwrap так как сверху проверяли на наличие
        let user_info = try!( db.user_by_id( invite_info.user_id ) ).unwrap();
        let group_info = try!( db.group_info( invite_info.group_id ) ).unwrap();

        let start_time = time::get_time();
        let end_time = start_time + time::Duration::days( 7 );

        Ok( FullEventInfo{
            id: ID,
            name: user_info.name,
            start_time: start_time,
            end_time: end_time,
            data: json::encode( &invite_info ).unwrap(),
            group: None,
            creator: Some( user_id )
        })
    }
}

impl Event for UserInvite {
    /// идентификатор события
    fn id( &self ) -> EventId {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );

        // ставим что пользователь может проголовать, будет ли он присоединяться
        let db = try!( stuff.get_current_db_conn() );
        try!( db.add_rights_of_voting( body.scheduled_id, &[ info.user_id ] ) );

        // пишем и отсылаем письмо с приглашением в группу
        let inviter_id = body.creator.unwrap();
        let inviter_info = try!( db.user_by_id( inviter_id ) ).unwrap();
        let user_info = try!( db.user_by_id( info.user_id ) ).unwrap();

        let group_info = try!( db.group_info( info.group_id ) ).unwrap();

        let (subject, body) = stuff.write_invite_to_group_mail( &inviter_info.name,
                                                                &group_info.name,
                                                                body.scheduled_id );
        try!( stuff.send_mail( &mut self, &user_info, &subject, &body ) );
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        if 0 < votes.yes.len() {
            //TODO: создать событие для голосвания всей группой о приглашении нового пользователя
        }
    }
    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        let group_info = try!( db.group_info( body.group_id ) ).unwrap();
        let user_info = try!( db.user_by_id( body.user_id ) ).unwrap();
        let invite_info = InviteInfo {
            group: ShortInfo {
                id: group_info.id,
                name: group_info.name
            },
            group_description: group_info.description,
            user: ShortInfo {
                id: user_info.id,
                name: user_info.name
            }
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
