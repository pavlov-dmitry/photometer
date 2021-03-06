use std::collections::HashSet;
use rustc_serialize::json;
use iron::prelude::*;

use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    UserCreatedEvent,
    FullEventInfo,
    Description,
    UserAction,
};
use super::helpers;
use types::{ Id, EmptyResult, CommonResult, CommonError };
use answer::{ Answer, AnswerResult };
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use db::votes::DbVotes;
use mailer::Mailer;
use db::users::DbUsers;
use db::groups::DbGroups;
use db::group_feed::DbGroupFeed;
use authentication::{ Userable };
use time;
use mail_writer::MailWriter;
use get_body::GetBody;
use answer_types::{ FieldErrorInfo };
use events::feed_types::FeedEventState;
use parse_utils::GetMsecs;

#[derive(Clone)]
pub struct GroupCreation;
pub const ID : EventId = EventId::GroupCreation;

impl GroupCreation {
    pub fn new() -> GroupCreation {
        GroupCreation
    }
}

type Members = HashSet<Id>;

#[derive(Clone, RustcDecodable)]
struct Member {
    id: Id
}

#[derive(Clone, RustcDecodable)]
struct GroupInfo {
    name: String,
    description: String,
    members: Vec<Member>
}

#[derive(RustcEncodable)]
struct EditEventInfo {
    edit_event: EventId
}

static NAME : &'static str = "name";
static DESCRIPTION : &'static str = "description";

impl UserCreatedEvent for GroupCreation {
    /// описание создания
    fn user_creating_get( &self, _req: &mut Request ) -> AnswerResult {
        let answer = Answer::good( EditEventInfo {
            edit_event: ID
        } );
        Ok( answer )
    }
    /// применение создания
    fn user_creating_post( &self, req: &mut Request ) -> Result<FullEventInfo, AnswerResult> {
        let group_info = try!( req.get_body::<GroupInfo>() );
        let user_id = req.user().id;

        let mut info = Info {
            initiator: user_id,
            members: HashSet::new(),
            name: group_info.name.clone(),
            description: group_info.description.clone()
        };

        let mut errors = Vec::new();
        if group_info.name.is_empty() {
            errors.push( FieldErrorInfo::empty( NAME ) );
        }
        if group_info.description.is_empty() {
            errors.push( FieldErrorInfo::empty( DESCRIPTION ) );
        }
        if 64 < group_info.name.chars().count() {
            errors.push( FieldErrorInfo::too_long( NAME ) );
        }
        if 2048 < group_info.description.chars().count() {
            errors.push( FieldErrorInfo::too_long( DESCRIPTION ) );
        }

        // проверяем что такой группы нет
        let group_exist = {
            let db = try!( req.stuff().get_current_db_conn() );
            try!( db.is_group_exists( &group_info.name ) )
        };
        if group_exist {
            errors.push( FieldErrorInfo::new( "group", "exists" ) );
        }

        // проверка наличия пользователей
        let db = try!( req.stuff().get_current_db_conn() );
        for member in group_info.members.iter() {
            let is_exists = try!( db.user_id_exists( member.id ) );
            match is_exists {
                true => {
                    info.members.insert( member.id );
                },
                false => {
                    let id_str = format!("{}", member.id);
                    errors.push( FieldErrorInfo::not_found( &id_str ) )
                }
            }
        }

        // если вдруг решили приглясить себя, то просто удаляем и списка приглашенных
        info.members.remove( &user_id );

        if info.members.is_empty() {
            errors.push( FieldErrorInfo::not_found( "members" ) );
        }

        // если ошибок в запросе не найдено, то запрос валиден
        if errors.is_empty() == false {
            let answer = Answer::bad( errors );
            return Err( Ok( answer ) );
        }

        //формирование
        let start_time = time::get_time();
        let end_time = start_time + time::Duration::days( 3 );
        Ok( FullEventInfo {
            id: ID,
            name: group_info.name,
            start_time: start_time,
            end_time: end_time,
            data: json::encode( &info ).unwrap(),
            group: None,
            creator: Some( user_id )
        })
    }
}

impl Event for GroupCreation {
    /// идентификатор события
    fn id( &self ) -> EventId {
        ID
    }
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );

        let exists_members = {
            let db = try!( stuff.get_current_db_conn() );
            //пока преобразовываю в vec, до тех пор пока не сделают нормальную передачу итераторов
            let members : Vec<_> = info.members.iter().cloned().collect();
            try!( db.users_by_id( &members ) )
        };

        let exists_ids : Vec<_> = exists_members.iter()
            .map( |m| m.id )
            .collect();

        // даём право голоса пользователям
        {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.add_rights_of_voting( body.scheduled_id, &exists_ids ) );
        }
        // рассылаем письма что можно голосовать
        let (subject, mail) = stuff.write_group_invite_mail( &info.name, body.scheduled_id );
        for member in exists_members.iter() {
            try!( stuff.send_mail( member, &subject, &mail ) );
        }
        let (subject, mail) = stuff.write_group_creation_started_mail( &info.name, body.scheduled_id );
        // посылаем письмо тому кто создавал группу, с сообщением что всех пригласили.
        let maybe_user = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.user_by_id( info.initiator ) )
        };
        if let Some( initiator_user ) = maybe_user {
            try!( stuff.send_mail( &initiator_user, &subject, &mail ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
        //FIXME: что-то великовата функция, надо бы разбить и упростить
        let info = try!( get_info( &body.data ) );
        // собиарем голоса
        let mut votes = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.get_votes( body.scheduled_id ) )
        };
        // проверяем что такой группы нет
        let group_exist = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.is_group_exists( &info.name ) )
        };
        if group_exist == false {
            // елси хоть кто-то решил присоединиться
            if votes.yes.is_empty() == false {
                let ( group_id, users ) = {
                    let db = try!( stuff.get_current_db_conn() );
                    // создаём группу
                    let group_id = try!( db.create_group( &info.name,
                                                          &info.description,
                                                          time::get_time() ) );
                    // и тех кто проголовал ЗА добавляем в эту группу
                    votes.yes.push( info.initiator );
                    try!( db.add_members( group_id, &votes.yes, time::get_time().msecs() ) );
                    let users = try!( db.users_by_id( &votes.yes ) );
                    (group_id, users)
                };
                // рассылаем письма что группа создана
                let (subject, mail) = stuff.write_welcome_to_group_mail( &info.name );
                for member in users.iter() {
                    try!( stuff.send_mail( member, &subject, &mail ) );
                }
                // добавляем первую запись в ленту группы
                let db = try!( stuff.get_current_db_conn() );
                try!( db.add_to_group_feed( time::get_time().msecs(),
                                            group_id,
                                            body.scheduled_id,
                                            FeedEventState::Finish,
                                            "" ) );
            }
            else {
                // отсылаем жалостливое письмо что никто в твою группу не хочет, если он еще здесь
                let maybe_user = {
                    let db = try!( stuff.get_current_db_conn() );
                    try!( db.user_by_id( info.initiator ) )
                };
                if let Some( user ) = maybe_user {
                    let (subject, mail) = stuff.write_nobody_need_your_group_mail( &info.name );
                    try!( stuff.send_mail( &user, &subject, &mail ) );
                }
            }
        }
        else {
            // отсылаем письмо что группа с таким именем уже созданна и надо поменять ей имя
            votes.yes.push( info.initiator );
            let users = {
                let db = try!( stuff.get_current_db_conn() );
                try!( db.users_by_id( &votes.yes ) )
            };
            let (subject, mail) = stuff.write_group_name_already_exists_mail( &info.name );
            for user in users.iter() {
                try!( stuff.send_mail( user, &subject, &mail ) );
            }
        }
        Ok( () )
    }
    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let info = try!( get_info( &body.data ) );
        let db = try!( stuff.get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );

        let desc = Description::new( GroupCreationInfo {
            description: info.description.clone(),
            all_count: votes.all_count,
            yes: votes.yes.len(),
            no: votes.no.len()
        } );
        Ok( desc )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        //досрочно завершается когда все проголосвали
        let db = try!( stuff.get_current_db_conn() );
        db.is_all_voted( body.scheduled_id )
    }
    /// действие которое должен осуществить пользователь
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction> {
        helpers::get_action_by_vote( stuff, body.scheduled_id, user_id )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        helpers::set_user_vote( req, body )
    }
}

#[derive(RustcEncodable, Debug)]
struct GroupCreationInfo {
    description: String,
    all_count: usize,
    yes: usize,
    no: usize
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    initiator: Id,
    members: HashSet<Id>,
    name: String,
    description: String
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( &str_body )
        .map_err( |e| CommonError( format!( "GroupCreation event data decode error: {}", e ) ) )
}
