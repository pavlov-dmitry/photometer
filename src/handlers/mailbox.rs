use answer::{ AnswerResult, Answer };
use database::{ Databaseable };
use stuff::Stuffable;
use db::mailbox::{ DbMailbox };
use authentication::{ Userable };
use iron::prelude::*;
use iron::status;
use get_body::GetBody;
use types::Id;
use answer_types::{ OkInfo, CountInfo, AccessErrorInfo };

const IN_PAGE_COUNT: u32 = 10;

pub fn count(request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, count_answer( request, false )) ) )
}

pub fn count_unreaded( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, count_answer( request, true )) ) )
}

fn count_answer( req: &mut Request, only_unreaded: bool ) -> AnswerResult {
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let count = try!( db.messages_count( user_id, only_unreaded ) );
    let answer = Answer::good( CountInfo::new( count ) );
    Ok( answer )
}

pub fn get(request: &mut Request ) -> IronResult<Response> {
   Ok( Response::with( (status::Ok, get_answer( request, false )) ) )
}

pub fn get_unreaded(request: &mut Request) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, get_answer( request, true )) ) )
}

#[derive(Clone, Copy, RustcDecodable)]
struct PageInfo {
    page: u32
}

fn get_answer( req: &mut Request, only_unreaded: bool ) -> AnswerResult {
    let page = try!( req.get_body::<PageInfo>() ).page;
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let infos = {
        let mut infos = Vec::new();
        try!( db.messages_from_last( user_id,
                                     only_unreaded,
                                     page * IN_PAGE_COUNT,
                                     IN_PAGE_COUNT,
                                     &mut |mail_info| infos.push( mail_info ) ) );
        infos
    };
    let answer = Answer::good( infos );
    Ok( answer )
}

pub fn mark_as_readed( request: &mut Request) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, mark_as_readed_answer( request )) ) )
}

#[derive(Clone, Copy, RustcDecodable)]
struct IdInfo {
    id: Id
}

pub fn mark_as_readed_answer( req: &mut Request ) -> AnswerResult {
    let id = try!( req.get_body::<IdInfo>() ).id;
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let success = try!( db.mark_as_readed( user_id, id ) );
    let answer = if success {
        Answer::good( OkInfo::new( "marked" ) )
    }
    else {
        Answer::bad( AccessErrorInfo::new() )
    };
    Ok( answer )
}
