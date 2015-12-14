use iron::prelude::*;
use answer::{ AnswerResult, Answer, AnswerResponse };
use types::{ Id };
use answer_types::{ FieldErrorInfo };
use get_body::GetBody;
use db::groups::DbGroups;
// use types::PhotoInfo;
use authentication::Userable;
use database::Databaseable;
use stuff::Stuffable;

#[derive(Clone, Copy, RustcDecodable)]
struct GroupQuery {
    group_id: Id,
}

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
                editable: is_member,
                members: members
            };
            Answer::good( group )
        },
        None => Answer::bad( FieldErrorInfo::not_found( "group" ) )
    };
    Ok( answer )
}
