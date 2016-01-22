use answer::{ AnswerResult, Answer, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use authentication::{ Userable };
use db::photos::{ DbPhotos };
use db::group_feed::DbGroupFeed;
use iron::prelude::*;
use get_body::GetBody;
use answer_types::CountInfo;
use types::{ Id, PhotoInfo, ShortInfo };
use answer_types::{ PaginationInfo };

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

pub fn gallery_unpublished_path() -> &'static str {
    "/gallery/unpublished"
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

pub fn gallery( request: &mut Request ) -> IronResult<Response> {
    let response = AnswerResponse( gallery_answer( request ) );
    Ok( Response::with( response ) )
}

pub fn gallery_unpublished( request: &mut Request ) -> IronResult<Response> {
    let response = AnswerResponse( gallery_unpublished_answer( request ) );
    Ok( Response::with( response ) )
}

fn gallery_count_answer( req: &mut Request ) -> AnswerResult {
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let photos_count = try!( db.get_photo_infos_count( user_id ) );
    let answer = Answer::good( CountInfo::new( photos_count ) );
    Ok( answer )
}

pub fn get_publication( request: &mut Request ) -> IronResult<Response> {
    let response = AnswerResponse( publication_answer( request ) );
    Ok( Response::with( response ) )
}

pub fn get_publication_photo( request: &mut Request ) -> IronResult<Response> {
    let response = AnswerResponse( publication_photo_answer( request ) );
    Ok( Response::with( response ) )
}

#[derive(Clone, RustcDecodable)]
struct PageInfo {
    page: u32
}

#[derive(RustcEncodable)]
struct GalleryInfo {
    owner_id: Id,
    pagination: PaginationInfo,
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

    let pages_count = calc_page_count( photos_count );

    let gallery_info = GalleryInfo {
        owner_id: user_id,
        pagination: PaginationInfo {
            current: page,
            count: pages_count
        },
        photos: photo_infos
    };
    let answer = Answer::good( gallery_info );
    Ok( answer )
}

fn gallery_unpublished_answer( req: &mut Request ) -> AnswerResult {
    let page = try!( req.get_body::<PageInfo>() ).page;

    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );

    let unpublished_count = try!( db.get_unpublished_photos_count( user_id ) );
    let photos = try!( db.get_unpublished_photo_infos(
        user_id,
        page * IN_PAGE_COUNT,
        IN_PAGE_COUNT
    ));

    let pages_count = calc_page_count( unpublished_count );

    let gallery_info = GalleryInfo {
        owner_id: user_id,
        pagination: PaginationInfo {
            current: page,
            count: pages_count
        },
        photos: photos
    };
    let answer = Answer::good( gallery_info );
    Ok( answer )
}

fn calc_page_count( all_count: u32 ) -> u32 {
    let mut pages_count = all_count / IN_PAGE_COUNT;
    if ( all_count % IN_PAGE_COUNT ) != 0 {
        pages_count += 1;
    }
    pages_count
}

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
        None => return Ok( Answer::not_found() )
    };

    let (prev, next) = try!( db.get_photo_neighbours_in_gallery( photo_context.user, photo_context.photo ) );

    let answer = Answer::good( GalleryPhotoInfo{
        prev: prev,
        photo: photo_info,
        next: next
    });
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct PublicationPhotoQuery {
    feed_id: Id,
    photo_id: Id
}

#[derive(RustcEncodable)]
struct PublicationPhotoInfo {
    group: ShortInfo,
    feed: ShortInfo,
    prev: Option<Id>,
    photo: PhotoInfo,
    next: Option<Id>
}

fn publication_photo_answer( req: &mut Request ) -> AnswerResult {
    let photo_query = try!( req.get_body::<PublicationPhotoQuery>() );
    let user_id = req.user().id;

    let db = try!( req.stuff().get_current_db_conn() );
    let feed_info = try!( db.get_feed_info( user_id, photo_query.feed_id ) );
    let feed_info = match feed_info {
        Some( info ) => info,
        None => return Ok( Answer::not_found() )
    };
    let photo_info = try!( db.get_photo_info( photo_query.photo_id ) );
    let photo_info = match photo_info {
        Some( photo_info ) => photo_info,
        None => return Ok( Answer::not_found() )
    };
    let (prev, next) = try!( db.get_photo_neighbours_in_publication(
        feed_info.scheduled_id,
        photo_query.photo_id
    ) );
    let answer = Answer::good( PublicationPhotoInfo{
        feed: ShortInfo {
            id: feed_info.id,
            name: feed_info.event_name
        },
        group: feed_info.group,
        prev: prev,
        photo: photo_info,
        next: next
    });
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct PublicationQuery {
    id: Id
}

fn publication_answer( req: &mut Request ) -> AnswerResult {
    let scheduled_id = try!( req.get_body::<PublicationQuery>() ).id;
    let db = try!( req.stuff().get_current_db_conn() );
    let photos = try!( db.get_publication_photo_infos( scheduled_id ) );
    Ok( Answer::good( photos ) )
}
