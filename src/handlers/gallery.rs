use answer::{ AnswerResult, Answer, AnswerResponse };
use database::{ Databaseable };
use stuff::Stuffable;
use authentication::{ Userable };
use db::photos::{ DbPhotos };
use db::group_feed::DbGroupFeed;
use db::users::DbUsers;
use iron::prelude::*;
use get_body::GetBody;
use answer_types::CountInfo;
use types::{ Id, PhotoInfo, ShortInfo };
use answer_types::{ PaginationInfo };
use super::helpers::make_pagination;

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
struct GalleryQueryInfo {
    user_id: Id,
    page: u32
}

#[derive(Clone, RustcDecodable)]
struct PageInfo {
    page: u32
}

#[derive(RustcEncodable)]
struct GalleryInfo {
    is_own: bool,
    owner: ShortInfo,
    pagination: PaginationInfo,
    photos: Vec<PhotoInfo>
}

fn gallery_answer( req: &mut Request ) -> AnswerResult {
    let query = try!( req.get_body::<GalleryQueryInfo>() );

    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );

    let owner_info = match try!( db.user_by_id( query.user_id ) ) {
        Some( info ) => info,
        None => return Ok( Answer::not_found() )
    };
    let photos_count = try!( db.get_photo_infos_count( query.user_id ) );

    let photo_infos = try!( db.get_photo_infos(
        user_id,
        query.user_id,
        query.page * IN_PAGE_COUNT,
        IN_PAGE_COUNT
    ) );

    let gallery_info = GalleryInfo {
        is_own: user_id == query.user_id,
        owner: ShortInfo {
            id: owner_info.id,
            name: owner_info.name
        },
        pagination: make_pagination( query.page, photos_count, IN_PAGE_COUNT ),
        photos: photo_infos
    };
    let answer = Answer::good( gallery_info );
    Ok( answer )
}

fn gallery_unpublished_answer( req: &mut Request ) -> AnswerResult {
    let page = try!( req.get_body::<PageInfo>() ).page;

    let user = req.user().clone();
    let db = try!( req.stuff().get_current_db_conn() );

    let unpublished_count = try!( db.get_unpublished_photos_count( user.id ) );
    let photos = try!( db.get_unpublished_photo_infos(
        user.id,
        user.id,
        page * IN_PAGE_COUNT,
        IN_PAGE_COUNT
    ));

    let gallery_info = GalleryInfo {
        owner: ShortInfo {
            id: user.id,
            name: user.name
        },
        is_own: true,
        pagination: make_pagination( page, unpublished_count, IN_PAGE_COUNT ),
        photos: photos
    };
    let answer = Answer::good( gallery_info );
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct PhotoContextInfo {
    user: Id,
    photo: Id
}

#[derive(RustcEncodable)]
struct GalleryPhotoInfo {
    is_own: bool,
    photo: PhotoInfo
}

pub fn photo_info( request: &mut Request ) -> IronResult<Response> {
    let answer = photo_info_answer( request );
    let answer = AnswerResponse( answer );
    Ok( Response::with( answer ) )
}

fn photo_info_answer( req: &mut Request ) -> AnswerResult {
    let photo_context = try!( req.get_body::<PhotoContextInfo>() );
    let user_id = req.user().id;

    let db = try!( req.stuff().get_current_db_conn() );
    let photo_info = try!( db.get_gallery_photo_info( user_id, photo_context.photo ) );
    let photo_info = match photo_info {
        Some( photo_info ) => photo_info,
        None => return Ok( Answer::not_found() )
    };

    let info = GalleryPhotoInfo {
        is_own: user_id == photo_info.owner.id,
        photo: photo_info
    };
    let answer = Answer::good( info );
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
    photo: PhotoInfo,
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
    let photo_info = try!( db.get_publication_photo_info( user_id, photo_query.photo_id ) );
    let photo_info = match photo_info {
        Some( photo_info ) => photo_info,
        None => return Ok( Answer::not_found() )
    };
    let answer = Answer::good( PublicationPhotoInfo{
        feed: ShortInfo {
            id: feed_info.id,
            name: feed_info.event_name
        },
        group: feed_info.group,
        photo: photo_info,
    });
    Ok( answer )
}

#[derive(Clone, RustcDecodable)]
struct PublicationQuery {
    id: Id
}

fn publication_answer( req: &mut Request ) -> AnswerResult {
    let scheduled_id = try!( req.get_body::<PublicationQuery>() ).id;
    let user_id = req.user().id;
    let db = try!( req.stuff().get_current_db_conn() );
    let photos = try!( db.get_publication_photo_infos( user_id, scheduled_id ) );
    Ok( Answer::good( photos ) )
}
