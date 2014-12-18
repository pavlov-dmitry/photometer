use err_msg;
use nickel::{ Request, Response };
use std::io;
use types::{ Id, CommonResult };
use database::{ Databaseable };
use db::photos::{ DbPhotos };
use photo_store::{ PhotoStoreable };

static FILENAME : &'static str = "filename";

pub fn photos_path() -> &'static str {
    "/photo/:filename.:ext"
}

pub fn get_photo( req: &Request, res: &mut Response ) {
    get_image( req, res, false );
}

#[inline]
pub fn get_image( req: &Request, res: &mut Response, is_preview: bool ) {
    match get_image_impl( req, res, is_preview ) {
        Ok( _ ) => {},
        Err( e ) => { let _ = writeln!( &mut io::stderr(), "{}", e ); }
    }
}

pub fn get_image_impl( req: &Request, res: &mut Response, is_preview: bool ) -> CommonResult<()> {
    let image_id = try!( 
        from_str::<Id>( req.param( FILENAME ) )
            .ok_or( err_msg::invalid_type_param( FILENAME ) ) 
    );
    let mut db = try!( req.get_db_conn() );
    let maybe_info = try!( db.get_photo_info( image_id ) );
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

pub fn get_preview( req: &Request, res: &mut Response ) {
    get_image( req, res, true );
}