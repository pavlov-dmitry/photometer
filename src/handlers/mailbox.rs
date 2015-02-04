use answer::{ AnswerResult, Answer };
use database::{ Databaseable };
use db::mailbox::{ DbMailbox };
use authentication::{ Userable };
use get_param::{ GetParamable };
use iron::prelude::*;
use iron::status;

static PAGE: &'static str = "page";
const IN_PAGE_COUNT: u32 = 10;

pub fn count(request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, count_answer( request, false )) ) )
}

pub fn count_unreaded( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, count_answer( request, true )) ) )
}

fn count_answer( req: &mut Request, only_unreaded: bool ) -> AnswerResult {
    let user_id = req.user().id;
    let db = try!( req.get_current_db_conn() );
    let count = try!( db.messages_count( user_id, only_unreaded ) );
    let mut answer = Answer::new();
    answer.add_record( "count", &count );
    Ok( answer )
}

pub fn get(request: &mut Request ) -> IronResult<Response> {
   Ok( Response::with( (status::Ok, get_answer( request, false )) ) )
}

pub fn get_unreaded(request: &mut Request) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, get_answer( request, true )) ) )
}

fn get_answer( req: &mut Request, only_unreaded: bool ) -> AnswerResult {
    let page = req.get_param_uint( PAGE ).unwrap_or( 0 ) as u32;
    let user_id = req.user().id;
    let db = try!( req.get_current_db_conn() );
    let mut answer = Answer::new();
    try!( 
        db.messages_from_last( 
            user_id,
            only_unreaded,
            page * IN_PAGE_COUNT,
            IN_PAGE_COUNT,
            &mut |mail_info| answer.add_to_records( mail_info )
        )
    );
    Ok( answer )
}

pub fn mark_as_readed( request: &mut Request) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, mark_as_readed_answer( request )) ) )
}

pub fn mark_as_readed_answer( req: &mut Request ) -> AnswerResult {
    let id = try!( req.get_param_id( "id" ) );
    let user_id = req.user().id;
    let db = try!( req.get_current_db_conn() );
    let success = try!( db.mark_as_readed( user_id, id ) );
    let mut answer = Answer::new();
    if success {
        answer.add_record( "marked", &success );
    }
    else {
        answer.add_error( "permisson", "denied" );
    }
    Ok( answer )
}
