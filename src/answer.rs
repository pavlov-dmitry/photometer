use nickel::{ Response };
use nickel::mimes::{ MediaType };
use serialize::{ Encodable, Encoder };
use serialize::json;
use serialize::json::{ ToJson, Json };
use std::collections::TreeMap;

use types::{ PhotoInfo, ImageType, CommonResult, MailInfo };

pub trait AnswerSendable {
    fn send_answer( &mut self, answer: &AnswerResult );
}

#[deriving(Encodable)]
pub struct Answer {
    records_exists: bool,
    records: Records,
    errors_exists: bool,
    errors: Vec<Error>
}

pub type AnswerResult = CommonResult<Answer>;

impl Answer {
    pub fn new() -> Answer {
        Answer{  
            records_exists: false,
            records: Vec::new(),
            errors_exists: false,
            errors: Vec::new()
        }
    }

    pub fn add_error( &mut self, field: &str, reason: &str ) {
        self.errors_exists = true;
        self.errors.push( Error{ field: field.to_string(), reason: reason.to_string() } );
    }
    pub fn add_record( &mut self, field: &str, value: &ToJson ) {
        self.records_exists = true;
        self.records.push( Record{ field: field.to_string(), value: value.to_json() }.to_json() );
    }

    pub fn add_to_records(&mut self, value: &ToJson ) {
        self.records_exists = true;
        self.records.push( value.to_json() );
    }
}

impl<'a, 'b> AnswerSendable for Response<'a, 'b> {
    fn send_answer( &mut self, answer: &AnswerResult ) {
        match answer {
            &Err( ref err_desc ) => self.send( err_desc.as_slice() ),
            &Ok( ref answer ) => {
                self.content_type( MediaType::Json );
                self.send( json::encode( answer ).as_slice() );
            }
        }
    }
}


#[deriving(Encodable)]
struct Record {
    field: String,
    value: Json
}

#[deriving(Encodable)]
struct Error {
    field: String,
    reason: String
}

type Records = Vec<Json>;

impl ToJson for Record {
    fn to_json(&self) -> Json {
        let mut d = TreeMap::new();
        d.insert( self.field.clone(), self.value.to_json() );
        Json::Object(d)
    }
}

impl ToJson for ImageType {
    fn to_json(&self) -> Json {
        Json::String( self.to_string() )
    }
}

trait AddToJson {
    fn add( &mut self, name: &str, value: &ToJson );
}

impl AddToJson for json::Object {
    fn add( &mut self, name: &str, value: &ToJson ) {
        self.insert( String::from_str( name ), value.to_json() );
    }
}

impl ToJson for PhotoInfo {
    fn to_json(&self) -> Json {
        let mut d = TreeMap::new();
        d.add( "id", &self.id );
        d.add( "type", &self.image_type );
        d.add( "width", &self.width );
        d.add( "height", &self.height );
        d.add( "name", &self.name );
        d.add( "iso", &self.iso );
        d.add( "shutter", &self.shutter_speed );
        d.add( "aperture", &self.aperture );
        d.add( "focal_length", &self.focal_length );
        d.add( "focal_length_35mm", &self.focal_length_35mm );
        d.add( "camera_model", &self.camera_model );
        /*d.insert( String::from_str( "id" ), self.id.to_json() );
        d.insert( String::from_str( "type" ), self.image_type.to_json() );
        d.insert( String::from_str( "width" ), self.width.to_json() );
        d.insert( String::from_str( "height" ), self.height.to_json() );
        d.insert( String::from_str( "name" ), self.name.to_json() );
        d.insert( String::from_str( "iso" ), self.iso.to_json() );
        d.insert( String::from_str( "shutter" ), self.shutter_speed.to_json() );
        d.insert( String::from_str( "aperture" ), self.aperture.to_json() );
        d.insert( String::from_str( "focal_length" ), self.focal_length.to_json() );
        d.insert( String::from_str( "focal_length_35mm" ), self.focal_length_35mm.to_json() );
        d.insert( String::from_str( "camera_model" ), self.camera_model.to_json() );
        */
        Json::Object( d )
    }
}


impl ToJson for MailInfo {
    fn to_json( &self ) -> Json {
        let mut d = TreeMap::new();
        d.add( "id", &self.id );
        d.add( "time", &self.creation_time.sec );
        d.add( "sender", &self.sender_name );
        d.add( "subject", &self.subject );
        d.add( "body", &self.body );
        d.add( "readed", &self.readed );
        Json::Object( d )
    }
}