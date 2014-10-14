///  Небольшая middleware которая парсит html параметры, в никеле елси параметра нет, то мы сразу падаем :(
 
extern crate nickel;
extern crate url;

use std::collections::HashMap;
use self::nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };

#[deriving(Clone)]
pub struct ParamsBodyParser;

impl Middleware for ParamsBodyParser {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {

        //println!( "______________________________" );
        //println!( "url={}", req.origin.request_uri );
        //println!( "method={}", req.origin.method );
        //println!( "body={}", req.origin.body );

        if !req.origin.body.is_empty() {
            let params_vec = url::form_urlencoded::parse_str( req.origin.body.as_slice() );
            let mut params_hash = HashMap::new();
            for &( ref key, ref value ) in params_vec.iter() {
                //println!( "{}={}", key, value );
                params_hash.insert( key.clone(), value.clone() );
            }
            req.map.insert( params_hash );
        }
        Ok( Continue )
    } 
}

pub trait ParamsBody {
    fn parameter_string(&self, &String) -> Option<&String>;
    fn parameter(&self, &str ) -> Option<&String>;
}

impl<'a, 'b> ParamsBody for Request<'a, 'b> {
    fn parameter_string(&self, key: &String) -> Option<&String> {
        self.map.find::<HashMap<String, String>>()
            .and_then( |ref hash| {
                hash.find( key )
            })
    }
    fn parameter(&self, key: &str ) -> Option<&String> {
        self.parameter_string( &key.to_string() )
    }
}

pub fn middleware() -> ParamsBodyParser {
    ParamsBodyParser
}