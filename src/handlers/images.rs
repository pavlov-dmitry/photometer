use err_msg;
use std::old_io as io;
use types::{ Id, CommonResult };
use database::{ Databaseable };
use db::photos::{ DbPhotos };
use photo_store::{ PhotoStoreable };
use std::str::FromStr;

static FILENAME : &'static str = "filename";

pub fn photos_path() -> &'static str {
    "/photo/:filename.:ext"
}

pub fn get_photo( req: &mut Request, res: &mut Response ) {
    get_image( req, res, false );
}

#[inline]
pub fn get_image( req: &mut Request, res: &mut Response, is_preview: bool ) {
    match get_image_impl( req, res, is_preview ) {
        Ok( _ ) => {},
        Err( e ) => { let _ = writeln!( &mut io::stderr(), "{}", e ); }
    }
}

pub fn get_image_impl( req: &mut Request, res: &mut Response, is_preview: bool ) -> CommonResult<()> {
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
            let _ = try!( 
                res.send_file( 
                    &req.photo_store().make_filename(
                        &user,
                        &info.upload_time,
                        &info.image_type,
                        is_preview
                    )
                ).map_err( |e| err_msg::fs_error( e ) )
            );
        },
        None => {}
    }
    Ok( () )
}

pub fn preview_path() -> &'static str {
    "/preview/:filename.:ext"
}

pub fn get_preview( req: &mut Request, res: &mut Response ) {
    get_image( req, res, true );
}