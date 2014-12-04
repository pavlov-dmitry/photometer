use nickel::{ Request, Response };
use photo_store::{ PhotoStoreable, PhotoStoreError };
use answer;
use answer::{ AnswerSendable, AnswerResult };
use super::get_param::{ GetParamable };
use database::{ Databaseable };
use super::err_msg;
use authentication::{ Userable };
use db::photos::{ DbPhotos };

pub fn crop_photo( request: &Request, response: &mut Response ) {
    response.send_answer( &crop_photo_answer( request ) );
}

fn crop_photo_answer( request: &Request ) -> AnswerResult {
    let id = try!( request.get_param_i64( "id" ) );
    let x1 = try!( request.get_param_uint( "x1" ) ) as u32;
    let y1 = try!( request.get_param_uint( "y1" ) ) as u32;
    let x2 = try!( request.get_param_uint( "x2" ) ) as u32;
    let y2 = try!( request.get_param_uint( "y2" ) ) as u32;
    let mut db = try!( request.get_db_conn() );
    let maybe_photo_info = try!( db.get_photo_info( id ) );
    let mut answer = answer::new();
    match maybe_photo_info {
        Some( (user, info ) ) => {
            if user == request.user().name {
                match request.photo_store().make_crop( &user, info.upload_time, info.image_type, (x1, y1), (x2, y2) ) {
                    Ok( _ ) => answer.add_record( "cropped", &String::from_str( "ok" ) ),
                    Err( e ) => match e {
                        PhotoStoreError::Fs( e ) => return Err( err_msg::fs_error( e ) ),
                        PhotoStoreError::Format => answer.add_error( "photo", "bad_image" ),
                        _ => return Err( String::from_str( "crop unknown error" ) )
                    }
                }
            }
            else {
                answer.add_error( "access", "denied" );
            }
        },
        None => answer.add_error( "photo", "not_found" ),
    }
    Ok( answer )
}