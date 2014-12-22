use nickel::{ Request, Response };
use authentication::{ Userable };

use answer::{ AnswerSendable, AnswerResult, Answer };
use get_param::{ GetParamable };
use database::{ Databaseable };
use db::photos::{ DbPhotos };

pub fn rename_photo( req: &Request, res: &mut Response ) {
	res.send_answer( &rename_answer( req ) );
}

fn rename_answer( request: &Request ) -> AnswerResult {
	let id = try!( request.get_param_id( "id" ) );
	let name = try!( request.get_param( "name" ) );
	let mut db = try!( request.get_db_conn() );
    let maybe_photo_info = try!( db.get_photo_info( id ) );
    let mut answer = Answer::new();
    match maybe_photo_info {
        Some( (user, _ ) ) => {
        	if user == request.user().name {
        		let _ = try!( db.rename_photo( id, name ) );
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