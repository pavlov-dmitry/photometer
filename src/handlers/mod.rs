use nickel::{ Request, Response };
use authentication::{ Userable };

pub use self::authentication::{ login, join_us };
pub use self::upload_photo::{ upload_photo };

pub mod authentication;
pub mod upload_photo;

mod err_msg;

pub fn hello ( request: &Request, response: &mut Response) { 
    let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
    response.send( answer );
}