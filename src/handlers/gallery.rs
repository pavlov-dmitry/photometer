use answer::{ AnswerResult, Answer, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use authentication::{ Userable };
use db::photos::{ DbPhotos };
use iron::prelude::*;
use get_body::GetBody;
use answer_types::CountInfo;
use types::{ Id, PhotoInfo };
use answer_types::PhotoErrorInfo;

// static YEAR: &'static str = "year";
const IN_PAGE_COUNT: u32 = 12;
// const FROM_YEAR: i32 = 1900;

pub fn gallery_count_path() -> &'static str {
    "/gallery/count"
}

// pub fn by_year_count_path() -> &'static str {
//     "/gallery/:year/count"
// }


pub fn gallery_path() -> &'static str {
    "/gallery"
}

// pub fn by_year_path() -> &'static str {
//     "/gallery/:year"
// }

pub fn photo_info_path() -> &'static str {
    "gallery/photo_info"
}

pub fn gallery_count( request: &mut Request ) -> IronResult<Response> {
    let response = AnswerResponse( gallery_count_answer( request ) );
    Ok( Response::with( response ) )
}

// pub fn by_year_count( request: &mut Request ) -> IronResult<Response> {
//     let year = FromStr::from_str( request.param( YEAR ) );
//     let answer = match year {
//         Ok( year ) => by_year_count_answer( request, year ),
//         Err( _ ) => Err( err_msg::invalid_path_param( YEAR ) )
//     };
//     let response = AnswerResponse( answer );
//     Ok( Response::with( response ) )
// }

pub fn gallery( request: &mut Request ) -> IronResult<Response> {
    let response = AnswerResponse( gallery_answer( request ) );
    Ok( Response::with( response ) )
}

// pub fn by_year( request: &mut Request ) -> IronResult<Response> {
//     let year = FromStr::from_str( request.param( YEAR ) );
//     let answer = match year {
//         Ok( year ) => by_year_answer( request, year ),
//         Err( _ ) => Err( err_msg::invalid_path_param( YEAR ) )
//     };
//     let response = AnswerResponse( answer );
//     Ok( Response::with( response ) )
// }

fn gallery_count_answer( req: &mut Request ) -> AnswerResult {
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let photos_count = try!( db.get_photo_infos_count( user_id ) );
    let answer = Answer::good( CountInfo::new( photos_count ) );
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct PageInfo {
    page: u32
}

#[derive(RustcEncodable)]
struct GalleryInfo {
    owner_id: Id,
    current_page: u32,
    pages_count: u32,
    photos: Vec<PhotoInfo>
}

fn gallery_answer( req: &mut Request ) -> AnswerResult {
    let page = try!( req.get_body::<PageInfo>() ).page;

    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );

    let photos_count = try!( db.get_photo_infos_count( user_id ) );

    let photo_infos = try!( db.get_photo_infos(
        user_id,
        page * IN_PAGE_COUNT,
        IN_PAGE_COUNT
    ) );

    let mut pages_count = photos_count / IN_PAGE_COUNT;
    if ( photos_count % IN_PAGE_COUNT ) != 0 {
        pages_count += 1;
    }

    let gallery_info = GalleryInfo {
        owner_id: user_id,
        current_page: page,
        pages_count: pages_count,
        photos: photo_infos
    };
    let answer = Answer::good( gallery_info );
    Ok( answer )
}

// fn times_gate_for_year( year: i32 ) -> (time::Tm, time::Tm) {
//     (
//         time::Tm {
//             tm_year: year - FROM_YEAR,
//             ..time::empty_tm()
//         },
//         time::Tm {
//             tm_year: year - FROM_YEAR,
//             tm_mon: 11,
//             tm_mday: 31,
//             tm_hour: 23,
//             tm_min: 59,
//             tm_sec: 59,
//             ..time::empty_tm()
//         }
//     )
// }

#[derive(Clone, RustcDecodable)]
struct PhotoContextInfo {
    user: Id,
    photo: Id
}

#[derive(RustcEncodable)]
struct GalleryPhotoInfo {
    prev: Option<Id>,
    photo: PhotoInfo,
    next: Option<Id>
}

pub fn photo_info( request: &mut Request ) -> IronResult<Response> {
    let answer = photo_info_answer( request );
    let answer = AnswerResponse( answer );
    Ok( Response::with( answer ) )
}

fn photo_info_answer( req: &mut Request ) -> AnswerResult {
    let photo_context = try!( req.get_body::<PhotoContextInfo>() );

    let db = try!( req.stuff().get_current_db_conn() );
    let photo_info = try!( db.get_photo_info( photo_context.photo ) );
    let photo_info = match photo_info {
        Some( photo_info ) => photo_info,
        None => return Ok( Answer::bad( PhotoErrorInfo::not_found() ) )
    };

    let (prev, next) = try!( db.get_photo_neighbours_in_gallery( photo_context.user, photo_context.photo ) );

    let answer = Answer::good( GalleryPhotoInfo{
        prev: prev,
        photo: photo_info,
        next: next
    });
    Ok( answer )
}
