use nickel::{ Response };
use nickel::mimes;
use serialize::{ Encodable, Encoder };
use serialize::json;
use serialize::json::{ ToJson, Json };
use std::collections::TreeMap;

use types::{ PhotoInfo, ImageType };

#[deriving(Encodable)]
struct Record {
    field: String,
    value: String
}

impl ToJson for Record {
    fn to_json(&self) -> Json {
        let mut d = TreeMap::new();
        d.insert("field".to_string(), self.field.to_json());
        d.insert("value".to_string(), self.value.to_json());
        json::Object(d)
    }
}

impl ToJson for ImageType {
    fn to_json(&self) -> Json {
        json::String( self.to_string() )
    }
}

impl ToJson for PhotoInfo {
    fn to_json(&self) -> Json {
        let mut d = TreeMap::new();
        d.insert( String::from_str( "id" ), self.id.to_json() );
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
        json::Object( d )
    }
}

#[deriving(Encodable)]
struct Error {
    field: String,
    reason: String
}

type Records = Vec<Json>;

#[deriving(Encodable)]
pub struct Answer {
    records_exists: bool,
    records: Records,
    errors_exists: bool,
    errors: Vec<Error>
}

pub type AnswerResult = Result<Answer, String>;

impl Answer {
    pub fn add_error( &mut self, field: &str, reason: &str ) {
        self.errors_exists = true;
        self.errors.push( Error{ field: field.to_string(), reason: reason.to_string() } );
    }
    pub fn add_record( &mut self, field: &str, value: &str ) {
        self.records_exists = true;
        self.records.push( Record{ field: field.to_string(), value: value.to_string() }.to_json() );
    }

    pub fn add_photo_info(&mut self, info: &PhotoInfo ) {
        self.records.push( info.to_json() );
    }
}

pub trait AnswerSendable {
    fn send_answer( &mut self, answer: &AnswerResult );
}

impl<'a, 'b> AnswerSendable for Response<'a, 'b> {
    fn send_answer( &mut self, answer: &AnswerResult ) {
        match answer {
            &Err( ref err_desc ) => self.send( err_desc.as_slice() ),
            &Ok( ref answer ) => {
                self.content_type( mimes::Json );
                self.send( json::encode( answer ).as_slice() );
            }
        }
    }
}

pub fn new () -> Answer {
    Answer{  
        records_exists: false,
        records: Vec::new(),
        errors_exists: false,
        errors: Vec::new()
    }
}