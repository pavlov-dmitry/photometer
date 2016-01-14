/// Сюда выношу общий код который может быть использован в разных событиях
use super::{
    UserAction,
    ScheduledEventInfo,
    VoteInfo
};
use iron::prelude::*;
use answer::{ AnswerResult, Answer };
use answer_types::{ OkInfo, FieldErrorInfo };
use types::{ Id, CommonResult, EmptyResult };
use authentication::{ User, Userable };
use stuff::{ Stuff, Stuffable };
use database::{ Databaseable };
use db::votes::DbVotes;
use db::groups::DbGroups;
use mailer::Mailer;
use get_body::GetBody;

/// Возвращает действие пользователя при голосовании
pub fn get_action_by_vote( stuff: &mut Stuff, scheduled_id: Id, user_id: Id ) -> CommonResult<UserAction> {
    let db = try!( stuff.get_current_db_conn() );
    let is_need_vote = try!( db.is_need_user_vote( scheduled_id, user_id ) );
    let action = match is_need_vote {
        true => UserAction::Vote,
        false => UserAction::None
    };
    Ok( action )
}


/// обработка голосования
pub fn set_user_vote( req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
    let vote_info = try!( req.get_body::<VoteInfo>() );
    let vote = vote_info.is_yes();

    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let is_need_vote = try!( db.is_need_user_vote( body.scheduled_id, user_id ) );

    let answer = if is_need_vote {
        try!( db.set_vote( body.scheduled_id, user_id, vote ) );
        Answer::good( OkInfo::new( "accepted" ) )
    }
    else {
        // TODO: возможно нужно создать новый общий тип под такие
        // действия или использовать старый AccessErrorInfo
        Answer::bad( FieldErrorInfo::new( "user", "no_need_vote" ) )
    };
    Ok( answer )
}

/// формирует рассылку писем определенной группе
pub fn send_to_group<F>( stuff: &mut Stuff, group_id: Id, make_mail: &mut F ) -> EmptyResult
    where F: FnMut(&mut Stuff, &User)->(String, String)
{
    let members = {
        let db = try!( stuff.get_current_db_conn() );
        try!( db.get_members( group_id ) )
    };
    for member in members {
        let (mail_subject, mail_body) = make_mail( stuff, &member );
        try!( stuff.send_mail( &member, &mail_subject, &mail_body ) );
    }
    Ok( () )
}
