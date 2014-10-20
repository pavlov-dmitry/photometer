use std::io::{ File, stdio };
use serialize::json;
use std::io::net::ip::{ Ipv4Addr, IpAddr };

#[deriving(Encodable, Decodable)]
pub struct Config {
    server_ip: (u8, u8, u8, u8),
    pub server_port: u16,
    pub static_files_path : String,
    pub login_page_path : String,
    pub db_name : String,
    pub db_user : String,
    pub db_password : String,
    pub db_min_connections : uint,
    pub db_max_connections : uint
}

impl Config {
    pub fn server_ip(&self) -> IpAddr {
        let (fst, snd, thr, fth) = self.server_ip;
        Ipv4Addr( fst, snd, thr, fth )
    }
}

impl Config {
    fn new() -> Config {
        Config{ 
            server_ip : (127, 0, 0, 1),
            server_port : 6767,
            login_page_path : "../www/login.html".to_string(),
            static_files_path : "../www/".to_string(),
            db_name : "photometer".to_string(),
            db_user : "photometer".to_string(),
            db_password : "parol".to_string(),
            db_min_connections : 10,
            db_max_connections : 100
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