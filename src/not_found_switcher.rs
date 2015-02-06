use iron::prelude::*;
use iron::{ Handler, AroundMiddleware };
use iron::status;

#[derive(Clone)]
pub struct NotFoundSwitcher<H: Handler> {
    switch_to: Box<H>
}

impl<H: Handler> NotFoundSwitcher<H> {
    pub fn new( handler: H ) -> NotFoundSwitcher<H> {
        NotFoundSwitcher {
            switch_to : Box::new( handler )
        }
    }
}

impl<H: Handler> AroundMiddleware for NotFoundSwitcher<H> {
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

impl<H: Handler> Handler for NotFoundSwitcherHandler<H> {
    fn handle( &self, req: &mut Request ) -> IronResult<Response> {
        let response = self.handler.handle( req );
        match response {
            Ok( Response{ status, ..} ) if status == Some( status::NotFound ) => {
                debug!( "make switch" );
                self.switch_to.handle( req )
            }
            Err( IronError{ response: Response{ status, .. }, ..} ) if status == Some( status::NotFound ) => {
                debug!( "make switch" );
                self.switch_to.handle( req )    
            }
            _ => response
        }
    }
}