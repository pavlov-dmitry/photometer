use iron::prelude::*;
use db::comments::{ DbComments, CommentFor };
use get_body::GetBody;
use answer::{ AnswerResult, Answer, AnswerResponse };
use types::{ Id, CommentInfo };
use authentication::{ Userable };
use database::{ Databaseable };
use stuff::Stuffable;
use answer_types::{ PaginationInfo };
use super::helpers::make_pagination;
use time;
use parse_utils::GetMsecs;

pub fn get_event_comments( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( get_comments( req, CommentFor::Event ) );
    Ok( Response::with( answer ) )
}

pub fn get_photo_comments( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( get_comments( req, CommentFor::Photo ) );
    Ok( Response::with( answer ) )
}

pub fn post_event_comment( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( add_comment( req, CommentFor::Event ) );
    Ok( Response::with( answer ) )
}

pub fn post_photo_comment( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( add_comment( req, CommentFor::Event ) );
    Ok( Response::with( answer ) )
}

const IN_PAGE_COUNT: u32 = 10;

#[derive(Clone, RustcDecodable)]
struct CommentsQuery {
    id: Id,
    page: u32
}

#[derive(RustcEncodable)]
struct CommentsInfo {
    pagination: PaginationInfo,
    comments: Vec<CommentInfo>
}

fn get_comments( req: &mut Request, comment_for: CommentFor ) -> AnswerResult {
    let comments_query = try!( req.get_body::<CommentsQuery>() );
    let user_id = req.user().id;

    let db = try!( req.stuff().get_current_db_conn() );
    let comments_count = try!( db.get_comments_count( comment_for.clone(), comments_query.id ) );
    let answer = match comments_count {
        Some( count ) => {
            let comments = try!( db.get_comments( user_id,
                                                  comment_for,
                                                  comments_query.id,
                                                  IN_PAGE_COUNT,
                                                  comments_query.page * IN_PAGE_COUNT ) );
            Answer::good( CommentsInfo{
                pagination: make_pagination( comments_query.page, count, IN_PAGE_COUNT ),
                comments: comments
            })
        },
        None => Answer::not_found()
    };
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct AddCommentInfo {
    id: Id,
    text: String
}

fn add_comment( req: &mut Request, comment_for: CommentFor ) -> AnswerResult {
    let info = try!( req.get_body::<AddCommentInfo>() );
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    try!( db.add_comment( user_id,
                          time::get_time().msecs(),
                          comment_for,
                          info.id,
                          &info.text ) );
    Ok( Answer::good( "ok" ) )
}
