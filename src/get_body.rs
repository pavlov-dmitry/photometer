/// обёртка над bodyparser, для более удобной работы
use types::CommonResult;
use rustc_serialize::{ Decodable, json };
use iron::prelude::*;
use iron::method::Method;
use bodyparser;
use url;

pub trait GetBody {
    fn get_body<T: Decodable + Clone + 'static>( &mut self ) -> CommonResult<T>;
}

impl<'a> GetBody for Request<'a> {
    fn get_body<T: Decodable + Clone + 'static>( &mut self ) -> CommonResult<T> {
        // NOTE: для GET запросов мы ожидаем наш json на месте
        // параметров в url запроса, для остальных в теле запроса
        if self.method == Method::Get {
            match self.url.query {
                Some( ref query ) => {
                    let query = url::lossy_utf8_percent_decode( query.as_bytes() );
                    json::decode::<T>( &query )
                        .map_err( |e| format!( "error parsing query: {:?}", e ) )
                }

                None => Err( String::from_str( "no query part found" ) )
            }
        }
        else {
            match self.get::<bodyparser::Struct<T>>() {
                Ok( Some( value ) ) => Ok( value ),
                Ok( None ) => Err( String::from_str( "no body found." ) ),
                Err( e ) => Err( format!( "error parsing body: {:?}", e ) )
            }
        }
    }
}
