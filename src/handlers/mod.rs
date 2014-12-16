use nickel::{ Request, Response };
use authentication::{ Userable };

pub use self::authentication::{ login, join_us };
pub use self::upload_photo::{ upload_photo };
pub use self::crop::{ crop_photo };
pub use self::rename::{ rename_photo };

pub mod authentication;
pub mod upload_photo;
pub mod crop;
pub mod rename;
pub mod gallery;
pub mod images;
pub mod mailbox;

pub fn hello ( request: &Request, response: &mut Response) { 
    let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
    response.send( answer );
}