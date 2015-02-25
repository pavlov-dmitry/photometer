use std::collections::HashSet;
use rustc_serialize::json;
use std::error::FromError;
use std::str::FromStr;
use iron::prelude::*;

use super::{ 
    Event, 
    ScheduledEventInfo, 
    UserEvent, 
    FullEventInfo
};
use err_msg;
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use get_param::GetParamable;
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use db::votes::DbVotes;
use mailer::Mailer;
use db::users::DbUsers;
use db::groups::DbGroups;
use authentication::{ Userable };
use time;
use std::time::Duration;
use mail_writer::MailWriter;

#[derive(Clone)]
pub struct GroupCreation;
pub const ID : Id = 2;

impl GroupCreation {
    pub fn new() -> GroupCreation {
        GroupCreation
    }
}

static MEMBERS: &'static str = "members";
type Members = HashSet<Id>;

impl UserEvent for GroupCreation {
    /// описание создания
    fn user_creating_get( &self, _req: &mut Request ) -> AnswerResult {
        let mut answer = Answer::new();
        answer.add_record( "edit_event", &ID );
        Ok( answer )
    }
    /// применение создания
    fn user_creating_post( &self, req: &mut Request ) -> Result<FullEventInfo, AnswerResult> {
        let group_name = try!( req.get_param( "name" ) ).to_string();
        let mut answer = Answer::new();

        let mut info = Info {
            initiator: req.user().id,
            members: HashSet::new(),
            name: group_name.clone(),
            description: try!( req.get_param( "description" ) ).to_string()
        };
        //конвертация идентификаторов из строк
        {
            let members_str = try!( req.get_params( MEMBERS ) );
            for member_str in members_str.iter() {
                let member = try!( convert_member( member_str ) );
                if member != info.initiator {
                    info.members.insert( member );
                }
            }
        }
        if info.members.is_empty() {
            answer.add_error( "members", "not_found" );
            return Err( Ok( answer ) );
        }
        // проверка наличия пользователей
        let db = try!( req.stuff().get_current_db_conn() );
        for member in info.members.iter() {
            if try!( db.user_id_exists( *member ) ) == false {
                answer.add_error( "user", "not_found" );
                return Err( Ok( answer ) );
            }
        }
        //формирование 
        let start_time = time::get_time();
        let end_time = start_time + Duration::days( 1 );
        Ok( FullEventInfo {
            id: ID,
            name: group_name,
            start_time: start_time,
            end_time: end_time,
            data: json::encode( &info ).unwrap()
        })
    }
}

impl Event for GroupCreation {
    /// идентификатор события
    fn id( &self ) -> Id {
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
        let (subject, mail) = stuff.write_group_creation_mail( &info.name, body.scheduled_id );
        for member in exists_members.iter() {
            try!( stuff.send_mail( member, &subject, &mail ) );
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult {
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
                let users = {
                    let db = try!( stuff.get_current_db_conn() );
                    // создаём группу
                    let group_id = try!( db.create_group( &info.name, &info.description ) );
                    // и тех кто проголовал ЗА добавляем в эту группу
                    votes.yes.push( info.initiator );
                    try!( db.add_members( group_id, &votes.yes ) );
                    try!( db.users_by_id( &votes.yes ) )
                };
                // рассылаем письма что группа создана
                let (subject, mail) = stuff.write_welcome_to_group_mail( &info.name );
                for member in users.iter() {
                    try!( stuff.send_mail( member, &subject, &mail ) );
                }
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
    /// описание действия пользователя на это событие 
    fn user_action_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        //TODO: Согласовать с Саньком, что именно ему здесь надо отсылать
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );
        let is_need_vote = try!( db.is_need_user_vote( body.scheduled_id, user_id ) );
        
        let mut answer = Answer::new();
        if is_need_vote {
            answer.add_record( "user", &"need_some_voting".to_string() );    
        } else {
            answer.add_record( "user", &"no_need_vote".to_string() );
        }
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let vote: bool = try!( req.get_param( "vote" ) ) == "yes";
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );
        let is_need_vote = try!( db.is_need_user_vote( body.scheduled_id, user_id ) );

        let mut answer = Answer::new();
        if is_need_vote {
            try!( db.set_vote( body.scheduled_id, user_id, vote ) );
            answer.add_record( "vote", &"accepted".to_string() );
        }
        else {
            answer.add_error( "user", "no_need_vote" );
        }
        Ok( answer )
    }
    /// информация о состоянии события
    fn info_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let db = try!( req.stuff().get_current_db_conn() );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        let mut answer = Answer::new();
        answer.add_record( "name", &info.name );
        answer.add_record( "description", &info.description );
        answer.add_record( "all_count", &votes.all_count );
        answer.add_record( "yes", &votes.yes.len() );
        answer.add_record( "no", &votes.no.len() );
        Ok( answer )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        //досрочно завершается когда все проголосвали
        let db = try!( stuff.get_current_db_conn() );
        db.is_all_voted( body.scheduled_id )
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Info {
    initiator: Id,
    members: HashSet<Id>,
    name: String,
    description: String
}

impl FromError<String> for AnswerResult {
    fn from_error( err: String ) -> AnswerResult {
        Err( err )
    }
}

fn convert_member( s: &String ) -> CommonResult<Id> {
    match FromStr::from_str( &s ).ok() {
        Some( id ) => Ok( id ),
        None => Err( err_msg::invalid_type_param( MEMBERS ) )
    }
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( &str_body ).map_err( |e| format!( "GroupCreation event data decode error: {}", e ) )   
}