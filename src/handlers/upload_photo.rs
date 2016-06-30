use photo_store::{ PhotoStoreable, PhotoStoreError };
use answer::{ AnswerResult, Answer, AnswerResponse };
use authentication::{ Userable, User };
use err_msg;
use time;
use time::{ Timespec };
use types::{ PhotoInfo, ImageType, ShortInfo, CommonError, Id };
use answer_types::{ PhotoErrorInfo };
use exif_reader;
use exif_reader::{ ExifValues };
use database::{ Databaseable };
use stuff::Stuffable;
use db::photos::{ DbPhotos };
use iron::prelude::*;
use params_body_parser::{ ParamsBody, BinParamsError };
use std::convert::From;
use parse_utils::{ GetMsecs };
use rustc_serialize::{ Encodable };
use super::utils::{ get_id };
use events::events_manager::CustomEventsManager;
use events::publication::{ self, PublishError };
use events::late_publication;
use events::EventId;

static IMAGE : &'static str = "upload_img";

pub fn upload_photo( request: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse( upload_photo_answer( request ) );
    Ok( Response::with( answer ) )
}

pub fn upload_and_publish_path() -> &'static str {
    "upload_and_publish/:id"
}

pub fn upload_and_publish_photo( request: &mut Request ) -> IronResult<Response> {
    let answer = AnswerResponse(
        upload_and_publish_answer( request )
    );
    Ok( Response::with( answer ) )
}

fn upload_photo_answer( request: &mut Request ) -> AnswerResult {
    match make_upload_photo( request ) {
        Ok( id ) => Ok( Answer::good( id ) ),
        Err( e ) => e.into()
    }
}

fn upload_and_publish_answer( req: &mut Request ) -> AnswerResult {
    let scheduled_id = match get_id( "id", req ) {
        Some( id ) => id,
        None => return Ok( Answer::not_found() )
    };

    match make_upload_photo( req ) {
        Ok( photo_id ) => {
            req.custom_event_action_post( scheduled_id, |_, event_info, req| {
                let result = match event_info.id {
                    EventId::Publication => publication::process_publish_photo( req, scheduled_id, photo_id ),
                    EventId::LatePublication => late_publication::process_publish_photo( req, event_info, photo_id ),
                    _ => PublishError::err( Answer::not_found() )
                };
                match result {
                    Ok( _ ) => Ok( Answer::good( photo_id ) ),
                    Err( e ) => e.into()
                }
            })
        },
        Err( e ) => e.into()
    }
}

enum UploadError
{
    CommonError( CommonError ),
    Bad( Answer )
}

impl UploadError {
    pub fn bad<Body: Encodable + 'static>(body: Body) -> Result<Id, UploadError> {
        Err( UploadError::Bad( Answer::bad( body ) ) )
    }

    pub fn from( err: CommonError ) -> Result<Id, UploadError> {
        Err( UploadError::CommonError( err ) )
    }
}

impl From<CommonError> for UploadError {
    fn from( e: CommonError ) -> UploadError {
        UploadError::CommonError( e )
    }
}

impl Into<AnswerResult> for UploadError {
    fn into( self ) -> AnswerResult {
        match self {
            UploadError::CommonError( ce ) => Err( ce ),
            UploadError::Bad( answer ) => Ok( answer )
        }
    }
}

fn common_error( s: String ) -> Result<Id, UploadError> {
    return Err( UploadError::CommonError( CommonError( s ) ) );
}

fn make_upload_photo( request: &mut Request ) -> Result<Id, UploadError> {
    let params = match request.parse_bin_params() {
        Ok( p ) => p,

        Err( BinParamsError::TooBig ) => {
            return UploadError::bad( PhotoErrorInfo::too_big() );
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
        None => return UploadError::from( err_msg::param_not_found( IMAGE ) )
    };
    let image_filename = match image.filename {
        Some( ref filename ) => filename,
        None => return UploadError::from( err_msg::invalid_type_param( IMAGE ) )
    };
    match check_image_type( &image_filename ) {
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
                            Ok( added_id )
                        },
                        Err( e ) => common_error( format!( "{}", e ) )
                    }
                }
                Err( e ) => match e {
                    PhotoStoreError::Image( e ) => UploadError::from( CommonError::from( e ) ),
                    PhotoStoreError::Fs( e ) => UploadError::from( err_msg::fs_error( e ) ),
                    PhotoStoreError::Format => UploadError::bad( PhotoErrorInfo::bad_image() )
                }
            }
        }

        None => UploadError::bad( PhotoErrorInfo::unknown_format() )
    }
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
