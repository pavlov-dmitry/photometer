use err_msg;
use std::old_io as io;
use types::{ Id, CommonResult };
use database::{ Databaseable };
use db::photos::{ DbPhotos };
use photo_store::{ PhotoStoreable };
use std::str::FromStr;
use iron::prelude::*;
use iron::status;

static FILENAME : &'static str = "filename";

pub fn photos_path() -> &'static str {
    "/photo/:filename.:ext"
}

pub fn get_photo( req: &mut Request ) -> IronResult<Response> {
    get_image( req, false )
}

pub fn get_image( req: &mut Request, is_preview: bool ) -> IronResult<Response> {
    let image_id = try!( 
        FromStr::from_str( req.param( FILENAME ) )
            .ok_or( err_msg::invalid_type_param( FILENAME ) ) 
    );
    let maybe_info = {
        let db = try!( req.get_current_db_conn() );
        try!( db.get_photo_info( image_id ) )
    };
    match maybe_info {
        Some( (user, info) ) => {
            Ok( Response::with( (
                status::Ok, 
                req.photo_store().make_filename(
                    &user,
                    &info.upload_time,
                    &info.image_type,
                    is_preview
                )
            )))            
        },
        None => {
            Ok( Response::with( status::NotFound ) )
        }
    }
}

pub fn preview_path() -> &'static str {
    "/preview/:filename.:ext"
}

pub fn get_preview( req: &mut Request, res: &mut Response ) {
    get_image( req, res, true );
}