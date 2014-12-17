use types::{ Id, EmptyResult, CommonResult };
use database::{ DbConnection };
use serialize::json::{ Json };
use answer::{ AnswerResult };
use nickel::{ Request };
use time::Timespec;

pub mod events_manager;
mod time_store;
mod events_collection;
mod publication;

pub struct ScheduledEventInfo {
    pub id: Id,
    pub scheduled_id: Id,
    pub name: String,
    pub data: String,
}

pub struct FullEventInfo {
    pub id: Id,
    pub name: String,
    pub start_time: Timespec,
    pub end_time: Timespec,
    pub data: String
}

pub enum EventState {
    NotStartedYet = 1,
    Active = 2,
    Ended = 3
}

/// абстракция какого-то автоматического события
pub trait Event {
    /// идентификатор события
    fn id( &self ) -> Id;
    /// действие на начало события
    fn start( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> EmptyResult;
    /// действие на окончание события
    fn finish( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> EmptyResult;
    /// описание действиz пользователя на это событие 
    fn user_action_get( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// применение действия пользователя на это событие
    fn user_action_post( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// информация о состоянии события
    fn info_get( &self, db: &mut DbConnection, request: &Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, db: &mut DbConnection, body: &ScheduledEventInfo ) -> CommonResult<bool>;
}

/// абстракция события которое может быть создано из расписания
pub trait CreateFromTimetable {
    /// проверяет параметры на достоверность
    fn is_valid_params( &self, params: &String ) -> bool;
    /// создаёт данные для события, возвращет None если параметры не соответствуют этому событию
    fn from_timetable( &self, group_id: Id, params: &String ) -> Option<String>;
}

/// абстракция событий которые могут быть созданы пользователями
pub trait UserEvent {
    /// описание создания
    fn user_creating_get( &self, db: &mut DbConnection, request: &Request ) -> AnswerResult;
    /// применение создания
    fn user_creating_post( &self, db: &mut DbConnection, request: &Request ) -> Result<Json, AnswerResult>;
}