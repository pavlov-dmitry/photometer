use std::io::{ File, stdio };
use serialize::json;

#[deriving(Encodable, Decodable)]
pub struct Config {
	pub static_files_path : String,
	pub login_page_path : String,
	pub db_name : String,
	pub db_user_name : String,
	pub db_user_password : String
}

impl Config {
	fn new() -> Config {
		Config{ 
			login_page_path : "../www/login.html".to_string(),
			static_files_path : "../www/".to_string(),
			db_name : "photometer".to_string(),
			db_user_name : "photometer".to_string(),
			db_user_password : "parol".to_string()
		}
	}
}

pub fn default() -> Config {
	Config::new()
}

pub fn load( path: &Path ) -> Result<Config, String> {
	match File::open( path ).read_to_string() {
		Err( e ) => Err( format!( "Config fail to load, description: {}", e ) ),
		Ok ( content ) => {
			json::decode::<Config>( content.as_slice() )
				.map_err( | e | {
					format!( "Config fail to decode, description: {}", e )
				})
		}
	}
}

pub fn load_or_default( path: &Path ) -> Config {
	match load( path ) {
		Ok( cfg ) => cfg,
		Err( e ) => {
			stdio::stderr().write_line( e.as_slice() ).ok().expect( "can`t write to stderr!" );
			default()
		}
	}
}