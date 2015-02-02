use iron::typemap::Key;
use log;
use iron::{Handler, AroundMiddleware};
use iron::prelude::*;

use simple_time_profiler::SimpleTimeProfiler;

#[derive(Clone)]
struct RequestLogger;

impl Copy for RequestLogger {}

pub fn middleware() -> RequestLogger {
    RequestLogger
}

struct RequestLoggerHandler<H: Handler> {
    handler: H
}

impl AroundMiddleware for RequestLogger {
   fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new( RequestLoggerHandler{
            handler: handler
        })
    }
}

impl<H: Handler> Handler for RequestLoggerHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        if log_enabled!( log::LogLevel::Info ) {
            info!( "_______________________________________" );
            let request = format!( "request {}:{}", req.origin.method, req.origin.request_uri );
            info!( "{}", request );
            let _profiler = SimpleTimeProfiler::new( request.as_slice() );
            self.handler.handle( req )
        }
        else {
            self.handler.handle( req )
        }
    }
}

impl Key for RequestLogger { type Value = SimpleTimeProfiler; }