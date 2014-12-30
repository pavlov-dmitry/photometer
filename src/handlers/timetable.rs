use nickel::{ Request, Response, NickelError, NickelErrorKind, Halt, MiddlewareResult };
use answer::{ AnswerSendable, AnswerResult, Answer };
use db::timetable::{ DbTimetable, TimetableEventInfo };
use db::groups::DbGroups;
use database::Databaseable;
use get_param::GetParamable;
use types::{ Id };
use http::status::Status;

pub fn timetable_path() -> &'static str {
    "/timetable/:group_id"
}

pub fn set_timetable( req: &mut Request, res: &mut Response ) -> MiddlewareResult {
    let group_id = try!( get_group_id( req ) );
    res.send_answer( &set_timetable_answer( group_id, req ) );
    Ok( Halt )
}

fn set_timetable_answer( group_id: Id, req: &mut Request ) -> AnswerResult {
    let mut answer = Answer::new();
    let is_group_exists = {
        let mut db = try!( req.get_current_db_conn() );
        try!( db.is_group_id_exists( group_id ) )
    };
    if is_group_exists {
        let start_time = try!( req.get_param_time( "start_time" ) );
        let end_time = try!( req.get_param_time( "end_time" ) );

        let timetable_event = TimetableEventInfo {
            group_id: group_id,
            event_id: try!( req.get_param_id( "event_id" ) ),
            event_name: try!( req.get_param( "event_name" ) ).to_string(),
            start_time: start_time,
            end_time: end_time,
            params: try!( req.get_param( "params" ) ).to_string(),
        };
        let mut events = Vec::new();
        events.push( timetable_event );

        {
            let mut db = try!( req.get_current_db_conn() );
            let new_version = try!( db.add_new_timetable_version( group_id, &events ) );
            try!( db.set_timetable_version( group_id, new_version ) );
        }
        
        answer.add_record( "new version of timetable", &String::from_str( "setted" ) );
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