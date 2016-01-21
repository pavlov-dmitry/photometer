use authentication::{ Userable };

use answer::{ AnswerResult, Answer, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use db::photos::{ DbPhotos };
use iron::prelude::*;
use types::Id;
use get_body::GetBody;
use answer_types::{ OkInfo, AccessErrorInfo };

pub fn rename_photo( req: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( rename_answer( req ) );
    Ok( Response::with( answer ) )
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
    let answer = match maybe_photo_info {
        Some( info ) => {
            if info.owner.id == request.user().id {
                let db = try!( request.stuff().get_current_db_conn() );
                let _ = try!( db.rename_photo( rename_info.id, &rename_info.name ) );
                Answer::good( OkInfo::new( "rename" ) )
            }
            else {
                Answer::bad( AccessErrorInfo::new() )
            }
        },
        None => Answer::not_found()
    };
    Ok( answer )
}
