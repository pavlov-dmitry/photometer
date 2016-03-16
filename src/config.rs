use std::io;
use std::io::{ Read, Write };
use std::fs::File;
use rustc_serialize::json;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use types::{ CommonResult, CommonError, common_error };
use std::path::Path;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
    server_ip: (u8, u8, u8, u8),
    pub server_port: u16,
    pub root_url: String,
    pub db_name : String,
    pub db_user : String,
    pub db_password : String,
    pub db_min_connections : usize,
    pub db_max_connections : usize,
    pub photo_store_path: String,
    pub photo_store_max_photo_size_bytes : usize,
    pub photo_store_preview_size : usize,
    pub events_trigger_period_sec: u32,
    pub mail_smtp_address: String,
    pub mail_smtp_port: u16,
    pub mail_from_address: String,
    pub mail_from_pass: String,
}

impl Config {
    pub fn server_socket(&self) -> SocketAddr {
        let (fst, snd, thr, fth) = self.server_ip;
        let socket = SocketAddrV4::new( Ipv4Addr::new( fst, snd, thr, fth ),
                                        self.server_port );
        SocketAddr::V4( socket )
    }
}

impl Config {
    fn new() -> Config {
        Config{
            server_ip : (127, 0, 0, 1),
            server_port : 6767,
            root_url: "http://localhost".to_string(),
            db_name : "photometer".to_string(),
            db_user : "photometer".to_string(),
            db_password : "parol".to_string(),
            db_min_connections : 10,
            db_max_connections : 100,
            photo_store_path : "../data/photos/".to_string(),
            photo_store_max_photo_size_bytes : 3145728,
            photo_store_preview_size : 300,
            events_trigger_period_sec : 600,
            mail_smtp_address: "mail.someserver.org".to_string(),
            mail_smtp_port: 25,
            mail_from_address: "photometer@mail.com".to_string(),
            mail_from_pass: "qwerty".to_string(),
        }
    }
}

pub fn default() -> Config {
    Config::new()
}

pub fn load( path: &Path ) -> CommonResult<Config> {
    let mut file = match File::open( path ) {
        Ok( file ) => file,
        Err( e ) => return common_error( format!( "Fail to open config file, description: {}", e ) ),
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok ( _ ) => {
            json::decode::<Config>( &content )
                .map_err( |e| {
                    CommonError( format!( "Config fail to decode, description: {}", e ) )
                })
        }

        Err( e ) => common_error( format!( "Config fail to load, description: {}", e ) )
    }
}

pub fn load_or_default( path: &Path ) -> Config {
    match load( path ) {
        Ok( cfg ) => cfg,
        Err( e ) => {
            let _ = writeln!( &mut io::stderr(), "{}", &e );
            default()
        }
    }
}
