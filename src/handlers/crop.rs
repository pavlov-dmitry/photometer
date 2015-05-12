use photo_store::{ PhotoStoreable, PhotoStoreError };
use answer::{ AnswerResult, Answer, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use err_msg;
use authentication::{ Userable };
use db::photos::{ DbPhotos };
use iron::prelude::*;
use get_body::GetBody;
use types::{ Id };
use answer_types::{ OkInfo, PhotoErrorInfo, AccessErrorInfo };

pub fn crop_photo( request: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( crop_photo_answer( request ) );
    Ok( Response::with( answer ) )
}

#[derive(Clone, RustcDecodable)]
struct CropInfo {
    id: Id,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32
}

fn crop_photo_answer( request: &mut Request ) -> AnswerResult {
    let crop_info = try!( request.get_body::<CropInfo>() );

    let maybe_photo_info = {
        let db = try!( request.stuff().get_current_db_conn() );
        try!( db.get_photo_info( crop_info.id ) )
    };

    let answer = match maybe_photo_info {
        Some( (user, info ) ) => {
            if user == request.user().name {
                let photo_store = request.photo_store();
                let x1 = crop_info.x1 as u32;
                let x2 = crop_info.x2 as u32;
                let y1 = crop_info.y1 as u32;
                let y2 = crop_info.y2 as u32;
                let crop_result = photo_store.make_crop(
                    &user,
                    info.upload_time,
                    info.image_type,
                    (x1, y1),
                    (x2, y2)
                );
                match crop_result {
                    Ok( _ ) => Answer::good( OkInfo::new( "cropped" ) ),
                    Err( e ) => match e {
                        PhotoStoreError::Image( e ) => return Err( From::from( e ) ),
                        PhotoStoreError::Fs( e ) => return Err( err_msg::fs_error( e ) ),
                        PhotoStoreError::Format => Answer::bad( PhotoErrorInfo::bad_image() )
                    }
                }
            }
            else {
                Answer::bad( AccessErrorInfo::new() )
            }
        },

        None => Answer::bad( PhotoErrorInfo::not_found() )
    };
    Ok( answer )
}
