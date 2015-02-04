use iron::Request;
use iron::typemap;
use router::Router;

pub trait RouterParams {
    fn param<'x>( &'x self, prm : &str ) -> &'x str;
}

impl<'a> RouterParams for Request<'a>  {
    fn param<'x>( &'x self, prm : &str ) -> &'x str {
        self.extensions.get::<Router>().unwrap().find( prm ).unwrap()
    }
}