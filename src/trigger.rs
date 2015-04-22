/// поток событий по таймеру
use std::thread;
use std::io::stderr;
use std::io::Write;
use types::EmptyResult;
use events::events_manager::EventsManagerStuff;

use stuff::{ Stuff, StuffMiddleware };

#[allow(deprecated)]
pub fn start( interval_sec: u32, stuff_creator: StuffMiddleware ) {
    thread::spawn( move || {
        let mut stuff = stuff_creator.new_stuff();
        loop {
            // задерживаем на интервал таймера
            thread::sleep_ms( interval_sec * 1000 );

            // NOTE: пока вся обработка здесь но возможно придётся делать свои события
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
