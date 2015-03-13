/// Небольшая middleware которая парсит html параметры
/// Написано кривовато, но из готовых никто бинарные данные не разбирает, 
/// когда появится нормальная альтернатива заменю

use std::collections::HashMap;
use std::str;
use std::io::Read;
use parse_utils;
use iron::{ BeforeMiddleware, status, headers };
use iron::mime::{ self, Mime, TopLevel, SubLevel };
use iron::prelude::*;
use iron::typemap::Key;
use url;

#[derive(Clone)]
pub struct ParamsBodyParser;

struct StringHashMapKey;
struct BinaryHashMapKey;
struct BinaryDataKey;

impl Key for StringHashMapKey { type Value = StringHashMap; }
impl Key for BinaryHashMapKey { type Value = BinaryHashMap; }
impl Key for BinaryDataKey { type Value = Vec<u8>; }

impl BeforeMiddleware for ParamsBodyParser {
    fn before( &self, req: &mut Request ) -> IronResult<()> {
        let mut body = Vec::new();
        if let Err( e ) = req.body.read_to_end( &mut body ) {
            return Err( IronError::new( e, status::InternalServerError ) )
        }

        if !body.is_empty() {
            let mut bin_params = HashMap::new();
            let mut params_hash = HashMap::new();

            //match req.headers.content_type {
            if let Some( content_type ) = req.headers.get::<headers::ContentType>() {
                let mime_value : &Mime = &content_type.0; // не уверен что это лучшый способ
                match mime_value {
                    //если пришли бинарные данные
                    &Mime( TopLevel::Multipart, SubLevel::FormData, ref params ) => {
                        //и передан спец ограничитель
                        if let Some( &(_, ref val) ) = params.iter().find( |&&(ref attr,_)| *attr == mime::Attr::Ext( "boundary".to_string() ) ) {
                            if let &mime::Value::Ext( ref boundary ) = val {
                                // то перебираем все части ограниченные спец ограничителем
                                read_all_binary_parts( &body, boundary.as_bytes(), &mut bin_params, &mut params_hash );
                                req.extensions.insert::<BinaryDataKey>( body );     
                            }
                        }
                    }
                    // елси просто в текстовом виде
                    _ => {
                        let body_str = str::from_utf8( &body ).unwrap_or( "" );
                    
                        // то просто парсим их
                        for &( ref key, ref value ) in url::form_urlencoded::parse( body_str.as_bytes() ).iter() {
                            debug!( "param: {} = {}", key, value );
                            // и запихиваем в контейнер текстовых данных
                            let mut need_insert = false;
                            match params_hash.get_mut( key ) {
                                Some( strings ) => strings.push( value.clone() ),
                                None => need_insert = true
                            }
                            if need_insert {
                                let mut strings = Vec::new();
                                strings.push( value.clone() );
                                params_hash.insert( key.clone(), strings );
                            }                        
                        }
                    }
                }
            }

            req.extensions.insert::<StringHashMapKey>( params_hash );
            req.extensions.insert::<BinaryHashMapKey>( bin_params );
        }
        
        Ok( () )
    } 
}

pub type BinaryHashMap = HashMap<String, (usize, usize)>;
pub type StringHashMap = HashMap<String, Vec<String>>;

fn read_all_binary_parts( body: &[u8], boundary: &[u8], bin_hash: &mut BinaryHashMap, str_hash: &mut StringHashMap ) {
    // из-за ограничений наложенных AnyMap-ом приходится вместо использования стандартного типа &[T]
    // использовать кортеж (начало, конец)
    for slice_idx in parse_utils::boundary_idx( body, boundary ) {
        read_binary_part( body, slice_idx, bin_hash, str_hash );
    }
}

macro_rules! try_opt{
    ($expr:expr) => ({
        match $expr {
            Some( val ) => val,
            None => return
        }  
    })
}

fn read_binary_part( body: &[u8], (from, to) : (usize, usize), bin_hash: &mut BinaryHashMap, str_hash: &mut StringHashMap) {
    let chunk = &body[from .. to];
    //делим их на описательную часть и сами данные
    let (desc, data) = try_opt!( parse_utils::split_seq_alt( chunk, b"\r\n\r\n", b"\n\n" ) );
    let desc_str = str::from_utf8( desc ).unwrap_or( "" );
    // находим имя параметра
    let name = try_opt!( parse_utils::str_between( desc_str, "name=\"", "\"" ) );
    let idx_slice = (to - data.len(), to - 4 ); 
    // записываем имя и "координаты" данных
    debug!( "param: {} = {:?}", name, idx_slice );
    bin_hash.insert( name.to_string(), idx_slice ); 
    // запись имени файла бинарных данных с постфиксом "_filename"
    let filename = try_opt!( parse_utils::str_between( desc_str, "filename=\"", "\"" ) );
    let mut values = Vec::new();
    values.push( filename.to_string() );
    let name = name.to_string() + "_filename";
    debug!( "param: {} = {}", name, filename );
    str_hash.insert( name, values );
}

pub trait ParamsBody {
    fn parameter(&self, key: &str ) -> Option<&String>;
    fn bin_parameter<'a>(&self, key: &str) -> Option<&[u8]>;
    fn parameters( &self, key: &str ) -> Option<&Vec<String>>;
}

impl<'a> ParamsBody for Request<'a> {
    fn parameter(&self, key: &str ) -> Option<&String> {
        self.parameters( key ).map( |strings| &strings[ 0 ] )
    }
    fn bin_parameter<'x>(&self, key: &str) -> Option<&[u8]> {
        self.extensions.get::<BinaryHashMapKey>()
            .and_then( |hash| {
                self.extensions.get::<BinaryDataKey>()
                    .and_then( |body| {
                        hash.get( &key.to_string() )
                            .map( |&(from, to)| &body[from .. to] )  
                    })
            })
    }

    fn parameters( &self, key: &str ) -> Option<&Vec<String>> {
        self.extensions.get::<StringHashMapKey>()
            .and_then( |ref hash| hash.get( key ) )
    }
}

pub fn middleware() -> ParamsBodyParser {
    ParamsBodyParser
}