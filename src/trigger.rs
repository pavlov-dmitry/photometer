/// поток событий по таймеру
use std::thread;
#[allow(deprecated)]
use std::old_io::timer::Timer;
use std::time::duration::Duration;
use std::io::stderr;
use std::io::Write;
use types::EmptyResult;
use events::events_manager::EventsManagerStuff;

use stuff::{ Stuff, StuffMiddleware };

#[allow(deprecated)]
pub fn start( interval_sec: u32, stuff_creator: StuffMiddleware ) {
    thread::spawn( move || {
        let mut stuff = stuff_creator.new_stuff();
        let mut timer = Timer::new().unwrap();
        let period = timer.periodic( Duration::seconds( interval_sec as i64 ) );
        loop {
            // задерживаем на интервал таймера
            period.recv().unwrap();
            // пока вся обработка здесь но возможно придётся делат свои события
            if let Err( err_desc ) = process_events( &mut stuff ) {
                let _ = writeln!( &mut stderr(), "fail by trigger: {}", err_desc );
            }
        }
    });
}

fn process_events( stuff: &mut Stuff ) -> EmptyResult {
    try!( stuff.maybe_start_some_events() );
    try!( stuff.maybe_end_some_events() );
    // стартуем после стопа еще раз, потому-что некоторые события по стопу создают новые
    try!( stuff.maybe_start_some_events() );
    Ok( () )
}
