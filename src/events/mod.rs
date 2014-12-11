use types::{ Id, EmptyResult };
use database::{ DbConnection };
use serialize::json::{ Json };
use answer::{ AnswerResult };
use nickel::{ Request };

pub struct EventBody<'a> {
    pub sheduled_id: Id,
    pub data: &'a Json
}

/// абстракция какого-то автоматического события
pub trait Event {
    /// идентификатор события
    fn id( &self ) -> Id;
    /// действие на начало события
    fn start( &self, db: &mut DbConnection, body: &EventBody ) -> EmptyResult;
    /// действие на окончание события
    fn finish( &self, db: &mut DbConnection, body: &EventBody ) -> EmptyResult;
    /// описание действиz пользователя на это событие 
    fn user_action_get( &self, db: &mut DbConnection, request: &Request, body: &EventBody ) -> AnswerResult;
    /// применение действия пользователя на это событие
    fn user_action_post( &self, db: &mut DbConnection, request: &Request, body: &EventBody ) -> AnswerResult;
    /// информация о состоянии события
    fn info( &self, db: &mut DbConnection, request: &Request, body: &EventBody ) -> AnswerResult;
}