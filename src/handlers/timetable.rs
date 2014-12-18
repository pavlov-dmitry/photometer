use nickel::{ Request, Response, NickelError, NickelErrorKind, Halt, MiddlewareResult };
use authentication::{ Userable };
use answer::{ AnswerSendable, AnswerResult, Answer };
use db::timetable::{ DbTimetable, TimetableEventInfo };
use db::groups::DbGroups;
use database::Databaseable;
use get_param::GetParamable;
use time;
use types::{ Id };
use http::status::Status;

pub fn timetable_path() -> &'static str {
    "/timetable/:group_id"
}

pub fn set_timetable( req: &Request, res: &mut Response ) -> MiddlewareResult {
    let group_id = try!( get_group_id( req ) );
    res.send_answer( &set_timetable_answer( group_id, req ) );
    Ok( Halt )
}

fn set_timetable_answer( group_id: Id, req: &Request ) -> AnswerResult {
    let mut db = try!( req.get_db_conn() );
    let mut answer = Answer::new();

    if try!( db.is_group_exists( group_id ) ) {
        let start_time_str = try!( req.get_param( "start_time" ) );
        let start_time = try!( time::strptime( start_time_str, "%Y.%m.%d %k:%M:%S" )
            .map_err( |e| format!( "error parsing `start_time`: {}", e ) ) 
        );
        let end_time_str = try!( req.get_param( "end_time" ) );
        let end_time = try!( time::strptime( end_time_str, "%Y.%m.%d %k:%M:%S" )
            .map_err( |e| format!( "error parsing `end_time`: {}", e ) ) 
        );

        let timetable_event = TimetableEventInfo {
            group_id: group_id,
            event_id: try!( req.get_param_i64( "event_id" ) ),
            event_name: try!( req.get_param( "event_name" ) ).to_string(),
            start_time: start_time.to_timespec(),
            end_time: end_time.to_timespec(),
            params: try!( req.get_param( "params" ) ).to_string(),
        };
        let mut events = Vec::new();
        events.push( timetable_event );

        try!( db.add_new_timetable_version( group_id, &events ) );
        
        answer.add_record( "new version of timetable", &String::from_str( "found" ) );
    }
    else {
        answer.add_error( "group", "not_found" );
    }
    Ok( answer )
}

fn get_group_id( req: &Request ) -> Result<Id, NickelError> {
    let group_id_str = req.param( "group_id" );
    ::std::str::from_str::<Id>( group_id_str )
        .ok_or( NickelError::new("Error parsing request path", NickelErrorKind::ErrorWithStatusCode(Status::NotFound)) )
}