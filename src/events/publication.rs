use super::{
    Event,
    EventId,
    CreateFromTimetable,
    ScheduledEventInfo,
    Description,
    UserAction,
    get_group_id,
    helpers
};
use super::late_publication::LatePublication;
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use db::events::DbEvents;
use mail_writer::MailWriter;
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use authentication::{ Userable };
use time::{ self, Timespec };
use iron::prelude::*;
use get_body::GetBody;
use answer_types::{ OkInfo, PhotoErrorInfo };
use db::group_feed::DbGroupFeed;
use super::feed_types::FeedEventState;
use parse_utils::{ GetMsecs };

#[derive(Clone)]
pub struct Publication;
pub const ID: EventId = EventId::Publication;

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
    fn id( &self ) -> EventId {
        ID
    }

    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let group_id = try!( get_group_id( body ) );
        try!( helpers::send_to_group( stuff, group_id, &mut |stuff, user| {
            stuff.write_time_for_publication_mail( &body.name,
                                                   &user.name,
                                                   body.scheduled_id )
        }));

        // Добавляем запись в ленту группы
        let db = try!( stuff.get_current_db_conn() );
        try!( db.add_to_group_feed( time::get_time().msecs(),
                                    group_id,
                                    body.scheduled_id,
                                    FeedEventState::Start,
                                    "" ) );
        Ok( () )
    }

    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let group_id = try!( get_group_id( body ) );
        let db = try!( stuff.get_current_db_conn() );
        try!( db.make_publication_visible( body.scheduled_id ) );

        // Добавляем запись в ленту группы
        try!( db.add_to_group_feed( time::get_time().msecs(),
                                    group_id,
                                    body.scheduled_id,
                                    FeedEventState::Finish,
                                    "" ) );

        //TODO: старт голосования

        //старт события загрузки опоздавших
        //FIXME: использовать более "дешевую" функцию для определения что есть отставшие
        let unpublished_users = try!( db.get_unpublished_users( body.scheduled_id ) );
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
        let published_photo_count = try!( db.get_published_photo_count( body.scheduled_id ) );

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
        // публикацию досрочно завершать не будем, есть в ожидании что-то интересное
        Ok( false )
    }

    /// действие которое должен осуществить пользователь
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction> {
        let db = try!( stuff.get_current_db_conn() );
        let is_unpublished = try!( db.is_unpublished_user( body.scheduled_id,
                                                           user_id ) );
        let action = match is_unpublished {
            true => UserAction::Publication,
            false => UserAction::None
        };
        Ok( action )
    }

    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let photo_id = try!( req.get_body::<PhotoInfo>() ).id;
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );

        let answer = match try!( db.get_photo_info( photo_id ) ) {
            Some( photo_info ) => {
                if photo_info.owner.id == user_id {
                    match try!( db.is_photo_published( photo_id ) ) {
                        false => {
                            try!( db.public_photo( body.scheduled_id,
                                                   user_id,
                                                   photo_id,
                                                   false,
                                                   time::get_time().msecs() ) );
                            Answer::good( OkInfo::new( "published" ) )
                        },
                        true => Answer::bad( PhotoErrorInfo::already_published() )
                    }
                }
                else {
                    Answer::access_denied()
                }
            },
            None => Answer::not_found()
        };
        Ok( answer )
    }
}

#[derive(RustcEncodable, Debug)]
struct PublicationInfo {
    id: EventId,
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
    /// выдаёт пару начала и окончания по времени в зависимости от времени события
    fn time_gate( &self, time: &Timespec ) -> (Timespec, Timespec) {
        (*time - time::Duration::days( 1 ), *time)
    }
}
