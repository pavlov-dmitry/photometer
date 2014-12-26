///  Небольшая middleware которая парсит html параметры, в никеле елси параметра нет, то мы сразу падаем :(
extern crate url;

use std::collections::HashMap;
use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use std::str;
use parse_utils;
use typemap::Assoc;
use plugin::Extensible;

// в nickel-e есть возможность сделать эти парсеры ленивыми, но тогда мне не нравится что из-за кеширования
// нужно менять сигнатуру обработчиков на &mut Request, что лично мне не нравится

#[deriving(Clone)]
pub struct ParamsBodyParser;

struct StringHashMapKey;
struct BinaryHashMapKey;

impl Assoc<StringHashMap> for StringHashMapKey {}
impl Assoc<BinaryHashMap> for BinaryHashMapKey {}

impl Middleware for ParamsBodyParser {
    fn invoke<'a>(&self, req: &'a mut Request, _res: &mut Response) -> MiddlewareResult {
        if !req.origin.body.is_empty() {
            let mut bin_params = HashMap::new();
            let mut params_hash = HashMap::new();

            match req.origin.headers.content_type {
                //если пришли бинарные данные
                Some( ref media_type ) if media_type.type_.as_slice() == "multipart" && 
                                          media_type.subtype.as_slice() == "form-data" => {
                    let params = &media_type.parameters;
                    //и передан спец ограничитель
                    params.iter().filter( |&&(ref k,_)| k.as_slice() == "boundary" ).next() 
                        .map( |&(_, ref boundary)| {
                            // то перебираем все части ограниченные спец ограничителем
                            read_all_binary_parts( req.origin.body.as_slice(), boundary.as_bytes(), &mut bin_params, &mut params_hash );
                        });
                },
                // елси просто в текстовом виде
                _ => {
                    let body_str = str::from_utf8( req.origin.body.as_slice() ).unwrap_or( "" );
                    
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

            req.extensions_mut().insert::<StringHashMapKey, StringHashMap>( params_hash );
            req.extensions_mut().insert::<BinaryHashMapKey, BinaryHashMap>( bin_params );
        }
        
        Ok( Continue )
    } 
}

pub type BinaryHashMap = HashMap<String, (uint, uint)>;
pub type StringHashMap = HashMap<String, Vec<String>>;

fn read_all_binary_parts( body: &[u8], boundary: &[u8], bin_hash: &mut BinaryHashMap, str_hash: &mut StringHashMap ) {
    // из-за ограничений наложенных AnyMap-ом приходится вместо использования стандартного типа &[T]
    // использовать кортеж (начало, конец)
    for slice_idx in parse_utils::boundary_idx( body, boundary ) {
        read_binary_part( body, slice_idx, bin_hash, str_hash );
    }
}

macro_rules! try_opt(
    ($expr:expr) => ({
        match $expr {
            Some( val ) => val,
            None => return
        }  
    })
)

fn read_binary_part( body: &[u8], (from, to) : (uint, uint), bin_hash: &mut BinaryHashMap, str_hash: &mut StringHashMap) {
    let chunk = body.slice( from, to );
    //делим их на описательную часть и сами данные
    let (desc, data) = try_opt!( parse_utils::split_seq_alt( chunk, b"\r\n\r\n", b"\n\n" ) );
    let desc_str = str::from_utf8( desc ).unwrap_or( "" );
    // находим имя параметра
    let name = try_opt!( parse_utils::str_between( desc_str, "name=\"", "\"" ) );
    let idx_slice = (to - data.len(), to - 4 ); 
    // записываем имя и "координаты" данных
    debug!( "param: {} = {}", name, idx_slice );
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

impl<'a, 'b> ParamsBody for Request<'a, 'b> {
    fn parameter(&self, key: &str ) -> Option<&String> {
        self.parameters( key ).map( |strings| &strings[ 0 ] )
    }
    fn bin_parameter<'x>(&self, key: &str) -> Option<&[u8]> {
        self.extensions().get::<BinaryHashMapKey, BinaryHashMap>()
            .and_then( |ref hash| {
                hash.get( &key.to_string() )
                    .map( |&(from, to)| self.origin.body.slice( from, to ) )
            })
    }

    fn parameters( &self, key: &str ) -> Option<&Vec<String>> {
        self.extensions().get::<StringHashMapKey, StringHashMap>()
            .and_then( |ref hash| hash.get( key ) )
    }
}

pub fn middleware() -> ParamsBodyParser {
    ParamsBodyParser
}