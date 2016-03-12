use photo_store::{ PhotoStoreable, PhotoStoreError };
use answer::{ AnswerResult, Answer, AnswerResponse };
use authentication::{ Userable, User };
use err_msg;
use time;
use time::{ Timespec };
use types::{ PhotoInfo, ImageType, ShortInfo, common_error };
use answer_types::{ OkInfo, PhotoErrorInfo };
use exif_reader;
use exif_reader::{ ExifValues };
use database::{ Databaseable };
use stuff::Stuffable;
use db::photos::{ DbPhotos };
use iron::prelude::*;
use params_body_parser::{ ParamsBody, BinParamsError };
use std::convert::From;
use parse_utils::{ GetMsecs };

static IMAGE : &'static str = "upload_img";


pub fn upload_photo( request: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( upload_photo_answer( request ) );
    Ok( Response::with( answer ) )
}

fn upload_photo_answer( request: &mut Request ) -> AnswerResult {
    let params = match request.parse_bin_params() {
        Ok( p ) => p,

        Err( BinParamsError::TooBig ) => {
            let answer = Answer::bad( PhotoErrorInfo::too_big() );
            return Ok( answer );
        }

        Err( BinParamsError::NotMultipartFormData ) => {
            return common_error( String::from( "not a multiform data" ) );
        }

        Err( BinParamsError::IoError ) => {
            return common_error( String::from( "error while reading request body" ) );
        }
    };

    // проверка правильности переданных параметров
    let image = match params.get( IMAGE ) {
        Some( img ) => img,
        None => return Err( err_msg::param_not_found( IMAGE ) )
    };
    let image_filename = match image.filename {
        Some( ref filename ) => filename,
        None => return Err( err_msg::invalid_type_param( IMAGE ) )
    };
    let answer = match check_image_type( &image_filename ) {
        Some( tp ) => {
            let photo_info = {
                let photo_store = request.photo_store();
                let upload_time = time::get_time();
                photo_store
                    .add_new_photo( request.user(), &upload_time, tp.clone(), &image.data )
                    .map( |(w, h)| make_photo_info( request.user(),
                                                    upload_time,
                                                    tp.clone(),
                                                    w,
                                                    h,
                                                    &image.data ) )
            };
            match photo_info {
                Ok( mut photo_info ) => {
                    let user_id = request.user().id;
                    let db = try!( request.stuff().get_current_db_conn() );
                    let next = try!( db.get_last_upload_gallery_photo( user_id ) );
                    photo_info.next = next.clone();
                    match db.add_photo( user_id, &photo_info ) {
                        Ok( added_id ) => {
                            if let Some( id ) = next {
                                try!( db.set_prev_in_gallery( id, added_id ) );
                            }
                            Answer::good( OkInfo::new( "photo_loaded" ) )
                        },
                        Err( e ) => return common_error( format!( "{}", e ) )
                    }
                }
                Err( e ) => match e {
                    PhotoStoreError::Image( e ) => return Err( From::from( e ) ),
                    PhotoStoreError::Fs( e ) => return Err( err_msg::fs_error( e ) ),
                    PhotoStoreError::Format => Answer::bad( PhotoErrorInfo::bad_image() )
                }
            }
        }

        None => Answer::bad( PhotoErrorInfo::unknown_format() )
    };
    Ok( answer )
}

fn make_photo_info( owner: &User,
                    upload_time: Timespec,
                    img_type: ImageType,
                    w: u32,
                    h: u32,
                    img_data: &[u8] ) -> PhotoInfo
{
    let exif = exif_reader::from_memory( img_data );
    let exif_ref = exif.as_ref();
    PhotoInfo {
        id: 0,
        upload_time: upload_time.msecs(),
        image_type: img_type,
        width: w,
        height: h,
        name: "".to_string(),
        iso: exif_ref.and_then( |e| e.iso() ),
        shutter_speed: exif_ref.and_then( |e| e.shutter_speed() ),
        aperture: exif_ref.and_then( |e| e.aperture() ),
        focal_length: exif_ref.and_then( |e| e.focal_length() ),
        focal_length_35mm: exif_ref.and_then( |e| e.focal_length_35mm() ),
        camera_model: exif_ref.and_then( |e| e.camera_model().map( |cm| cm.to_string() ) ),
        owner: ShortInfo {
            id: owner.id,
            name: owner.name.clone()
        },
        comments_count: 0,
        unreaded_comments: 0,
        next: None,
        prev: None
    }
}

fn check_image_type( filename: &str ) -> Option<ImageType> {
    if filename.ends_with( ".jpg" ) || filename.ends_with( ".JPG" ) ||
       filename.ends_with( ".jpeg" ) || filename.ends_with( ".JPEG" ) {
        Some( ImageType::Jpeg )
    }
    else if filename.ends_with( ".png" ) || filename.ends_with( ".PNG" )  {
        Some( ImageType::Png )
    }
    else {
        None
    }
}
