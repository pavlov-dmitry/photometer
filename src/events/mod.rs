use types::{ Id, EmptyResult, CommonResult };
use answer::{ AnswerResult };
use time::Timespec;
use iron::prelude::*;
use stuff::Stuff;

pub mod events_manager;
mod time_store;
mod events_collection;
mod publication;
mod group_creation;
mod late_publication;

pub struct ScheduledEventInfo {
    pub id: Id,
    pub scheduled_id: Id,
    pub name: String,
    pub data: String,
    pub state: EventState
}

#[derive(Debug)]
pub struct FullEventInfo {
    pub id: Id,
    pub name: String,
    pub start_time: Timespec,
    pub end_time: Timespec,
    pub data: String
}

pub enum EventState {
    NotStartedYet = 0,
    Active = 1,
    Finished = 2
}

/// абстракция какого-то автоматического события
pub trait Event {
    /// идентификатор события
    fn id( &self ) -> Id;
    /// действие на начало события
    fn start( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult;
    /// действие на окончание события
    fn finish( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> EmptyResult;
    /// описание действиz пользователя на это событие 
    fn user_action_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// применение действия пользователя на это событие
    fn user_action_post( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// информация о состоянии события
    fn info_get( &self, req: &mut Request, body: &ScheduledEventInfo ) -> AnswerResult;
    /// проверка на возможное досрочное завершение
    fn is_complete( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<bool>;
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
    fn user_creating_get( &self, req: &mut Request ) -> AnswerResult;
    /// применение создания
    fn user_creating_post( &self, req: &mut Request ) -> Result<FullEventInfo, AnswerResult>;
}

pub fn make_event_action_link( id: Id ) -> String {
    format!( "/event/action/{}", id )
}
