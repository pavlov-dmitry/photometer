use nickel::{ Request, Response };
use photo_store::{ PhotoStoreable, AllCorrect, FsError, FormatError, FileSizeError };
use answer;
use answer::{ AnswerSendable };
use super::get_param::{ GetParamable };
use authentication::{ Userable };
use super::err_msg;
use time;

static IMAGE : &'static str = "upload_img";

pub fn upload_photo( request: &Request, response: &mut Response ) {
    let answer_result = 
        request.get_param_bin( IMAGE )
            .and_then( |img_data| {
                let photo_store = request.photo_store();
                //проверка на размер фото
                let upload_time = time::get_time();
                match photo_store.add_new_photo( request.user(), &upload_time, img_data ) {
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
            });

    response.send_answer( &answer_result );
}