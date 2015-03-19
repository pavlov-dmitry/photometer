use super::{ Event, CreateFromTimetable, ScheduledEventInfo };
use super::late_publication::LatePublication;
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use rustc_serialize::json;
use mailer::Mailer;
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use db::events::DbEvents;
use mail_writer::MailWriter;
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use authentication::{ Userable };
use std::time::duration::Duration;
use time;
use iron::prelude::*;
use get_body::GetBody;

#[derive(Clone)]
pub struct Publication;
pub const ID : Id = 1;

impl Publication {
    pub fn new() -> Publication {
        Publication
    }
}

#[derive(Clone, Copy, RustcDecodable)]
struct PhotoInfo {
    id: Id
}

impl Event for Publication {
    /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let members = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_members( info.group_id ) )
        };
        for user in members.iter() {
            let (subject, mail) = stuff.write_time_for_publication_mail( 
                &body.name, 
                &user.name, 
                body.scheduled_id 
            );
            try!( stuff.send_mail( user, &subject, &mail ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        try!( db.make_publication_visible( body.scheduled_id, info.group_id ) );
        //TODO: старт голосования
        
        //старт события загрузки опоздавших
        //FIXME: использовать более "дешевую" функцию для определения что есть отставшие
        let unpublished_users = try!( db.get_unpublished_users( body.scheduled_id, info.group_id ) );
        if unpublished_users.is_empty() == false {
            let event_info = LatePublication::create_info( 
                body.scheduled_id,
                info.group_id, 
                &body.name, 
                time::get_time(), 
                Duration::days( 365 )
            );
            try!( db.add_events( &[ event_info ] ) );
        }
        
        Ok( () )
    }
    /// описание действиz пользователя на это событие 
    fn user_action_get( &self, _req: &mut Request, _body: &ScheduledEventInfo ) -> AnswerResult {
        let mut answer = Answer::new();
        // TODO: переделать на нормальное отдачу, поговорить с Саньком, что ему нужно в этот момент
        answer.add_record( "choose", &"from_gallery".to_string() );
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let photo_id = try!( req.get_body::<PhotoInfo>() ).id;
        let user = req.user().clone();
        let db = try!( req.stuff().get_current_db_conn() );
        let mut answer = Answer::new();
        if let Some( (user_name, _) ) = try!( db.get_photo_info( photo_id ) ) {
            if user_name == user.name {
                try!( db.public_photo( body.scheduled_id, info.group_id, user.id, photo_id, false ) );
                answer.add_record( "published", &"ok".to_string() );
            }
            else {
                answer.add_error( "permisson", "denied" );
            }
        }
        else {
            answer.add_error( "photo", "not_found" );
        }
        Ok( answer )
    }
    /// информация о состоянии события
    fn info_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( req.stuff().get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( info.group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( body.scheduled_id, info.group_id ) );

        let mut answer = Answer::new();
        answer.add_record( "id", &"publication".to_string() );
        answer.add_record( "name", &body.name );
        answer.add_record( "all_count", &group_members_count );
        answer.add_record( "published", &published_photo_count );
        Ok( answer )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, _stuff: &mut Stuff, _body: &ScheduledEventInfo ) -> CommonResult<bool> {
        // публикацию досрочно заверщать не будем, есть в ожидании что-то интересное
        Ok( false )
    }
}

impl CreateFromTimetable for Publication {
    /// проверяет параметры на достоверность
    fn is_valid_params( &self, _params: &String ) -> bool {
        true
    }
    /// создаёт данные для события, возвращет None если параметры не соответствуют этому событию
    fn from_timetable( &self, group_id: Id, _params: &String ) -> Option<String> {
        let data = Info{ group_id: group_id };
        Some( json::encode( &data ).unwrap() )
    }
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( &str_body ).map_err( |e| format!( "Publication event decode error: {}", e ) )   
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    group_id: Id
}
