use nickel::{ Request, Response };
use answer;
use answer::{ AnswerSendable, AnswerResult };
use super::err_msg;
use time;
use super::get_param::{ GetParamable };
use std::from_str::{ from_str };
use database::{ Databaseable };
use authentication::{ Userable };

static YEAR : &'static str = "year";
static PAGE : &'static str = "page";
const IN_PAGE_COUNT : u32 = 10;

pub fn current_year_path() -> &'static str {
    "/gallery"
}

pub fn by_year_path() -> &'static str {
    "/gallery/:year"
}

pub fn current_year( request: &Request, response: &mut Response ) {
     response.send_answer( &by_year_answer( request, time::now().tm_year ) );
}

pub fn by_year( request: &Request, response: &mut Response ) {
    let year = from_str::<i32>( request.param( YEAR ) );
    let answer = match year {
        Some( year ) => by_year_answer( request, year ),
        None => Err( err_msg::invalid_path_param( YEAR ) )
    };
    response.send_answer( &answer );
}

fn by_year_answer( req: &Request, year: i32 ) -> AnswerResult {
    let mut answer = answer::new();
    let page = req.get_param_uint( PAGE ).unwrap_or( 0 ) as u32;

    let from = time::Tm {
        tm_year: year,
        ..time::empty_tm()
    };
    let to = time::Tm {
        tm_year: year,
        tm_mon: 11,
        tm_mday: 31,
        tm_hour: 23,
        tm_min: 59,
        tm_sec: 59,
        ..time::empty_tm()
    };

    let photo_infos = try!( req.db().get_photo_infos(  
        req.user().id,
        from.to_timespec(),
        to.to_timespec(),
        page * IN_PAGE_COUNT,
        IN_PAGE_COUNT
    ) );
    answer.add_record( "photo_count", photo_infos.len().to_string().as_slice() );
    for photo_info in photo_infos.iter() {
        answer.add_photo_info( photo_info );
    }
    Ok( answer )
}