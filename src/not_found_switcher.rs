use iron::prelude::*;
use iron::{ Handler, AroundMiddleware };
use iron::status;

#[derive(Clone)]
pub struct NotFoundSwitcher<H: Handler> {
	switch_to: H
}

impl NotFoundSwitcher {
	pub fn new<H: Handler>( handler: H ) {
		NotFoundSwitcher {
			switch_to : handler
		}
	}
}

impl AroundMiddleware for NotFoundSwitcher {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new( NotFoundSwitcherHandler {
			handler: handler,
			switch_to: self.switch_to
		})
	}
}

struct NotFoundSwitcherHandler<H: Handler> {
	handler: H,
	switch_to: H
}

impl Handler for NotFoundSwitcherHandler {
	fn handle( &self, req: &mut Request ) -> IronResult<Response> {
		let response = self.handler.handle( req );
		match response {
			Ok( Response{ status, ..} ) if status == Some( status::NotFound ) => self.switch_to.handle( req ),
			_ => response
		}
	}
}