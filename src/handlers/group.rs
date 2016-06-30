use iron::prelude::*;
use answer::{ AnswerResult, Answer, AnswerResponse };
use types::{ Id, EmptyResult };
use get_body::GetBody;
use db::groups::DbGroups;
use db::group_feed::DbGroupFeed;
use db::visited::{ DbVisited, VisitedContent };
use authentication::Userable;
use database::Databaseable;
use stuff::{ Stuff, Stuffable };
use parse_utils::GetMsecs;
use events::feed_types::FeedEventInfo;
use events::EventId;
use events::events_manager::EventsManagerStuff;

#[derive(Clone, Copy, RustcDecodable)]
struct GroupQuery {
    group_id: Id,
}

#[derive(Clone, Copy, RustcDecodable)]
struct GroupFeedQuery {
    group_id: Id,
    page: u32
}

#[derive(RustcEncodable)]
struct GroupFeedResult {
    group_id: Id,
    group_name: String,
    feed: Vec<FeedEventInfo>
}

const IN_PAGE_COUNT: u32 = 10;

#[derive(Clone, Copy, RustcDecodable)]
struct PublicationQuery {
    group_id: Id,
    publication_idx: u32
}

#[derive(RustcEncodable)]
struct UserInfo {
    id: Id,
    name: String
}

#[derive(RustcEncodable)]
struct GroupInfo {
    id: Id,
    name: String,
    description: String,
    creation_time: u64,
    editable: bool,
    members: Vec<UserInfo>
}

// #[derive(RustcEncodable)]
// struct PublicationInfo {
//     id: Id,
//     group_id: Id,
//     name: String,
//     description: String,
//     is_new: bool,
//     photos: Vec<PhotoInfo>,
//     current_idx: u32,
//     count: u32
// }

pub fn get_group_info( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( group_info( req ) );
    Ok( Response::with( answer ) )
}

pub fn get_group_feed( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( group_feed( req ) );
    Ok( Response::with( answer ) )
}

pub fn get_group_feed_element( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( group_feed_element( req ) );
    Ok( Response::with( answer ) )
}

fn group_info( req: &mut Request ) -> AnswerResult {
    let group_id = try!( req.get_body::<GroupQuery>() ).group_id;
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let is_member = try!( db.is_member( user_id, group_id ) );
    let members = try!( db.get_members( group_id ) );
    let members = members.into_iter()
        .map( |u| UserInfo {
            id: u.id,
            name: u.name
        })
        .collect();
    let info = try!( db.group_info( group_id ) );
    let answer = match info {
        Some( info ) => {
            let group = GroupInfo {
                id: info.id,
                name: info.name,
                description: info.description,
                creation_time: info.creation_time.msecs(),
                editable: is_member,
                members: members
            };
            Answer::good( group )
        },
        None => Answer::not_found()
    };
    Ok( answer )
}

fn group_feed( req: &mut Request ) -> AnswerResult {
    let group_query = try!( req.get_body::<GroupFeedQuery>() );
    let user_id = req.user().id;
    let stuff = req.stuff();
    let group_info = {
        let db = try!( stuff.get_current_db_conn() );
        try!( db.group_info( group_query.group_id ) )
    };

    let answer = match group_info {
        Some( group_info ) => {
            let mut feed = {
                let db = try!( stuff.get_current_db_conn() );
                try!( db.get_group_feed( user_id,
                                         group_query.group_id,
                                         IN_PAGE_COUNT,
                                         IN_PAGE_COUNT * group_query.page ) )
            };

            try!( add_actions_to_feed( user_id, stuff, &mut feed ) );

            let new_feeds = feed.iter()
                .filter_map( |f| if f.is_new { Some( f.id ) } else { None } )
                .collect::<Vec<_>>();
            let db = try!( stuff.get_current_db_conn() );
            try!( db.set_visited( user_id, VisitedContent::Feed, &new_feeds ) );
            Answer::good( GroupFeedResult{
                group_id: group_info.id,
                group_name: group_info.name,
                feed: feed
            })
        },
        None => Answer::not_found()
    };

    Ok( answer )
}

fn add_actions_to_feed( user_id: Id, stuff: &mut Stuff, feed: &mut Vec<FeedEventInfo> ) -> EmptyResult
{
    for elem in feed.iter_mut() {
        if elem.event_id == EventId::Publication {
            elem.action = try!( stuff.event_user_action(
                elem.scheduled_id,
                user_id
            ) );
        }
    }
    Ok( () )
}

#[derive(Clone, RustcDecodable)]
struct GroupFeedElementQuery {
    id: Id
}

fn group_feed_element( req: &mut Request ) -> AnswerResult {
    let element_id = try!( req.get_body::<GroupFeedElementQuery>() ).id;
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let answer = match try!( db.get_feed_info( user_id, element_id ) ) {
        Some( feed_info ) => {
            if feed_info.is_new {
                try!( db.set_visited( user_id, VisitedContent::Feed, &[ feed_info.id ] ) );
            }
            Answer::good( feed_info )
        },
        None => Answer::not_found()
    };
    Ok( answer )
}
