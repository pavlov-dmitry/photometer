use iron::Request;
use router::Router;

pub trait RouterParams {
    fn param<'x>( &'x self, prm : &str ) -> Option<&'x str>;
}

impl<'a, 'b> RouterParams for Request<'a, 'b>  {
    fn param<'x>( &'x self, prm : &str ) -> Option<&'x str> {
        self.extensions.get::<Router>().unwrap().find( prm )
    }
}
