/// обёртка над bodyparser, для более удобной работы
use types::CommonResult;
use rustc_serialize::Decodable;
use iron::prelude::*;
use bodyparser;

pub trait GetBody {
    fn get_body<T: Decodable + Clone + 'static>( &mut self ) -> CommonResult<T>;
}

impl<'a> GetBody for Request<'a> {
    fn get_body<T: Decodable + Clone + 'static>( &mut self ) -> CommonResult<T> {
         match self.get::<bodyparser::Struct<T>>() {
            Ok( Some( value ) ) => Ok( value ),
            Ok( None ) => Err( String::from_str( "no body found." ) ),
            Err( e ) => Err( format!( "error parsing body: {:?}", e ) )
         }
    }
}
