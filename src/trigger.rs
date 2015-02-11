/// поток событий по таймеру
use std::thread::Thread;
use std::old_io::timer::sleep;
use std::time::duration::Duration;
use std::old_io::stdio::stderr;
use types::EmptyResult;
use events::events_manager::EventsManagerStuff;

use stuff::{ Stuff, StuffMiddleware };

pub fn start( interval_sec: u32, stuff_creator: StuffMiddleware ) {
    Thread::spawn( move || {
        let mut stuff = stuff_creator.new_stuff();
        loop { 
            // пока вся обработка здесь но возможно придётся делат свои события
            if let Err( err_desc ) = process_events( &mut stuff ) {
                let _ = writeln!( &mut stderr(), "fail by trigger: {}", err_desc );
            }

            sleep( Duration::seconds( interval_sec as i64 ) )
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