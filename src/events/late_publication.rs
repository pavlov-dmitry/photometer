use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    FullEventInfo,
    Description,
    UserAction,
    get_group_id,
    publication
};
use super::publication::PublishError;
use types::{ Id, EmptyResult, CommonResult, CommonError };
use answer::{ AnswerResult };
use database::{ Databaseable };
use stuff::{ Stuff };
use mailer::Mailer;
use db::groups::DbGroups;
use db::publication::DbPublication;
use mail_writer::MailWriter;
use time::{ self, Timespec };
use rustc_serialize::json;
use iron::prelude::*;
use std::convert::From;

#[derive(Clone)]
pub struct LatePublication;
pub const ID : EventId = EventId::LatePublication;

impl LatePublication {
    pub fn new() -> LatePublication {
        LatePublication
    }

    pub fn create_info( parent_id: Id, group_id: Id, name: &str, start_time: Timespec, duration: time::Duration ) -> FullEventInfo {
        let info = Info {
            parent_id: parent_id,
        };
        FullEventInfo {
            id: ID,
            name: String::from( "Догоняем " ) + name,
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
        let info = try!( get_info( body ) );
        publication::make_info( stuff, body, info.parent_id )
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
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction> {
        let info = try!( get_info( body ) );
        publication::get_user_action( stuff, user_id, info.parent_id )
    }

    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( body ) );
        publication::process_user_action_post( req, info.parent_id )
    }
}

pub fn process_publish_photo( req: &mut Request, body: &ScheduledEventInfo, photo_id: Id ) -> Result<(), PublishError> {
    let info = try!( get_info( body ) );
    publication::process_publish_photo( req, info.parent_id, photo_id )
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    parent_id: Id
}

fn get_info( body: &ScheduledEventInfo ) -> CommonResult<Info> {
    json::decode( &body.data )
        .map_err( |e| CommonError( format!( "LatePublication event decode error: {}", e ) ) )
}
