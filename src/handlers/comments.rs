use iron::prelude::*;
use db::comments::{ DbComments, CommentFor };
use db::visited::{ DbVisited, VisitedContent };
use get_body::GetBody;
use answer::{ AnswerResult, Answer, AnswerResponse };
use types::{ Id, CommentInfo };
use authentication::{ Userable };
use database::{ Databaseable };
use stuff::Stuffable;
use answer_types::{ PaginationInfo, FieldErrorInfo };
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
    let answer = AnswerResponse( add_comment( req, CommentFor::Photo ) );
    Ok( Response::with( answer ) )
}

pub fn post_edit_comment( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( edit_comment( req ) );
    Ok( Response::with( answer ) )
}

const IN_PAGE_COUNT: u32 = 10;
const MAX_COMMENT_LENGTH: usize = 1024;

#[derive(Clone, RustcDecodable)]
struct CommentsQuery {
    id: Id,
    page: u32
}

#[derive(RustcEncodable)]
struct CommentsInfo {
    all_count: u32,
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
            // помечаем новые комментрии как посещенные
            let new_visited = comments.iter()
                .filter( |c| c.is_new )
                .map( |c| c.id )
                .collect::<Vec<Id>>();
            try!( db.set_visited( user_id, VisitedContent::Comment, &new_visited ) );

            Answer::good( CommentsInfo{
                all_count: count,
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

fn check_comment( info: &AddCommentInfo ) -> Option<FieldErrorInfo> {
    if info.text.is_empty() {
        return Some( FieldErrorInfo::empty("text") );
    }
    if MAX_COMMENT_LENGTH < info.text.len() {
        return Some( FieldErrorInfo::too_long("text") );
    }
    None
}

fn add_comment( req: &mut Request, comment_for: CommentFor ) -> AnswerResult {
    let info = try!( req.get_body::<AddCommentInfo>() );
    if let Some( e ) = check_comment( &info ) {
        return Ok( Answer::bad( e ) );
    }

    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    try!( db.add_comment( user_id,
                          time::get_time().msecs(),
                          comment_for,
                          info.id,
                          &info.text ) );
    Ok( Answer::good( "ok" ) )
}

fn edit_comment( req: &mut Request ) -> AnswerResult {
    let info = try!( req.get_body::<AddCommentInfo>() );
    if let Some( e ) = check_comment( &info ) {
        return Ok( Answer::bad( e ) );
    }

    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let comment_info = try!( db.get_comment_info( user_id, info.id ) );
    let answer = match comment_info {
        Some( comment ) => {
            if comment.creator.id == user_id {
                try!( db.edit_comment(
                    info.id,
                    time::get_time().msecs(),
                    &info.text
                ));
                Answer::good( "ok" )
            }
            else {
                Answer::access_denied()
            }
        },
        None => Answer::not_found()
    };
    Ok( answer )
}
