use authentication::{ Userable };

use answer::{ AnswerResult, Answer };
use get_param::{ GetParamable };
use database::{ Databaseable };
use stuff::Stuffable;
use db::photos::{ DbPhotos };
use iron::prelude::*;
use iron::status;

pub fn rename_photo( req: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, rename_answer( req )) ) )
}

fn rename_answer( request: &mut Request ) -> AnswerResult {
    let id = try!( request.get_param_id( "id" ) );
    let name = try!( request.get_param( "name" ) ).to_string();
    let maybe_photo_info = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.get_photo_info( id ) )
    };
    let mut answer = Answer::new();
    match maybe_photo_info {
        Some( (user, _ ) ) => {
            if user == request.user().name {
                let db = try!( request.stuff().get_current_db_conn() );
                let _ = try!( db.rename_photo( id, &name ) );
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
