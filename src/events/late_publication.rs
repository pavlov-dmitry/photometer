use super::{ Event, ScheduledEventInfo, FullEventInfo };
use types::{ Id, EmptyResult, CommonResult, CommonError };
use answer::{ Answer, AnswerResult };
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use mailer::Mailer;
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use mail_writer::MailWriter;
use time::{ self, Timespec };
use rustc_serialize::json;
use authentication::Userable;
use iron::prelude::*;
use get_body::GetBody;
use answer_types::{ OkInfo, AccessErrorInfo, PhotoErrorInfo, FieldErrorInfo };
use std::convert::From;

#[derive(Clone)]
pub struct LatePublication;
pub const ID : Id = 3;

impl LatePublication {
    pub fn new() -> LatePublication {
        LatePublication
    }

    pub fn create_info( parent_id: Id, group_id: Id, name: &str, start_time: Timespec, duration: time::Duration ) -> FullEventInfo {
        let info = Info {
            group_id: group_id,
            parent_id: parent_id,
        };
        let event_name: String = From::from( "Догоняем " );
        FullEventInfo {
            id: ID,
            name: event_name + name,
            start_time: start_time,
            end_time: start_time + duration,
            data: json::encode( &info ).unwrap()
        }
    }
}

#[derive(Clone, RustcDecodable)]
struct PhotoInfo {
    id: Id
}

impl Event for LatePublication {
    /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let users = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_unpublished_users( info.parent_id, info.group_id ) )
        };
        for user in users {
            let (subject, mail) = stuff.write_late_publication_mail(
                &body.name,
                &user.name,
                body.scheduled_id
            );
            try!( stuff.send_mail( &user, &subject, &mail ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, _stuff: &mut Stuff, _body: &ScheduledEventInfo ) -> EmptyResult {
        // нечего тут делать
        Ok( () )
    }
    /// описание действиz пользователя на это событие
    fn user_action_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let user_id = req.user().id;
        // если такой пользователь должен выложиться
        let db = try!( req.stuff().get_current_db_conn() );
        let need_publish = try!( db.is_unpublished_user( info.parent_id, info.group_id, user_id ) );
        let answer = if need_publish {
            // TODO: переделать на нормальное отдачу, поговорить с
            // Саньком, что ему нужно в этот момент
            Answer::good( OkInfo::new( "choose_from_gallery" ) )
        }
        else {
            // TODO: возможно необходимо вывести общий тип для таких ответов
            Answer::good( OkInfo::new( "nothing_to_do" ) )
        };
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let photo_id = try!( req.get_body::<PhotoInfo>() ).id;
        let current_user_name = req.user().name.clone();
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );

        // если такой пользователь есть должен выложиться
        let need_publish = try!( db.is_unpublished_user( info.parent_id, info.group_id, user_id ) );
        let answer = if need_publish {
            let photo_info = try!( db.get_photo_info( photo_id ) );
            if let Some( (user_name, _) ) = photo_info {
                if user_name == current_user_name {
                    try!( db.public_photo( info.parent_id,
                                           info.group_id,
                                           user_id,
                                           photo_id,
                                           true ) );
                    Answer::good( OkInfo::new( "published" ) )
                }
                else {
                    Answer::bad( AccessErrorInfo::new() )
                }
            }
            else {
                Answer::bad( PhotoErrorInfo::not_found() )
            }
        }
        else {
            Answer::good( FieldErrorInfo::new( "user", "nothing_to_do" ) )
        };
        Ok( answer )
    }
    /// информация о состоянии события
    fn info_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( req.stuff().get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( info.group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( body.scheduled_id,
                                                                        info.group_id ) );

        let answer = Answer::good( LatePublicationInfo {
            id: ID,
            name: body.name.clone(),
            all_count: group_members_count,
            published: published_photo_count
        } );
        Ok( answer )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( info.group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( body.scheduled_id, info.group_id ) );
        Ok( group_members_count == published_photo_count )
    }
}

#[derive(RustcEncodable)]
struct LatePublicationInfo {
    id: Id,
    name: String,
    all_count: u32,
    published: u32
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    group_id: Id,
    parent_id: Id
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( &str_body )
        .map_err( |e| CommonError( format!( "LatePublication event decode error: {}", e ) ) )
}
