use super::{ Event, CreateFromTimetable, ScheduledEventInfo, make_event_action_link };
use super::late_publication::LatePublication;
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use rustc_serialize::json;
use db::mailbox::DbMailbox;
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use db::events::DbEvents;
use get_param::GetParamable;
use database::{ Databaseable };
use authentication::{ Userable };
use std::time::duration::Duration;
use time;

#[derive(Clone)]
pub struct Publication;
pub const ID : Id = 1;

impl Publication {
    pub fn new() -> Publication {
        Publication
    }
}

impl Event for Publication {
    /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, req: &mut Request, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( req.get_current_db_conn() );
        let members = try!( db.get_members( info.group_id ) );
        let sender_name = make_sender_name( &body.name );
        let subject = make_subject( &body.name );
        for user in members.iter() {
            try!( db.send_mail( 
                user.id, 
                sender_name.as_slice(), 
                subject.as_slice(), 
                make_text_body( &user.name, body ).as_slice() 
            ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, req: &mut Request, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( req.get_current_db_conn() );
        try!( db.make_publication_visible( body.scheduled_id, info.group_id ) );
        //TODO: старт голосования
        
        //старт события загрузки опоздавших
        let unpublished_users = try!( db.get_unpublished_users( body.scheduled_id, info.group_id ) );
        if unpublished_users.is_empty() == false {
            let unpublished_users_ids = unpublished_users.iter().map( |&(id, _)| id ).collect::<Vec<_>>();
            let event_info = LatePublication::create_info( 
                body.scheduled_id,
                info.group_id, 
                body.name.as_slice(), 
                time::get_time(), 
                Duration::days( 365 ), 
                unpublished_users_ids.as_slice() 
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
        let photo_id = try!( req.get_param_id( "photo" ) );
        let user = req.user().clone();
        let db = try!( req.get_current_db_conn() );
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
        let db = try!( req.get_current_db_conn() );
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
    fn is_complete( &self, _req: &mut Request, _body: &ScheduledEventInfo ) -> CommonResult<bool> {
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

fn make_sender_name( name: &String ) -> String {
    format!( "Публикация {}", name )
}

fn make_subject( name: &String ) -> String {
    format!( "Пора выкладывать {}", name )
}

fn make_text_body( user: &String, info: &ScheduledEventInfo ) -> String {
    format!( 
"Привет {}!
Настало время публиковать фотографии для '{}'.
Ты можешь сделать перейдя по вот этой ссылке: {}", 
        user,
        info.name,
        make_event_action_link( info.scheduled_id )
    )
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( str_body.as_slice() ).map_err( |e| format!( "Publication event decode error: {}", e ) )   
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    group_id: Id
}
