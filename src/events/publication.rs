use super::{
    Event,
    EventId,
    CreateFromTimetable,
    ScheduledEventInfo,
    Description,
    UserAction,
    get_group_id,
};
use super::late_publication::LatePublication;
use types::{ Id, EmptyResult, CommonResult, CommonError };
use answer::{ Answer, AnswerResult };
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use db::events::DbEvents;
use database::{ Databaseable };
use mysql::PooledConn;
use stuff::{ Stuffable, Stuff };
use authentication::{ Userable };
use time::{ self, Timespec };
use iron::prelude::*;
use get_body::GetBody;
use answer_types::{ PhotoErrorInfo };
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

        // Добавляем запись в ленту группы
        try!( db.add_to_group_feed( time::get_time().msecs(),
                                    group_id,
                                    body.scheduled_id,
                                    FeedEventState::Finish,
                                    "" ) );
        // удаляем стартовое за ненадобностью из ленты группы
        try!( db.remove_start_event_from_feed( body.scheduled_id ) );
        // подправляем время старта следующей за этой публикации
        try!( maybe_correct_next_publication_starttime( db, body ) );

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
        make_info( stuff, body, body.scheduled_id )
    }

    /// проверка на возможное досрочное завершение
    fn is_complete( &self, _stuff: &mut Stuff, _body: &ScheduledEventInfo ) -> CommonResult<bool> {
        // публикацию досрочно завершать не будем, есть в ожидании что-то интересное
        Ok( false )
    }

    /// действие которое должен осуществить пользователь
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction> {
        get_user_action( stuff, user_id, body.scheduled_id )
    }

    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        process_user_action_post( req, body.scheduled_id )
    }
}

fn maybe_correct_next_publication_starttime(
    db: &mut PooledConn,
    body: &ScheduledEventInfo
) -> EmptyResult
{
    debug!( "TRY TO UPDATE TIME!");
    match try!( db.get_next_event_after( &body.start_time, EventId::Publication ) ) {
        Some( (next_id, next_start_time) ) => {
            let calced_start_time = body.end_time + time::Duration::days( 1 );
            if calced_start_time < next_start_time {
                try!( db.update_event_start_time( next_id, &calced_start_time ) );
                debug!( "updating time for {}", next_id );
            }
            else {
                debug!( "No need to update time for {}", next_id );
            }
        },
        None => {}
    }
    Ok( () )
}

#[derive(RustcEncodable, Debug)]
pub struct PublicationInfo {
    id: EventId,
    name: String,
    all_count: u32,
    published: u32
}

pub fn make_info( stuff: &mut Stuff, body: &ScheduledEventInfo, scheduled_id: Id ) -> CommonResult<Description> {
    let group_id = try!( get_group_id( body ) );
    let db = try!( stuff.get_current_db_conn() );
    let group_members_count = try!( db.get_members_count( group_id ) );
    let published_photo_count = try!( db.get_published_photo_count( scheduled_id ) );

    let desc = Description::new( PublicationInfo {
        id: ID,
        name: body.name.clone(),
        all_count: group_members_count,
        published: published_photo_count
    } );
    Ok( desc )
}

pub fn get_user_action( stuff: &mut Stuff, user_id: Id, scheduled_id: Id ) -> CommonResult<UserAction> {
    let db = try!( stuff.get_current_db_conn() );
    let is_unpublished = try!( db.is_unpublished_user( scheduled_id, user_id ) );
    let action = match is_unpublished {
        true => UserAction::Publication,
        false => UserAction::None
    };
    Ok( action )
}

pub fn process_user_action_post( req: &mut Request, scheduled_id: Id ) -> AnswerResult {
    let photo_id = try!( req.get_body::<PhotoInfo>() ).id;
    match process_publish_photo( req, scheduled_id, photo_id ) {
        Ok( _ ) => Ok( Answer::good( "published" ) ),
        Err( e ) => e.into()
    }
}

pub enum PublishError
{
    Common( CommonError ),
    Answer( Answer )
}

impl PublishError {
    pub fn err( a: Answer ) -> Result<(), PublishError> {
        Err( PublishError::Answer( a ) )
    }
}

impl From<CommonError> for PublishError {
    fn from( e: CommonError ) -> PublishError {
        PublishError::Common( e )
    }
}

impl Into<AnswerResult> for PublishError {
    fn into( self ) -> AnswerResult {
        match self {
            PublishError::Common( ce ) => Err( ce ),
            PublishError::Answer( a ) => Ok( a )
        }
    }
}

pub fn process_publish_photo( req: &mut Request, scheduled_id: Id, photo_id: Id ) -> Result<(), PublishError>
{
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );

    //FIXME: как-то надо облагородить такую вложенность
    match try!( db.get_short_photo_info( photo_id ) ) {
        Some( photo_info ) => {
            if photo_info.owner.id == user_id {
                match try!( db.is_photo_published( photo_id ) ) {
                    false => {
                        let prev = try!( db.get_last_pubcation_photo( scheduled_id ) );
                        try!( db.public_photo( scheduled_id,
                                               user_id,
                                               photo_id,
                                               time::get_time().msecs(),
                                               prev.clone() ) );
                        if let Some( last_id ) = prev {
                            try!( db.set_next_publication_photo( last_id, photo_id ) );
                        }
                        Ok( () )
                    },
                    true => PublishError::err( Answer::bad( PhotoErrorInfo::already_published() ) )
                }
            }
            else {
                PublishError::err( Answer::access_denied() )
            }
        },
        None => PublishError::err( Answer::not_found() )
    }
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
