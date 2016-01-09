use types::{ Id, EmptyResult, CommonResult, common_error };
use answer::{ AnswerResult };
use time::Timespec;
use iron::prelude::*;
use stuff::Stuff;
use rustc_serialize::{ Encodable, Encoder };
use rustc_serialize::json;
use std::fmt::Debug;

pub mod events_manager;
mod events_collection;
mod publication;
mod group_creation;
mod late_publication;
mod group_voting;
mod change_timetable;

pub struct ScheduledEventInfo {
    pub id: Id,
    pub scheduled_id: Id,
    pub start_time: Timespec,
    pub end_time: Timespec,
    pub name: String,
    pub data: String,
    pub state: EventState,
    /// показывает что событие привязано к какой-то группе
    pub group: Option<Id>
}

#[derive(Debug)]
pub struct FullEventInfo {
    pub id: Id,
    pub name: String,
    pub start_time: Timespec,
    pub end_time: Timespec,
    pub data: String,
    /// показывает что событие привязано к какой-то группе
    pub group: Option<Id>
}

#[derive(Copy, Clone, RustcEncodable)]
pub enum EventState {
    Disabled = 0,
    NotStartedYet = 1,
    Active = 2,
    Finished = 3
}

#[derive(Copy, Clone, RustcEncodable)]
pub enum UserAction {
    None = 0,
    Vote = 1,
    Publication = 2
}

#[derive(Clone, Debug)]
pub struct Description {
    value: String
}

/// абстракция какого-то автоматического события
pub trait Event {
    /// идентификатор события
    fn id( &self ) -> Id;
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult;
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult;
    /// информация о состоянии события
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description>;
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool>;
    /// действие которое должен осуществить пользователь
    fn user_action( &self, stuff: &mut Stuff, body: &ScheduledEventInfo, user_id: Id ) -> CommonResult<UserAction>;
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult;
}

/// абстракция события которое может быть создано добавлено в расписание группы
pub trait CreateFromTimetable {
    /// проверяет параметры на достоверность
    fn is_valid_params( &self, params: &String ) -> bool;
    /// создаёт данные для события, возвращет None если параметры не соответствуют этому событию
    fn from_timetable( &self, group_id: Id, params: &String ) -> Option<String>;
    /// выдаёт пару начала и окончания по времени в зависимости от времени события
    fn time_gate( &self, time: &Timespec ) -> (Timespec, Timespec);
}

/// абстракция событий которые могут быть созданы пользователями
pub trait UserCreatedEvent {
    /// описание создания
    fn user_creating_get( &self, req: &mut Request ) -> AnswerResult;
    /// применение создания
    fn user_creating_post( &self, req: &mut Request ) -> Result<FullEventInfo, AnswerResult>;
}

/// абстракция событий которые могут быть созданы пользователями для группы
pub trait GroupCreatedEvent {
    /// описание создания
    fn user_creating_get( &self, req: &mut Request, group_id: Id ) -> AnswerResult;
    /// применение создания
    fn user_creating_post( &self, req: &mut Request, group_id: Id ) -> Result<FullEventInfo, AnswerResult>;
}

//TODO: Когда будет возможность перейти со String на тип Json или аналог. Когда в
// стабильной версии можно будет использовать чужиe derive
impl Description {
    pub fn new<T: Encodable + Debug>( v: T ) -> Description {
        Description {
            value: json::encode( &v )
                .expect( &format!( "Invalid encoding to description struct: {:?}", v ) )
        }
    }
}

impl Encodable for Description {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.value.encode( s )
    }
}

pub fn make_event_link( id: Id ) -> String {
    format!( "/#event/{}", id )
}

pub fn get_group_id( body: &ScheduledEventInfo ) -> CommonResult<Id> {
    match body.group {
        Some( id ) => Ok( id ),
        None => common_error(
            format!( "group_id not found in ScheduledEventInfo id={} scheduled_id={}",
                      body.id,
                      body.scheduled_id ))
    }
}
