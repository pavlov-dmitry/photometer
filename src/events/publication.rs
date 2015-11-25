use super::{
    Event,
    CreateFromTimetable,
    ScheduledEventInfo,
    Description,
    UserAction,
    get_group_id
};
use super::late_publication::LatePublication;
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use mailer::Mailer;
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use db::events::DbEvents;
use mail_writer::MailWriter;
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use authentication::{ Userable };
use time;
use iron::prelude::*;
use get_body::GetBody;
use answer_types::{ OkInfo, PhotoErrorInfo, AccessErrorInfo };

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
        let group_id = try!( get_group_id( body ) );
        let members = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_members( group_id ) )
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
        let group_id = try!( get_group_id( body ) );
        let db = try!( stuff.get_current_db_conn() );
        try!( db.make_publication_visible( body.scheduled_id, group_id ) );
        //TODO: старт голосования

        //старт события загрузки опоздавших
        //FIXME: использовать более "дешевую" функцию для определения что есть отставшие
        let unpublished_users = try!( db.get_unpublished_users( body.scheduled_id, group_id ) );
        if unpublished_users.is_empty() == false {
            let event_info = LatePublication::create_info(
                body.scheduled_id,
                group_id,
                &body.name,
                time::get_time(),
                time::Duration::days( 365 )
            );
            try!( db.add_events( &[ event_info ] ) );
        }

        Ok( () )
    }

    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let group_id = try!( get_group_id( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( body.scheduled_id, group_id ) );

        let desc = Description::new( PublicationInfo {
            id: ID,
            name: body.name.clone(),
            all_count: group_members_count,
            published: published_photo_count
        } );
        Ok( desc )
    }

    /// проверка на возможное досрочное завершение
    fn is_complete( &self, _stuff: &mut Stuff, _body: &ScheduledEventInfo ) -> CommonResult<bool> {
        // публикацию досрочно заверщать не будем, есть в ожидании что-то интересное
        Ok( false )
    }

    /// действие которое должен осуществить пользователь
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction> {
        let group_id = try!( get_group_id( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let is_unpublished = try!( db.is_unpublished_user( body.scheduled_id,
                                                           group_id,
                                                           user_id ) );
        let action = if is_unpublished { UserAction::Publication } else { UserAction::None };
        Ok( action )
    }

    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let group_id = try!( get_group_id( body ) );
        let photo_id = try!( req.get_body::<PhotoInfo>() ).id;
        let user = req.user().clone();
        let db = try!( req.stuff().get_current_db_conn() );

        let answer = {
            let photo_info = try!( db.get_photo_info( photo_id ) );
            if let Some( (user_name, _) ) = photo_info {
                if user_name == user.name {
                    try!( db.public_photo( body.scheduled_id,
                                           group_id,
                                           user.id,
                                           photo_id,
                                           false ) );
                    Answer::good( OkInfo::new( "published" ) )
                }
                else {
                    Answer::bad( AccessErrorInfo::new() )
                }
            }
            else {
                Answer::bad( PhotoErrorInfo::not_found() )
            }
        };
        Ok( answer )
    }
}

#[derive(RustcEncodable, Debug)]
struct PublicationInfo {
    id: Id,
    name: String,
    all_count: u32,
    published: u32
}

impl CreateFromTimetable for Publication {
    /// проверяет параметры на достоверность
    fn is_valid_params( &self, params: &String ) -> bool {
        params.is_empty()
    }
    /// создаёт данные для события, возвращет None если параметры не соответствуют этому событию
    fn from_timetable( &self, _group_id: Id, _params: &String ) -> Option<String> {
        Some( String::new() )
    }
}
