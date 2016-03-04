use super::{
    Event,
    EventId,
    ScheduledEventInfo,
    FullEventInfo,
    Description,
};
use super::group_voting::{ self, ChangeByVoting };
use types::{
    Id,
    ShortInfo,
    EmptyResult,
    CommonResult,
    CommonError
};
use stuff::{ Stuff };
use time;
use rustc_serialize::json;
use rustc_serialize::{ Encodable, Encoder };
use database::Databaseable;
use db::users::DbUsers;
use db::groups::DbGroups;
use mail_writer::MailWriter;
use mailer::Mailer;

#[derive(Clone)]
pub struct JoinToGroup;
pub const ID: EventId = EventId::JoinToGroup;

impl JoinToGroup {
    pub fn new() -> JoinToGroup {
        JoinToGroup
    }

    pub fn create( creator: Id, group_id: Id, user: ShortInfo, text: &str ) -> FullEventInfo {
        let start = time::get_time();
        let end = start + time::Duration::days( 7 );
        let data = Data {
            user_id: user.id,
            text: text.to_owned()
        };
        let data = json::encode( &data ).unwrap();

        let info = FullEventInfo {
            id: ID,
            name: format!( "Присоединение к группе пользователя '{}'", user.name ),
            start_time: start,
            end_time: end,
            data: data,
            group: Some( group_id ),
            creator: Some( creator )
        };
        group_voting::new( group_id, 1., &info )
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
struct Data {
    user_id: Id,
    text: String
}

#[derive(Debug, RustcEncodable)]
struct JoinToGroupInfo {
    user: ShortInfo,
    text: String
}

impl ChangeByVoting for JoinToGroup {
    fn info( &self, stuff: &mut Stuff, body: &ScheduledEventInfo ) -> CommonResult<Description> {
        let data = try!( get_data( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let user_info = try!( db.user_by_id( data.user_id ) ).unwrap();
        let info = JoinToGroupInfo {
            user: ShortInfo {
                id: user_info.id,
                name: user_info.name
            },
            text: data.text
        };
        Ok( Description::new( info ) )
    }

    // действие на старт голосования
    fn start( &self, stuff: &mut Stuff, group: &ShortInfo, body: &ScheduledEventInfo ) -> EmptyResult{
        let data = try!( get_data( body ) );
        let (subject, mail) = stuff.write_time_for_group_join_voit_mail( &group.name,
                                                                         body.scheduled_id );
        let user_info = {
            let db = try!( stuff.get_current_db_conn() );
            try!( db.user_by_id( data.user_id ) ).unwrap()
        };
        try!( stuff.send_mail( &user_info, &subject, &mail ) );
        Ok( () )
    }
    /// применить елси согласны
    fn apply( &self, stuff: &mut Stuff, group: &ShortInfo, body: &ScheduledEventInfo ) -> EmptyResult {
        let data = try!( get_data( body ) );
        let db = try!( stuff.get_current_db_conn() );
        try!( db.add_members( group.id, &[ data.user_id ] ) );
        Ok( () )
    }
    /// краткое имя события, будет в основном использоваться в рассылке
    fn name( &self, stuff: &mut Stuff, group: &ShortInfo, body: &ScheduledEventInfo ) -> CommonResult<String> {
        let data = try!( get_data( body ) );
        let db = try!( stuff.get_current_db_conn() );
        let user_info = try!( db.user_by_id( data.user_id ) ).unwrap();
        let name = format!( "Присоединение '{}' к группе '{}'", user_info.name, group.name );
        Ok( name )
    }
}

fn get_data( body: &ScheduledEventInfo ) -> CommonResult<Data> {
    json::decode( &body.data )
        .map_err( |e| CommonError( format!( "JoinToGroup event data decode error: {}", e ) ) )
}
