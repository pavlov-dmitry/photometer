use super::{ Event };
use types::{ Id, EmptyResult, EventInfo, CommonResult };
use answer::{ Answer, AnswerResult };
use serialize::json;
use std::error::FromError;
use db::mailbox::DbMailbox;
use db::groups::DbGroups;
use db::publication::DbPublication;
use db::photos::DbPhotos;
use get_param::GetParamable;
use database::{ DbConnection };
use nickel::{ Request };
use authentication::{ Userable, User };

pub struct Publication;

impl Publication {
    pub fn new() -> Publication {
        Publication
    }
}

const ID : Id = 1;

impl Event for Publication {
        /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, db: &mut DbConnection, body: &EventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        let members = try!( db.get_members( info.group_id ) );
        let sender_name = make_sender_name( &body.name );
        let subject = make_subject( &body.name );
        for user in members.iter() {
            try!( db.send_mail( 
                user.id, 
                sender_name.as_slice(), 
                subject.as_slice(), 
                make_text_body( &user.name, &info, body ).as_slice() 
            ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, db: &mut DbConnection, body: &EventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        try!( db.make_publication_visible( body.scheduled_id, info.group_id ) );
        //TODO: старт голосования
        //TODO: старт загрузки опоздавших
        Ok( () )
    }
    /// описание действиz пользователя на это событие 
    fn user_action_get( &self, db: &mut DbConnection, request: &Request, body: &EventInfo ) -> AnswerResult {
        let mut answer = Answer::new();
        // TODO: переделать на нормальное отдачу, поговорить с Саньком, что ему нужно в этот момент
        answer.add_record( "choose", &"from_gallery".to_string() );
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, db: &mut DbConnection, request: &Request, body: &EventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let photo_id = try!( request.get_param_i64( "photo" ) );
        let user = request.user();
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
    fn info_get( &self, db: &mut DbConnection, request: &Request, body: &EventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
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
    fn is_complete( &self, db: &mut DbConnection, body: &EventInfo ) -> CommonResult<bool> {
        // публикацию досрочно заверщать не будем, есть в ожидании что-то интересное
        Ok( false )
    }
}

fn make_sender_name( name: &String ) -> String {
    format!( "Публикация {}", name )
}

fn make_subject( name: &String ) -> String {
    format!( "Пора выкладывать {}", name )
}

fn make_text_body( user: &String, group: &Info, info: &EventInfo ) -> String {
    format!( 
        "Привет {}!
        Настало время публиковать фотографии для {}.
        Ты можешь сделать перейдя по вот этой ссылке: {}
        ", 
        user,
        info.name,
        make_link( info.scheduled_id )
    )
}

fn make_link( id: Id ) -> String {
    format!( "/event/action/{}", id )
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( str_body.as_slice() ).map_err( |e| format!( "Publication event decode error: {}", e ) )   
}

#[deriving(Encodable, Decodable)]
struct Info {
    group_id: Id
}