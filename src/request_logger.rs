use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use typemap::Assoc;
use plugin::Extensible;
use log;

use simple_time_profiler::SimpleTimeProfiler;

#[deriving(Clone)]
struct RequestLogger;

impl Copy for RequestLogger {}

pub fn middleware() -> RequestLogger {
    RequestLogger
}

impl Middleware for RequestLogger {
    fn invoke<'a>(&self, req: &'a mut Request, _res: &mut Response) -> MiddlewareResult {
        if log_enabled!( log::INFO ) {
            info!( "_______________________________________" );
            let request = format!( "request {}:{}", req.origin.method, req.origin.request_uri );
            info!( "{}", request );
            req.extensions_mut().insert::<RequestLogger, SimpleTimeProfiler>( SimpleTimeProfiler::new( request.as_slice() ) );
        }
        Ok( Continue )
    }
}

impl Assoc<SimpleTimeProfiler> for RequestLogger {}