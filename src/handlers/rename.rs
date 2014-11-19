use nickel::{ Request, Response };
use authentication::{ Userable };

use answer;
use answer::{ AnswerSendable, AnswerResult };
use super::get_param::{ GetParamable };
use database::{ Databaseable };

pub fn rename_photo( req: &Request, res: &mut Response ) {
	res.send_answer( &rename_answer( req ) );
}

fn rename_answer( request: &Request ) -> AnswerResult {
	let id = try!( request.get_param_i64( "id" ) );
	let name = try!( request.get_param( "name" ) );
	let mut answer = answer::new();
    let maybe_photo_info = try!( request.db().get_photo_info( id ) );
    match maybe_photo_info {
        Some( (user, _ ) ) => {
        	if user == request.user().name {
        		let _ = try!( request.db().rename_photo( id, name ) );
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