use nickel::{ Response };
use serialize::json;

#[deriving(Encodable)]
struct Record {
    field: String,
    value: String
}

#[deriving(Encodable)]
struct Error {
	field: String,
	reason: String
}

#[deriving(Encodable)]
pub struct Answer {
	records_exists: bool,
	records: Vec<Record>,
	errors_exists: bool,
	errors: Vec<Error>
}

impl Answer {
	pub fn add_error( &mut self, field: &str, reason: &str ) {
		self.errors_exists = true;
		self.errors.push( Error{ field: field.to_string(), reason: reason.to_string() } );
	}
	pub fn add_record( &mut self, field: &str, value: &str ) {
		self.records_exists = true;
		self.records.push( Record{ field: field.to_string(), value: value.to_string() } );
	}
}

pub trait AnswerSendable {
	fn send_answer( &mut self, answer: &Result<Answer, String> );
}

impl<'a, 'b> AnswerSendable for Response<'a, 'b> {
	fn send_answer( &mut self, answer: &Result<Answer, String> ) {
		match answer {
	        &Err( ref err_desc ) => self.send( err_desc.as_slice() ),
	        &Ok( ref answer ) => {
	        	self.content_type( "application/json" );
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