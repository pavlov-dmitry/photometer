/// Считывание запросов в json формате
use types::{ CommonResult, CommonError, common_error };
use std::io::{ Read };
use rustc_serialize::{ Decodable, json };
use iron::prelude::*;
use iron::method::Method;
use url;

pub trait GetBody {
    fn get_body<T: Decodable + Clone + 'static>( &mut self ) -> CommonResult<T>;
}

const BODY_LIMIT: u64 = 2 * 1024 * 1024 ;

impl<'a, 'b> GetBody for Request<'a, 'b> {
    fn get_body<T: Decodable + Clone + 'static>( &mut self ) -> CommonResult<T> {
        // NOTE: для GET запросов мы ожидаем наш json на месте
        // параметров в url запроса, для остальных в теле запроса
        if self.method == Method::Get {
            match self.url.query {
                Some( ref query ) => {
                    let query = url::lossy_utf8_percent_decode( query.as_bytes() );
                    json::decode::<T>( &query )
                        .map_err( |e| CommonError( format!( "error parsing query: {:?}", e ) ) )
                }

                None => common_error( From::from( "no query part found" ) )
            }
        }
        else {
            let mut bytes = Vec::new();
            let mut limited_body = self.body.by_ref().take( BODY_LIMIT );
            match limited_body.read_to_end( &mut bytes ) {
                Ok( readed ) => {
                    if readed == BODY_LIMIT as usize {
                        return common_error( From::from( "request body too BIG!" ) );
                    }
                    // TODO: после обновления до beta заменить
                    // let body_as_str = String::from_ut8_lossy( &bytes );
                    let body_as_str = String::from_utf8( bytes ).unwrap_or( String::new() );
                    json::decode::<T>( &body_as_str )
                        .map_err( |e| CommonError( format!( "error parsing query: {:?}", e ) ) )
                },
                Err(_) => {
                    return common_error( From::from( "error while reading request body" ) )
                }
            }

        }
    }
}
