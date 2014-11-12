use nickel::{ Request, Response };
use photo_store::{ PhotoStoreable, AllCorrect, FsError, FormatError, FileSizeError };
use answer;
use answer::{ AnswerSendable };
use super::get_param::{ GetParamable };
use authentication::{ Userable };
use super::err_msg;
use time;
use time::{ Timespec };
use photo_info::{ PhotoInfo, ImageType };
use exif_reader;
use exif_reader::{ ExifValues };
use database::{ Databaseable };

static IMAGE : &'static str = "upload_img";
static IMAGE_FILENAME : &'static str = "upload_img_filename";


pub fn upload_photo( request: &Request, response: &mut Response ) {
    let answer_result = 
        request.get_param( IMAGE_FILENAME )
            .and_then( |filename| request.get_param_bin( IMAGE )
                .and_then( |img_data| {
                    let mut answer = answer::new();
                    match check_image_type( filename ) {
                        Some( tp ) => {
                            let photo_store = request.photo_store();
                            let upload_time = time::get_time();
                            match photo_store.add_new_photo( request.user(), &upload_time, tp, img_data ) {
                                AllCorrect( w, h ) => {
                                    let mut db = request.db();
                                    match db.add_photo( request.user().id, &make_photo_info( upload_time, tp, w, h, img_data ) ) {
                                        Ok( _ ) => answer.add_record( "photo_loaded", "ok" ),
                                        Err( e ) => panic!( e )
                                    }
                                }
                                FsError( e ) => panic!( err_msg::fs_error( e ) ),
                                FormatError => answer.add_record( "photo", "bad_image" ),
                                FileSizeError => answer.add_error( "photo", "too_big" ),
                            }
                        }
                        None => answer.add_record( "photo", "unknown_format" )
                    }
                    Ok( answer )
                })
            );


    response.send_answer( &answer_result );
}

fn make_photo_info( upload_time: Timespec, img_type: ImageType, w: u32, h: u32, img_data: &[u8] ) -> PhotoInfo {
    let exif = exif_reader::from_memory( img_data );
    let exif_ref = exif.as_ref();
    PhotoInfo {
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