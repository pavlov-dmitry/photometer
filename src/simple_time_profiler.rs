use time;

pub struct SimpleTimeProfiler {
    msg: String,
    start_time_ns: u64
}

impl SimpleTimeProfiler {
    pub fn new( msg: &str ) -> SimpleTimeProfiler {
        SimpleTimeProfiler {
            msg: msg.to_string(),
            start_time_ns: time::precise_time_ns()
        }
    }
}

impl Drop for SimpleTimeProfiler {
    fn drop(&mut self) {
        info!( "{} - {} ms", self.msg, ( time::precise_time_ns() - self.start_time_ns ) / 1000000_u64 );
    }
}

/*pub fn time_measure<R>( msg: &str, do_this: ||-> R ) -> R {
    let _profiler = SimpleTimeProfiler::new( msg );
    do_this()
}*/

//судя по всему чтобы этот макрос заработал, его нужно выносить в отдельный crate
#[macro_export]
macro_rules! measure {
    ($expr:expr) => ({ let _profiler = simple_time_profiler::SimpleTimeProfiler::new( stringify!( $expr ) ); $expr })
}
