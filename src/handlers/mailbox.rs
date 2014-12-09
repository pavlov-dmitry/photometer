use nickel::{ Request, Response };
use answer::{ AnswerResult, AnswerSendable, Answer };
use database::{ Databaseable };
use db::mailbox::{ DbMailbox };
use authentication::{ Userable };
use super::get_param::{ GetParamable };

static PAGE: &'static str = "page";
const IN_PAGE_COUNT: u32 = 10;

pub fn count(request: &Request, response: &mut Response ) {
    response.send_answer( &count_answer( request, false ) );
}

pub fn count_unreaded( request: &Request, response: &mut Response ) {
    response.send_answer( &count_answer( request, true ) );
}

fn count_answer( req: &Request, only_unreaded: bool ) -> AnswerResult {
    let mut db = try!( req.get_db_conn() );
    let count = try!( db.messages_count( req.user().id, only_unreaded ) );
    let mut answer = Answer::new();
    answer.add_record( "count", &count );
    Ok( answer )
}

pub fn get(request: &Request, response: &mut Response ) {
    response.send_answer( &get_answer( request, false ) );
}

pub fn get_unreaded(request: &Request, response: &mut Response ) {
    response.send_answer( &get_answer( request, true ) );
}

fn get_answer( req: &Request, only_unreaded: bool ) -> AnswerResult {
    let page = req.get_param_uint( PAGE ).unwrap_or( 0 ) as u32;
    let mut db = try!( req.get_db_conn() );
    let mut answer = Answer::new();
    try!( 
        db.messages_from_last( 
            req.user().id,
            only_unreaded,
            page * IN_PAGE_COUNT,
            IN_PAGE_COUNT,
            |mail_info| answer.add_to_records( mail_info )
        )
    );
    Ok( answer )
}

pub fn mark_as_readed( request: &Request, response: &mut Response ) {
    response.send_answer( &mark_as_readed_answer( request ) );
}

pub fn mark_as_readed_answer( req: &Request ) -> AnswerResult {
    let id = try!( req.get_param_i64( "id" ) );
    let mut db = try!( req.get_db_conn() );
    let success = try!( db.mark_as_readed( req.user().id, id ) );
    let mut answer = Answer::new();
    if success {
        answer.add_record( "marked", &success );
    }
    else {
        answer.add_error( "permisson", "denied" );
    }
    Ok( answer )
}