use super::{ Event, ScheduledEventInfo, FullEventInfo, make_event_action_link };
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use database::{ Databaseable };
use db::mailbox::DbMailbox;
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use std::time;
use time::Timespec;
use rustc_serialize::json;
use authentication::Userable;
use get_param::GetParamable;

#[derive(Clone)]
pub struct LatePublication;
pub const ID : Id = 3;

impl LatePublication {
    pub fn new() -> LatePublication {
        LatePublication
    }

    pub fn create_info( parent_id: Id, group_id: Id, name: &str, start_time: Timespec, duration: time::Duration, late_users: &[Id] ) -> FullEventInfo {
        let info = Info {
            group_id: group_id,
            parent_id: parent_id,
            late_users: late_users.to_vec()
        };
        FullEventInfo {
            id: ID,
            name: String::from_str( "Догоняем " ) + name,
            start_time: start_time,
            end_time: start_time + duration,
            data: json::encode( &info ).unwrap()
        }
    }
}

impl Event for LatePublication {
    /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, req: &mut Request, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        for user in info.late_users.iter() {
            try!( send_mail_you_can_public_photos( req, *user, body, &info ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, _req: &mut Request, _body: &ScheduledEventInfo ) -> EmptyResult {
        // нечего тут делать
        Ok( () )
    }
    /// описание действиz пользователя на это событие 
    fn user_action_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let mut answer = Answer::new();
        // если такой пользователь есть должен выложиться
        if info.late_users.iter().any( |c| *c == req.user().id ) {
            // TODO: переделать на нормальное отдачу, поговорить с Саньком, что ему нужно в этот момент
            answer.add_record( "choose", &"from_gallery".to_string() );
        }
        else {
            answer.add_record( "user", &"nothing_to_do".to_string() );
        }
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let photo_id = try!( req.get_param_id( "photo" ) );
        let current_user_name = req.user().name.clone();
        let user_id = req.user().id;
        let db = try!( req.get_current_db_conn() );

        let mut answer = Answer::new();
        // если такой пользователь есть должен выложиться
        if info.late_users.iter().any( |c| *c == user_id ) {
            if let Some( (user_name, _) ) = try!( db.get_photo_info( photo_id ) ) {
                if user_name == current_user_name {
                    try!( db.public_photo( info.parent_id, info.group_id, user_id, photo_id, true ) );
                    answer.add_record( "published", &"ok".to_string() );
                }
                else {
                    answer.add_error( "permisson", "denied" );
                }
            }
            else {
                answer.add_error( "photo", "not_found" );
            }
        }
        else {
            answer.add_error( "user", "nothing_to_do" );
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
    fn is_complete( &self, req: &mut Request, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        let info = try!( get_info( &body.data ) );
        let db = try!( req.get_current_db_conn() );
        let group_members_count = try!( db.get_members_count( info.group_id ) );
        let published_photo_count = try!( db.get_published_photo_count( body.scheduled_id, info.group_id ) ); 
        Ok( group_members_count == published_photo_count )
    }
}

static SENDER_NAME: &'static str = "Публикация с опозданием";

fn send_mail_you_can_public_photos( req: &mut Request, user: Id, body: &ScheduledEventInfo, _info: &Info ) -> EmptyResult {
    let db = try!( req.get_current_db_conn() );
    db.send_mail(
        user,
        SENDER_NAME,
        format!( "{} '{}'", SENDER_NAME, body.name ).as_slice(),
        format!( 
"Ну что не получилось вовремя опубликовать свою фотографию? Ну ничего, не растраивайся!
Ты всё равно можешь это сделать по вот этой ссылке {}. Возможно она уже не будет участвовать в конкурсах,
но хотя бы не будет этого Гомера.",
            make_event_action_link( body.scheduled_id )
        ).as_slice()
    )
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    group_id: Id,
    parent_id: Id,
    late_users:Vec<Id>
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( str_body.as_slice() )
        .map_err( |e| format!( "LatePublication event decode error: {}", e ) )   
}
