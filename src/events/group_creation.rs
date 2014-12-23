use std::collections::HashSet;
use serialize::json;
use std::error::FromError;

use super::{ Event, ScheduledEventInfo, UserEvent, make_event_action_link };
use err_msg;
use types::{ Id, EmptyResult, CommonResult };
use nickel::{ Request };
use answer::{ Answer, AnswerResult };
use get_param::GetParamable;
use database::DbConnection;
use db::votes::DbVotes;
use db::mailbox::DbMailbox;
use db::users::DbUsers;
use db::groups::DbGroups;
use authentication::Userable;

#[deriving(Clone)]
pub struct GroupCreation;

impl GroupCreation {
    pub fn new() -> GroupCreation {
        GroupCreation
    }
}

const ID : Id = 2;
static MEMBERS: &'static str = "members";
static SENDER_NAME: &'static str = "Создание группы";
type Members = HashSet<Id>;

impl UserEvent for GroupCreation {
    /// описание создания
    fn user_creating_get( &self, _request: &Request ) -> AnswerResult {
        let mut answer = Answer::new();
        answer.add_record( "edit_event", &ID );
        Ok( answer )
    }
    /// применение создания
    fn user_creating_post( &self, db: &mut DbConnection, req: &Request ) -> Result<String, AnswerResult> {
        let members_str = try!( req.get_params( MEMBERS ) );
        let mut answer = Answer::new();

        let mut info = Info {
            initiator: req.user().id,
            members: HashSet::new(),
            name: try!( req.get_param( "name" ) ).to_string(),
            description: try!( req.get_param( "description" ) ).to_string()
        };
        //конвертация идентификаторов из строк
        for member_str in members_str.iter() {
            info.members.insert( try!( convert_member( member_str ) ) );
        }
        // проверка наличия пользователей
        for member in info.members.iter() {
            if try!( db.user_id_exists( *member ) ) == false {
                answer.add_error( "user", "not_found" );
                return Err( Ok( answer ) );
            }
        }
        //формирование 
        Ok( json::encode( &info ) )
    }
}

impl Event for GroupCreation {
    /// идентификатор события
    fn id( &self ) -> Id {
        ID
    }
    /// действие на начало события
    fn start( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );

        let mut exists_members = Vec::new();
        for member in info.members.iter() {
            if try!( db.user_id_exists( *member ) ) { // filter итератора нельзя использовать так как это делается через db
                exists_members.push( *member );
            }
        }

        // даём право голоса пользователям
        try!( db.add_rights_of_voting( body.scheduled_id, exists_members.as_slice() ) );
        // тот кто создаёт группу по умолчанию проголосовал ЗА!
        try!( db.set_vote( body.scheduled_id, info.initiator, true ) );
        // рассылаем письма что можно голосовать
        for member in exists_members.iter() {
            try!( db.send_mail( 
                *member,  
                SENDER_NAME,
                make_mail_subject( &info.name ).as_slice(),
                make_mail_body( &info.name, body.scheduled_id ).as_slice()
            ));
        }
        Ok( () )
    }
    /// действие на окончание события
    fn finish( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( &body.data ) );
        // собиарем голоса
        let votes = try!( db.get_votes( body.scheduled_id ) );
        // проверяем что такой группы нет
        if try!( db.is_group_exists( &info.name ) ) == false {
            // елси хоть кто-то решил присоединиться
            if 1 < votes.yes.len() {
                // создаём группу
                let group_id = try!( db.create_group( &info.name, &info.description ) );
                // и тех кто проголовал ЗА добавляем в эту группу
                try!( db.add_members( group_id, votes.yes.as_slice() ) );
            }
            else {
                // отсылаем жалостливое письмо что никто в твою группу не хочет
                try!( send_mail_nobody_need_your_group( db, &info ) );
            }            
        }
        else {
            // отсылаем письмо что группа с таким именем уже созданна и надо поменять ей имя
            for member_id in votes.yes.iter() {
                try!( send_mail_group_name_already_exists( db, *member_id, &info.name ) );
            }
        }
        Ok( () )
    }
    /// описание действия пользователя на это событие 
    fn user_action_get( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult {
        //TODO: Согласовать с Саньком, что именно ему здесь надо отсылать
        let is_already_voted = try!( db.is_user_already_voted( body.scheduled_id, request.user().id ) );
        let mut answer = Answer::new();
        if is_already_voted {
            answer.add_error( "user", "alredy_voted" );
        }
        else {
            answer.add_record( "group_creation", &"edit".to_string() );    
        }
        Ok( answer )
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let vote: bool = try!( request.get_param( "vote" ) ) == "yes";
        let is_already_voted = try!( db.is_user_already_voted( body.scheduled_id, request.user().id ) );

        let mut answer = Answer::new();
        if is_already_voted == false {
            try!( db.set_vote( body.scheduled_id, request.user().id, vote ) );
            answer.add_record( "vote", &"accepted".to_string() );
        }
        else {
            answer.add_error( "user", "alredy_voted" );
        }
        Ok( answer )
    }
    /// информация о состоянии события
    fn info_get( &self, db: &mut DbConnection, _request: &Request, body: &ScheduledEventInfo ) -> AnswerResult {
        let info = try!( get_info( &body.data ) );
        let votes = try!( db.get_votes( body.scheduled_id ) );
        let mut answer = Answer::new();
        answer.add_record( "name", &info.name );
        answer.add_record( "description", &info.name );
        answer.add_record( "all_count", &votes.all_count );
        answer.add_record( "yes", &votes.yes.len() );
        answer.add_record( "no", &votes.no.len() );
        Ok( answer )
    }
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        //досрочно завершается когда все проголосвали
        db.is_all_voted( body.scheduled_id )
    }
}

#[deriving(Encodable, Decodable)]
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
    match ::std::str::from_str::<Id>( s.as_slice() ) {
        Some( id ) => Ok( id ),
        None => Err( err_msg::invalid_type_param( MEMBERS ) )
    }
}

fn get_info( str_body: &String ) -> CommonResult<Info> {
    json::decode( str_body.as_slice() ).map_err( |e| format!( "GroupCreation event data decode error: {}", e ) )   
}

fn make_mail_subject( name: &String ) -> String {
    format!( "Создание новой группы `{}`", name )
}

fn make_mail_body( name: &String, scheduled_id: Id ) -> String {
    format!( "Вас приглашают создать новую группу `{}`.
        Узнать подробности и принять решение о присоединении вы можете пройдя по этой ссылке {}.
        У вас есть неделя чтобы принять решение.",
        name,
        make_event_action_link( scheduled_id ) )
}

fn send_mail_nobody_need_your_group( db: &mut DbConnection, info: &Info ) -> EmptyResult {
    db.send_mail(
        info.initiator,
        SENDER_NAME,
        format!( "Группа '{}' не создана", info.name ).as_slice(),
        "К сожалению ни один из приглашенных вами пользователей не согласился создать группу."
    )
}

fn send_mail_group_name_already_exists( db: &mut DbConnection, id: Id, name: &String ) -> EmptyResult {
    db.send_mail(
        id,
        SENDER_NAME,
        format!( "Группа с именем '{}' уже существует", name ).as_slice(),
        format!( "Группа с именем '{}' была уже создана за время пока вы решали создавать вашу группу или нет. 
            Создайте новую группу с другим именем или присоединитесь к существующей", name ).as_slice()
    )
}