use authentication::{ Userable };

use answer::{ AnswerResult, Answer };
use database::{ Databaseable };
use stuff::Stuffable;
use db::photos::{ DbPhotos };
use iron::prelude::*;
use iron::status;
use types::Id;
use get_body::GetBody;

pub fn rename_photo( req: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, rename_answer( req )) ) )
}

#[derive(Clone, RustcDecodable)]
struct RenameInfo {
    id: Id,
    name: String
}

fn rename_answer( request: &mut Request ) -> AnswerResult {
    let rename_info = try!( request.get_body::<RenameInfo>() );
    let maybe_photo_info = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.get_photo_info( rename_info.id ) )
    };
    let mut answer = Answer::new();
    match maybe_photo_info {
        Some( (user, _ ) ) => {
            if user == request.user().name {
                let db = try!( request.stuff().get_current_db_conn() );
                let _ = try!( db.rename_photo( rename_info.id, &rename_info.name ) );
                answer.add_record( "rename", &String::from_str( "ok" ) );
            }
            else {
                answer.add_error( "access", "denied" );
            }
        },
        None => answer.add_error( "photo", "not_found" ),
    }
    Ok( answer )
}
