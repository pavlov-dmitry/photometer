use nickel::{ Request, Response };
use photo_store::{ PhotoStoreable, PhotoStoreError };
use answer::{ AnswerSendable, AnswerResult, Answer };
use get_param::{ GetParamable };
use authentication::{ Userable };
use err_msg;
use time;
use time::{ Timespec };
use types::{ PhotoInfo, ImageType };
use exif_reader;
use exif_reader::{ ExifValues };
use database::{ Databaseable };
use db::photos::{ DbPhotos };

static IMAGE : &'static str = "upload_img";
static IMAGE_FILENAME : &'static str = "upload_img_filename";


pub fn upload_photo( request: &Request, response: &mut Response ) {
    response.send_answer( &upload_photo_answer( request ) );
}

fn upload_photo_answer( request: &Request ) -> AnswerResult {
    let filename = try!( request.get_param( IMAGE_FILENAME ) );
    let img_data = try!( request.get_param_bin( IMAGE ) );
    let mut answer = Answer::new();
    match check_image_type( filename ) {
        Some( tp ) => {
            let photo_store = request.photo_store();
            let upload_time = time::get_time();
            match photo_store.add_new_photo( request.user(), &upload_time, tp.clone(), img_data ) {
                Ok( (w, h) ) => {
                    let mut db = try!( request.get_db_conn() );
                    match db.add_photo( request.user().id, &make_photo_info( upload_time, tp, w, h, img_data ) ) {
                        Ok( _ ) => answer.add_record( "photo_loaded", &String::from_str( "ok" ) ),
                        Err( e ) => panic!( e )
                    }
                }
                Err( e ) => match e {
                    PhotoStoreError::Fs( e ) => return Err( err_msg::fs_error( e ) ),
                    PhotoStoreError::Format => answer.add_error( "photo", "bad_image" ),
                    PhotoStoreError::FileSize => answer.add_error( "photo", "too_big" )
                }
            }
        }
        None => answer.add_record( "photo", &String::from_str( "unknown_format" ) )
    }
    Ok( answer )
}

fn make_photo_info( upload_time: Timespec, img_type: ImageType, w: u32, h: u32, img_data: &[u8] ) -> PhotoInfo {
    let exif = exif_reader::from_memory( img_data );
    let exif_ref = exif.as_ref();
    PhotoInfo {
        id: 0,
        upload_time: upload_time,
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