/// Простой парсер печенек, оформленный в виде middleware

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use std::collections::HashMap;

struct Cookies(HashMap<String, String>);

#[deriving(Clone)]
pub struct CookiesParser;

impl Middleware for CookiesParser {
	fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
		let mut cookies = Cookies( HashMap::new() );
		req.origin.headers.extensions
			.find_with( |k| {
				"Cookie".cmp( &k.as_slice() )
			})
			.map( |value| {
				for ref cookie in value.as_slice().split( ';' ) {
					let cookie_parts : Vec<&str> = cookie.split( '=' ).collect();
					if cookie_parts.len() == 2 {
						let  Cookies( ref mut cookies_map ) = cookies;
						cookies_map.insert( cookie_parts[ 0 ].to_string(), cookie_parts[ 1 ].to_string() );
					}
				}
			});
		req.map.insert( cookies );
        Ok( Continue )
    } 
}

pub trait Cookieable {
	fn cookie( &self, &str ) -> Option<&String>;
}

impl<'a, 'b> Cookieable for Request<'a, 'b> {
    fn cookie(&self, key: &str) -> Option<&String> {
    	self.map.find::<Cookies>()
    		.and_then( |c| {
    			let &Cookies( ref hash ) = c;
    			hash.find( &key.to_string() )
    		})
    }
}

pub fn middleware() -> CookiesParser {
    CookiesParser    
}