/// Простой парсер печенек, оформленный в виде middleware

use std::collections::HashMap;
use iron::typemap::Key;
use iron::middleware::BeforeMiddleware;
use iron::prelude::*;

pub trait Cookieable {
    fn cookie( &self, &str ) -> Option<&String>;
}

pub type StringHashMap = HashMap<String, String>;

#[derive(Clone)]
pub struct CookiesParser;

impl Key for CookiesParser { type Value = StringHashMap; }

impl BeforeMiddleware for CookiesParser {
    fn before( &self, req: &mut Request ) -> IronResult<()> {
        let mut cookies = HashMap::new();
        req.origin.headers.extensions.get( "Cookie" )
            .map( |value| {
                for ref cookie in value.as_slice().split( ';' ) {
                    let cookie_parts : Vec<&str> = cookie.split( '=' ).collect();
                    if cookie_parts.len() == 2 {
                        cookies.insert( cookie_parts[ 0 ].to_string(), cookie_parts[ 1 ].to_string() );
                    }
                }
            });
        req.extensions_mut().insert::<CookiesParser>( cookies );
        Ok( () )
    } 
}

impl<'a, 'b> Cookieable for Request<'a, 'b> {
    fn cookie(&self, key: &str) -> Option<&String> {
        self.extensions().get::<CookiesParser>()
            .and_then( |hash| hash.get( &key.to_string() ) )
    }
}

pub fn middleware() -> CookiesParser {
    CookiesParser    
}