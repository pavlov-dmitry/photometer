use answer::{ AnswerResult, Answer, AnswerResponse };
use stuff::Stuffable;
use database::Databaseable;
use iron::prelude::*;
use authentication::User;
use get_body::GetBody;
use db::users::DbUsers;

const IN_PAGE_COUNT: u32 = 100;

pub fn users_path() -> &'static str {
    "/search/users"
}

pub fn users( request: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( get_users( request ) );
    Ok( Response::with( answer ) )
}

#[derive(Clone, RustcDecodable)]
struct UsersQuery {
    like: String
}

#[derive(RustcEncodable)]
struct UsersResult {
    users: Vec<User>
}

fn get_users( req: &mut Request ) -> AnswerResult {
    let like = try!( req.get_body::<UsersQuery>() ).like + "%";
    let db = try!( req.stuff().get_current_db_conn() );
    let users = try!( db.get_users_like( &like, 0, IN_PAGE_COUNT ) );
    let users_result = UsersResult {
        users: users
    };
    let answer = Answer::good( users_result );
    Ok( answer )
}
