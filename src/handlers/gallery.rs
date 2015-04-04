use answer::{ AnswerResult, Answer };
use err_msg;
use time;
use std::str::FromStr;
use database::{ Databaseable };
use stuff::Stuffable;
use authentication::{ Userable };
use db::photos::{ DbPhotos };
use iron::prelude::*;
use iron::status;
use router_params::RouterParams;
use get_body::GetBody;
use answer_types::CountInfo;

static YEAR: &'static str = "year";
const IN_PAGE_COUNT: u32 = 10;
const FROM_YEAR: i32 = 1900;

pub fn current_year_count_path() -> &'static str {
    "/gallery/count"
}

pub fn by_year_count_path() -> &'static str {
    "/gallery/:year/count"
}


pub fn current_year_path() -> &'static str {
    "/gallery"
}

pub fn by_year_path() -> &'static str {
    "/gallery/:year"
}

pub fn current_year_count( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (
        status::Ok,
        by_year_count_answer( request, time::now().tm_year + FROM_YEAR )
    ) ) )
}

pub fn by_year_count( request: &mut Request ) -> IronResult<Response> {
    let year = FromStr::from_str( request.param( YEAR ) );
    let answer = match year {
        Ok( year ) => by_year_count_answer( request, year ),
        Err( _ ) => Err( err_msg::invalid_path_param( YEAR ) )
    };
    Ok( Response::with( (status::Ok, answer) ) )
}

pub fn current_year( request: &mut Request ) -> IronResult<Response> {
    Ok( Response::with( (status::Ok, by_year_answer( request, time::now().tm_year + FROM_YEAR )) ) )
}

pub fn by_year( request: &mut Request ) -> IronResult<Response> {
    let year = FromStr::from_str( request.param( YEAR ) );
    let answer = match year {
        Ok( year ) => by_year_answer( request, year ),
        Err( _ ) => Err( err_msg::invalid_path_param( YEAR ) )
    };
    Ok( Response::with( (status::Ok, answer) ) )
}

fn by_year_count_answer( req: &mut Request, year: i32 ) -> AnswerResult {
    let ( from, to ) = times_gate_for_year( year );
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let photos_count = try!( db.get_photo_infos_count(
        user_id,
        from.to_timespec(),
        to.to_timespec()
    ) );
    let answer = Answer::good( CountInfo::new( photos_count ) );
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct PageInfo {
    page: u32
}

fn by_year_answer( req: &mut Request, year: i32 ) -> AnswerResult {
    let page = try!( req.get_body::<PageInfo>() ).page;

    let ( from, to ) = times_gate_for_year( year );
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let photo_infos = try!( db.get_photo_infos(
        user_id,
        from.to_timespec(),
        to.to_timespec(),
        page * IN_PAGE_COUNT,
        IN_PAGE_COUNT
    ) );
    // for photo_info in photo_infos.iter() {
    //     answer.add_to_records( photo_info );
    // }
    let answer = Answer::good( photo_infos );
    Ok( answer )
}

fn times_gate_for_year( year: i32 ) -> (time::Tm, time::Tm) {
    (
        time::Tm {
            tm_year: year - FROM_YEAR,
            ..time::empty_tm()
        },
        time::Tm {
            tm_year: year - FROM_YEAR,
            tm_mon: 11,
            tm_mday: 31,
            tm_hour: 23,
            tm_min: 59,
            tm_sec: 59,
            ..time::empty_tm()
        }
    )
}
