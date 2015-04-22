use iron::{ Request, headers };

pub trait Cookieable {
    fn cookie( &self, &str ) -> Option<&String>;
}

impl<'a, 'b> Cookieable for Request<'a, 'b> {
    fn cookie(&self, key: &str) -> Option<&String> {
        self.headers.get::<headers::Cookie>()
            .and_then( |cookies| cookies.iter().find( |&c| c.name == key ) )
            .map( |cookie| &cookie.value )
    }
}
