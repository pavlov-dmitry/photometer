use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    FullEventInfo,
    Description,
    UserAction,
    get_group_id
};
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
use answer_types::{ OkInfo, AccessErrorInfo };
use std::convert::From;
use parse_utils::GetMsecs;

#[derive(Clone)]
pub struct LatePublication;
pub const ID : EventId = EventId::LatePublication;

impl LatePublication {
    pub fn new() -> LatePublication {
        LatePublication
    }

    #[allow(dead_code)]
    pub fn create_info( parent_id: Id, group_id: Id, name: &str, start_time: Timespec, duration: time::Duration ) -> FullEventInfo {
        let info = Info {
            parent_id: parent_id,
        };
        let event_name: String = String::from( "Догоняем " );
        FullEventInfo {
            id: ID,
            name: event_name + name,
            start_time: start_time,
            end_time: start_time + duration,
            data: json::encode( &info ).unwrap(),
            group: Some( group_id ),
            creator: None
        }
    }
}

#[derive(Clone, RustcDecodable)]
struct PhotoInfo {
    id: Id
}

impl Event for LatePublication {
    /// идентификатор события
    fn id( &self ) -> EventId {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( body ) );
        let users = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_unpublished_users( info.parent_id ) )
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
    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let group_id = try!( get_group_id( body ) );
        let info = try!( get_info( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( info.parent_id ) );
        // let need_publish = try!( db.is_unpublished_user( info.parent_id, info.group_id, user_id ) );
        // let answer = if need_publish {
        //     // TODO: переделать на нормальное отдачу, поговорить с
        //     // Саньком, что ему нужно в этот момент
        //     Answer::good( OkInfo::new( "choose_from_gallery" ) )
        // }
        // else {
        //     // TODO: возможно необходимо вывести общий тип для таких ответов
        //     Answer::good( OkInfo::new( "nothing_to_do" ) )
        // };

        let desc = Description::new( LatePublicationInfo {
            id: ID,
            name: body.name.clone(),
            all_count: group_members_count,
            published: published_photo_count
        } );
        Ok( desc )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let group_id = try!( get_group_id( body ) );
        let info = try!( get_info( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( info.parent_id ) );
        Ok( group_members_count == published_photo_count )
    }

    /// действие которое должен осуществить пользователь
    fn user_action( &self, _stuff: &mut Stuff, _body: &ScheduledEventInfo, _user_id: Id ) -> CommonResult<UserAction> {
        unimplemented!();
    }

    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( body ) );
        let photo_id = try!( req.get_body::<PhotoInfo>() ).id;
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );

        // если такой пользователь есть должен выложиться
        let need_publish = try!( db.is_unpublished_user( info.parent_id, user_id ) );
        let answer = if need_publish {
            let photo_info = try!( db.get_short_photo_info( photo_id ) );
            if let Some( photo_info ) = photo_info {
                if photo_info.owner.id == user_id {
                    let prev = try!( db.get_last_pubcation_photo( info.parent_id ) );
                    try!( db.public_photo( info.parent_id,
                                           user_id,
                                           photo_id,
                                           true,
                                           time::get_time().msecs(),
                                           prev.clone() ) );
                    if let Some( last_id ) = prev {
                        try!( db.set_next_publication_photo( last_id, photo_id ) );
                    }
                    Answer::good( OkInfo::new( "published" ) )
                }
                else {
                    Answer::bad( AccessErrorInfo::new() )
                }
            }
            else {
                Answer::not_found()
            }
        }
        else {
            Answer::bad( AccessErrorInfo::new() )
        };
        Ok( answer )
    }
}

#[derive(RustcEncodable, Debug)]
struct LatePublicationInfo {
    id: EventId,
    name: String,
    all_count: u32,
    published: u32
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    parent_id: Id
}

fn get_info( body: &ScheduledEventInfo ) -> CommonResult<Info> {
    json::decode( &body.data )
        .map_err( |e| CommonError( format!( "LatePublication event decode error: {}", e ) ) )
}
