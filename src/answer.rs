use rustc_serialize::{ Encodable };
use rustc_serialize::json;
use iron::modifier::Modifier;
use iron::prelude::*;
use iron::mime;
use iron::status;

use types::{ CommonResult, CommonError };

pub trait AnswerSendable {
    fn send_answer( &mut self, answer: &AnswerResult );
}

type ToStringPtr = Box<ToString>;

//TODO: после обновления до беты убрать static lifetime
struct AnswerBody<Body: Encodable + 'static> {
    body: Body
}

fn new_body<Body: Encodable + 'static>( body: Body ) -> AnswerBody<Body> {
    AnswerBody {
        body: body
    }
}

impl<Body: Encodable + 'static> ToString for AnswerBody<Body> {
    fn to_string( &self ) -> String {
        json::encode( &self.body ).unwrap()
    }
}

pub enum Answer {
    Good( ToStringPtr ),
    Bad( ToStringPtr )
}

pub type AnswerResult = CommonResult<Answer>;
pub struct AnswerResponse( pub AnswerResult );

impl Answer {
    pub fn good<Body: Encodable + 'static>(body: Body) -> Answer {
        Answer::Good( Box::new( new_body( body ) ) as ToStringPtr )
    }

    pub fn bad<Body: Encodable + 'static>(body: Body) -> Answer {
        Answer::Bad( Box::new( new_body( body ) ) as ToStringPtr )
    }
}

impl Modifier<Response> for AnswerResponse {
    #[inline]
    fn modify(self, res: &mut Response) {
        match self {
            AnswerResponse( Ok( ref answer ) ) => {
                //TODO: переделать на константу
                let mime: mime::Mime = "application/json;charset=utf8".parse().unwrap();
                mime.modify( res );

                match answer {
                    &Answer::Good( ref body ) => {
                        let answer_status = status::Ok;
                        answer_status.modify( res );
                        body.to_string().modify( res );
                    }

                    &Answer::Bad( ref body ) => {
                        let answer_status = status::BadRequest;
                        answer_status.modify( res );
                        body.to_string().modify( res );
                    }
                }
            }

            AnswerResponse( Err( CommonError( err ) ) ) => {
                error!( "{}", err );
                let answer_status = status::InternalServerError;
                answer_status.modify( res );
                //TODO: в релизе отключить посылку описания ошибок интерфейсу
                // err.modify( res );
            }
        }
    }
}

impl From<CommonError> for AnswerResult {
    fn from( err: CommonError ) -> AnswerResult {
        Err( err )
    }
}
