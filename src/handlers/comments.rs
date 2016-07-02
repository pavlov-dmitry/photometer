use iron::prelude::*;
use db::comments::DbComments;
use db::visited::{ DbVisited, VisitedContent };
use db::photos::DbPhotos;
use db::events::DbEvents;
use db::users::DbUsers;
use db::comment_message_link::{ DbCommentMessageLink, MessageLink };
use db::mailbox::DbMailbox;
use get_body::GetBody;
use answer::{ AnswerResult, Answer, AnswerResponse };
use types::{
    Id,
    CommentInfo,
    CommentFor,
    EmptyResult,
    ShortPhotoInfo,
    common_error,
    CommonResult
};
use authentication::{ Userable, User };
use database::{ Databaseable };
use stuff::{ Stuffable, Stuff };
use answer_types::{ PaginationInfo, FieldErrorInfo };
use super::helpers::make_pagination;
use time;
use parse_utils::GetMsecs;
use std::collections::HashSet;
use regex::Regex;
use mail_writer::MailWriter;
use mailer::Mailer;

pub fn get_event_comments( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( get_comments( req, CommentFor::Event ) );
    Ok( Response::with( answer ) )
}

pub fn get_photo_comments( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( get_comments( req, CommentFor::Photo ) );
    Ok( Response::with( answer ) )
}

pub fn post_event_comment( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( add_comment( req, CommentFor::Event ) ); Ok( Response::with( answer ) ) }

pub fn post_photo_comment( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( add_comment( req, CommentFor::Photo ) );
    Ok( Response::with( answer ) )
}

pub fn post_edit_comment( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( edit_comment( req ) );
    Ok( Response::with( answer ) )
}

const IN_PAGE_COUNT: u32 = 10;
const MAX_COMMENT_LENGTH: usize = 8192;

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
            let message_ids = try!( db.get_linked_messages( user_id, new_visited ) );
            try!( db.mark_as_readed( user_id, &message_ids ) );

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
    if MAX_COMMENT_LENGTH < info.text.chars().count() {
        return Some( FieldErrorInfo::too_long("text") );
    }
    None
}

fn add_comment( req: &mut Request, comment_for: CommentFor ) -> AnswerResult {
    let info = try!( req.get_body::<AddCommentInfo>() );
    if let Some( e ) = check_comment( &info ) {
        return Ok( Answer::bad( e ) );
    }

    let comment_id = {
        let user_id = req.user().id;
        let db = try!( req.stuff().get_current_db_conn() );
        let comment_id = try!( db.add_comment( user_id,
                                               time::get_time().msecs(),
                                               comment_for,
                                               info.id,
                                               &info.text ) );
        match comment_for {
            CommentFor::Photo => try!( db.increment_photo_comments_count( info.id ) ),
            CommentFor::Event => try!( db.increment_event_comments_count( info.id ) )
        }
        comment_id
    };

    // генерим оповещения
    let user = req.user().clone();
    let mut notices = make_notice_set( &info.text );
    notices.remove( &user.name.to_lowercase() );
    let stuff = req.stuff();
    try!( send_notices( stuff, comment_id, notices, comment_for, &user, info.id ) );
    Ok( Answer::good( "ok" ) )
}

fn send_notices(
    stuff: &mut Stuff,
    comment_id: Id,
    mut notices: HashSet<String>,
    comment_for: CommentFor,
    commenter_user: &User,
    photo_id: Id
) -> EmptyResult
{
    let mut links = Vec::new();
    let owner_id = match comment_for {
        CommentFor::Photo => {
            //оповещяем владельца фотки, о новом комментарии
            let short_info = {
                let db = try!( stuff.get_current_db_conn() );
                //NOTE: Делаю здесь unwrap, так как сюда мы не можем добраться если этой офтки нет
                match try!( db.get_short_photo_info( photo_id ) ) {
                    Some( info ) => info,
                    None => return common_error( "invalid photo id".to_owned() )
                }
            };
            notices.remove( &short_info.owner.name.to_lowercase() );
            if commenter_user.id != short_info.owner.id {
                let link = try!( notify_owner( stuff, &short_info, &commenter_user.name ) );
                links.push( link );
            }
            short_info.owner.id
        },
        CommentFor::Event => 0
    };
    for name in notices {
        info!( "notice for {}", name );
        let maybe_user = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.user_by_name( &name ) )
        };
        if let Some( user ) = maybe_user {
            let (subject, mail) = stuff.write_you_refered_in_comment_mail(
                &commenter_user.name,
                comment_for,
                owner_id,
                photo_id
            );
            let message_id = try!( stuff.send_mail( &user, &subject, &mail ) );

            links.push( MessageLink{
                user_id: user.id,
                message_id: message_id
            } );
        }
    }

    let db = try!( stuff.get_current_db_conn() );
    try!( db.add_comment_message_links( comment_id, links ) );

    Ok( () )
}

fn notify_owner( stuff: &mut Stuff, photo_info: &ShortPhotoInfo, commenter_name: &str ) -> CommonResult<MessageLink>
{
    // генерим сообщение для владельца фотографии
    let (subject, mail) = stuff.write_your_photo_commented_mail(
        &photo_info.name,
        commenter_name,
        photo_info.owner.id,
        photo_info.id
    );
    let photo_owner = {
        let db = try!( stuff.get_current_db_conn() );
        match try!( db.user_by_id( photo_info.owner.id ) ) {
            Some( user ) => user,
            None => return common_error( "invalid photo owner id".to_owned() )
        }
    };
    let message_id = try!( stuff.send_mail( &photo_owner, &subject, &mail ) );
    Ok( MessageLink{
        user_id: photo_info.owner.id,
        message_id: message_id
    } )
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

fn make_notice_set( msg: &str ) -> HashSet<String> {
    let mut result = HashSet::new();
    let regexp = Regex::new( r"@(\w+)" ).unwrap();
    for cap in regexp.captures_iter( msg ) {
        cap.at( 1 ).map( |name| {
            result.insert( name.to_owned().to_lowercase() );
        });
    }
    result
}
