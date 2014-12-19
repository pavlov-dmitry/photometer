use super::{ Event, ScheduledEventInfo, make_event_action_link };
use types::{ Id, EmptyResult, CommonResult };
use answer::{ Answer, AnswerResult };
use get_param::GetParamable;
use database::DbConnection;
use nickel::QueryString;
use db::votes::DbVotes;
use db::mailbox::DbMailbox;

#[deriving(Clone)]
struct GroupCreation;

impl GroupCreation {
    fn new() -> GroupCreation {
        GroupCreation
    }
}

const ID : Id = 2;
static MEMBERS: &'static str = "members";
type Members = HashSet<id>;

impl UserEvent for GroupCreation {
    /// описание создания
    fn user_creating_get( &self, request: &Request ) -> AnswerResult {
        let answer = Answer::new();
        answer.add_record( "edit_event", &ID );
        Err( answer )
    }
    /// применение создания
    fn user_creating_post( &self, db: &mut DbConnection, req: &Request ) -> Result<String, AnswerResult> {
        let members_str = req.query( MEMBERS, "" );
        let answer = Answer::new();
        //проверка на наличие данных
        if members_str.is_empty() {
            answer.add_error( "members", "not_found" );
            return Err( anwer );
        }

        let mut info = Info {
            initiator: req.user().id,
            members: HashSet::new(),
            name = try!( req.get_param( "name" ) ),
            description = try!( req.get_param( "description" ) )
        }
        //конвертация идентификаторов из строк
        let members: Members = HashSet::new(); 
        for member_str in members_str.iter() {
            members.insert( try!( convert_member( &member_str ) ) );
        }
        // проверка наличия пользователей
        for member in members.iter() {
            if try!( db.user_id_exists( member ) ) == false {
                answer.add_error( "user", "not_found" );
                return Err( answer );
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

        let exists_members = Vec::new();
        for member in info.members.iter() {
            if try!( db.user_id_exists( member ) ) { // filter итератора нельзя использовать так как это делается через db
                exists_members.push( member );
            }
        }

        // даём право голоса пользователям
        db.add_rights_of_voting( body.scheduled_id, exists_members );
        // тот кто создаёт группу по умолчанию проголосовал ЗА!
        db.set_vote( body.scheduled_id, info.initiator, true );
        // рассылаем письма что можно голосовать
        for member in exists_members.iter() {
            db.send_mail( 
                member,  
                "Создание группы",
                make_subject( info.name ),
                make_mail_body( info.name, body.scheduled_id )
            )
        }
    }
    /// действие на окончание события
    fn finish( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> EmptyResult {
        let info = try!( get_info( body.data ) );
        // проверяем что такой группы нет
        if try!( db.is_group_exists( info.name ) ) == false {
            // собиарем голоса
            let votes = try!( db.votes( body.scheduled_id ) );]
            // елси хоть кто-то решил присоединиться
            if 1 < votes.yes.len() {
                // создаём группу
                let group_id = try!( db.create_group( info.name, info.description ) );
                // и тех кто проголовал ЗА добавляем в эту группу
                try!( db.add_members( group_id, votes.yes.as_slice() ) );
            }
            else {
                // отсылаем жалостливое письмо что никто в твою группу не хочет
                undefined!();
            }            
        }
        else {
            // отсылаем письмо что группа с таким имененм уже созданна и надо поменять ей имя
            undefined!();
        }
        Ok( () )
    }
    /// описание действия пользователя на это событие 
    fn user_action_get( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult {
        
    }
    /// применение действия пользователя на это событие
    fn user_action_post( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// информация о состоянии события
    fn info_get( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> CommonResult<bool> {
        db.is_all_voted( body.scheduled_id )
    }
}

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

fn make_subject( name: &String ) -> String {
    format!( "Создание новой группы `{}`", name )
}

fn make_mail_body( name: &String, scheduled_id: Id ) -> String {
    format!( "Вас приглашают создать новую группу `{}`.
        Узнать подробности и принять решение о присоединении вы можете пройдя по этой ссылке {}.
        У вас есть неделя чтобы принять решение.",
        name,
        make_event_action_link( scheduled_id ) )
}

