/// Middleware поставщик окружения для запросов, будет хранить всё необходимые указатели 
/// на всё окружение (БД и прочее)

extern crate nickel;

use self::nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use authentication::SessionsStore;
use std::sync::{ Arc, RWLock };

#[deriving(Clone)]
struct Stuff {
	pub sessions_store_for : Arc<RWLock<SessionsStore>>
}

/*impl Stuff {
	
}*/

impl Middleware for Stuff {
	fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
		req.map.insert( self.clone() );
		Ok( Continue )
	}
}

pub trait Stuffable {
	fn stuff(&self) -> &Stuff;
}

impl<'a, 'b> Stuffable for Request<'a, 'b> {
    fn stuff(&self) -> &Stuff {
    	self.map.find::<Stuff>().unwrap()
    }
}

pub fn new( sess_store: SessionsStore  ) -> Stuff {
	Stuff{ sessions_store_for : Arc::new( RWLock::new( sess_store ) ) }
}