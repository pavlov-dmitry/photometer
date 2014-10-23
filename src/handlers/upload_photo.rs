use nickel::{ Request, Response };
use photo_store::{ PhotoStoreable };
use photo_event;
use answer;
use answer::{ AnswerSendable };
use params_body_parser::{ ParamsBody };
use authentication::{ Userable };
use super::err_msg;
use std::from_str::{ from_str };

static WEEK : &'static str = "week";
static IMAGE : &'static str = "upload_img";

pub fn upload_photo( request: &Request, response: &mut Response ) {
    let answer_result = 
        request.bin_parameter_str( WEEK ).ok_or( err_msg::param_not_found( WEEK ) )
        .and_then( |week_str| from_str::<uint>( week_str.as_slice() ).ok_or( err_msg::invalid_type_param( WEEK ) ) )
        .and_then( 
            |week| request.bin_parameter( IMAGE ).ok_or( err_msg::param_not_found( IMAGE ) )
                .and_then( |img_data| {
                    let event = photo_event::create_weekly( 2014, week );
                    match request.photo_store().add_new_photo( request.user(), &event, img_data ) {
                        Ok(_) => {
                            let mut answer = answer::new();
                            answer.add_record( "photo_loaded", "ok" );
                            Ok( answer )
                        }
                        Err( e ) => Err( err_msg::fs_error( e ) )
                    }
                })
        );

    response.send_answer( &answer_result );
}