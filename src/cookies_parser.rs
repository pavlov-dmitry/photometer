/// Простой парсер печенек, оформленный в виде middleware

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use std::collections::HashMap;
use typemap::Assoc;
use plugin::Extensible;

pub trait Cookieable {
    fn cookie( &self, &str ) -> Option<&String>;
}

pub type StringHashMap = HashMap<String, String>;

#[deriving(Clone)]
pub struct CookiesParser;

impl Assoc<StringHashMap> for CookiesParser {}

impl Middleware for CookiesParser {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        let mut cookies = HashMap::new();
        req.origin.headers.extensions
            .find_with( |k| {
                "Cookie".cmp( k.as_slice() )
            })
            .map( |value| {
                for ref cookie in value.as_slice().split( ';' ) {
                    let cookie_parts : Vec<&str> = cookie.split( '=' ).collect();
                    if cookie_parts.len() == 2 {
                        cookies.insert( cookie_parts[ 0 ].to_string(), cookie_parts[ 1 ].to_string() );
                    }
                }
            });
        req.extensions_mut().insert::<CookiesParser, StringHashMap>( cookies );
        Ok( Continue )
    } 
}

impl<'a, 'b> Cookieable for Request<'a, 'b> {
    fn cookie(&self, key: &str) -> Option<&String> {
        self.extensions().get::<CookiesParser, StringHashMap>()
            .and_then( |hash| hash.get( &key.to_string() ) )
    }
}

pub fn middleware() -> CookiesParser {
    CookiesParser    
}