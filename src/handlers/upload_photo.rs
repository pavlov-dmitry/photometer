use nickel::{ Request, Response };
use photo_store::{ PhotoStoreable, AllCorrect, FsError, FormatError, FileSizeError };
use photo_event;
use answer;
use answer::{ AnswerSendable };
use super::get_param::{ GetParamable };
use authentication::{ Userable };
use super::err_msg;

static WEEK : &'static str = "week";
static IMAGE : &'static str = "upload_img";

pub fn upload_photo( request: &Request, response: &mut Response ) {
    let answer_result = 
        request.get_param_uint( WEEK )
            .and_then( |week| request.get_param_bin( IMAGE )
                .and_then( |img_data| {
                    let photo_store = request.photo_store();
                    //проверка на размер фото
                    let event = photo_event::create_weekly( 2014, week );
                    match photo_store.add_new_photo( request.user(), &event, img_data ) {
                        AllCorrect => {
                            let mut answer = answer::new();
                            answer.add_record( "photo_loaded", "ok" );
                            Ok( answer )
                        },
                        FsError( e ) => Err( err_msg::fs_error( e ) ),
                        FormatError => {
                            let mut answer = answer::new();
                            answer.add_record( "photo", "bad_image" );
                            Ok( answer )  
                        },
                        FileSizeError => {
                            let mut answer = answer::new();
                            answer.add_error( "photo", "too_big" );
                            Ok( answer )   
                        }
                    }
                })
        );

    response.send_answer( &answer_result );
}