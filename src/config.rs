extern crate serialize;

#[deriving(Encodable, Decodable)]
pub struct Config {
	login_page_path : Path
}

impl Config {
	fn new() -> Config {
		Config{ login_page_path : Path::new( "../www/login.html" ) }
	}
}

pub fn default() -> Config {
	Config::new()
}