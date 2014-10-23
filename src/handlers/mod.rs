use nickel::{ Request, Response };
use std::io::{ File };
use params_body_parser::{ ParamsBody };
use authentication::{ Userable };

pub use self::authentication::{ login, join_us };

pub mod authentication;

pub fn hello ( request: &Request, response: &mut Response) { 
    let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
    response.send( answer );
}

pub fn test_upload( request: &Request, response: &mut Response ) {
    match request.bin_parameter( "upload_img" ) {
        Some( data ) => {
            let path = Path::new( "../data/img.jpg" );
            let mut writed = false;
            {
                File::create( &path )
                    .and_then( |mut file| file.write( data ) )
                    .map( |_| writed = true )
                    .unwrap_or_else( |e| response.send( format!( "{}", e ) ) )
            }
            if writed == true {
                response.send_file( &path );
            }
        }
        None => response.send( "no upload_img" )
    }

}