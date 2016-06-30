use authentication::{ Userable };
use iron::status;
use iron::prelude::*;

pub use self::authentication::{ login, join_us };
pub use self::upload_photo::{
    upload_photo,
    upload_and_publish_path,
    upload_and_publish_photo
};
pub use self::crop::{ crop_photo };
pub use self::rename::{ rename_photo };

pub mod authentication;
pub mod upload_photo;
pub mod crop;
pub mod rename;
pub mod gallery;
pub mod images;
pub mod mailbox;
pub mod events;
pub mod group;
pub mod search;
pub mod comments;
pub mod helpers;

mod utils;

pub fn hello ( request: &mut Request ) -> IronResult<Response> {
    let answer = format!( "Hello {}!!! Glad to see you!", request.user().name );
    Ok( Response::with( (status::Ok, answer) ) )
}
