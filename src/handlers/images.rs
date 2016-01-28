use types::{ Id };
use database::{ Databaseable };
use stuff::Stuffable;
use db::photos::{ DbPhotos };
use photo_store::{ PhotoStoreable };
use std::str::FromStr;
use iron::prelude::*;
use iron::status;
use router_params::RouterParams;
use std::path::Path;

static FILENAME : &'static str = "filename";

pub fn photos_path() -> &'static str {
    "/photo/:filename"
}

pub fn get_photo( req: &mut Request ) -> IronResult<Response> {
    get_image( req, false )
}

fn image_id_from_filename( filename: &str ) -> Option<Id> {
    filename.split( '.' )
        .next() // берем только первый элемент
        .and_then( |name| FromStr::from_str( name ).ok() )
}

macro_rules! try_notfound{
    ($expr:expr) => ({
        match $expr {
            Ok( val ) => val,
            Err( _ ) => return Ok( Response::with( status::NotFound ) )
        }
    })
}

/*
fn mime_from_image_type( t: ImageType ) -> mime::Mime {
    match t {
        ImageType::Jpeg => mime::Mime( mime::TopLevel::Image, mime::SubLevel::Jpeg, Vec::new() ),
        ImageType::Png => mime::Mime( mime::TopLevel::Image, mime::SubLevel::Png, Vec::new() )
    }
}
*/

pub fn get_image( req: &mut Request, is_preview: bool ) -> IronResult<Response> {
    let image_id = match image_id_from_filename( req.param( FILENAME ) ) {
        Some( id ) => id,
        None => return Ok( Response::with( status::NotFound ) )
    };
    let maybe_info = {
        let db = try_notfound!( req.stuff().get_current_db_conn() );
        try_notfound!( db.get_short_photo_info( image_id ) )
    };
    match maybe_info {
        Some( info ) => {
            Ok( Response::with( (
                status::Ok,
                //временно используем Path, пока image не перейдёт на новое io
                Path::new( &req.photo_store().make_filename(
                    &info.owner.name,
                    info.upload_time,
                    &info.image_type,
                    is_preview
                ))
            )))
        },
        None => {
            Ok( Response::with( status::NotFound ) )
        }
    }
}

pub fn preview_path() -> &'static str {
    "/preview/:filename"
}

pub fn get_preview( req: &mut Request ) -> IronResult<Response> {
    get_image( req, true )
}
