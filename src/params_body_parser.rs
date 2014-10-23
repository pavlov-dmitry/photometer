///  Небольшая middleware которая парсит html параметры, в никеле елси параметра нет, то мы сразу падаем :(
 
extern crate nickel;
extern crate url;

use std::collections::HashMap;
use self::nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use std::str;
use parse_utils;

#[deriving(Clone)]
pub struct ParamsBodyParser;

impl Middleware for ParamsBodyParser {
    fn invoke<'a>(&self, req: &'a mut Request, _res: &mut Response) -> MiddlewareResult {

        //println!( "______________________________" );
        //println!( "url={}", req.origin.request_uri );
        //println!( "method={}", req.origin.method );

        if !req.origin.body.is_empty() {
            match req.origin.headers.content_type {
                //если пришли бинарные данные
                Some( ref media_type ) if media_type.type_.as_slice() == "multipart" && 
                                          media_type.subtype.as_slice() == "form-data" => {
                    let params = &media_type.parameters;
                    //и передан спец ограничитель
                    params.iter().filter( |&&(ref k,_)| k.as_slice() == "boundary" ).next() 
                        .map( |&(_, ref boundary)| {
                            // то перебираем все части ограниченные спец ограничителем
                            let mut bin_params = HashMap::new();
                            // из-за ограничений наложенных AnyMap-ом приходится вместо использования стандартного типа &[T]
                            // использовать кортеж (начало, конец)
                            for (from, to) in parse_utils::boundary_idx( req.origin.body.as_slice(), boundary.as_bytes() ) {
                                let chunk = req.origin.body.slice( from, to );
                                //делим их на описательную часть и сами данные
                                parse_utils::split_seq_alt( chunk, b"\r\n\r\n", b"\n\n" )
                                    .map( |(desc, data)| {
                                        let desc_str = str::from_utf8( desc ).unwrap_or( "" );
                                        // находим имя параметра
                                        parse_utils::str_between( desc_str, "name=\"", "\"" )
                                            .map( |name| {
                                                //TODO: проверить что 4 байта хвоста будут и под windows
                                                let idx_slice = (to - data.len(), to - 4 ); 
                                                // записываем имя и "координаты" данных
                                                bin_params.insert( name.to_string(), idx_slice ); 
                                            })
                                    });
                            }
                            req.map.insert( bin_params );
                        });
                },
                // елси просто в текстовом виде
                _ => {
                    let body_str = str::from_utf8( req.origin.body.as_slice() ).unwrap_or( "" );
                    let mut params_hash = HashMap::new();
                    // то просто парсим их
                    for &( ref key, ref value ) in url::form_urlencoded::parse_str( body_str ).iter() {
                        // и запихиваем в контейнер текстовых данных
                        params_hash.insert( key.clone(), value.clone() );
                    }
                    req.map.insert( params_hash );        
                }
            }

            
        }
        Ok( Continue )
    } 
}

pub trait ParamsBody {
    fn parameter_string(&self, &String) -> Option<&String>;
    fn parameter(&self, &str ) -> Option<&String>;
    fn bin_parameter<'a>(&self, &str) -> Option<&[u8]>;
    fn bin_parameter_str<'a>(&self, &str) -> Option<&str>;
}

impl<'a, 'b> ParamsBody for Request<'a, 'b> {
    fn parameter_string(&self, key: &String) -> Option<&String> {
        self.map.find::<HashMap<String, String>>()
            .and_then( |ref hash| {
                hash.find( key )
            })
    }
    fn parameter(&self, key: &str ) -> Option<&String> {
        self.parameter_string( &key.to_string() )
    }
    fn bin_parameter<'a>(&self, key: &str) -> Option<&[u8]> {
        self.map.find::<HashMap<String, (uint, uint)>>()
            .and_then( |ref hash| {
                hash.find( &key.to_string() )
                    .map( |&(from, to)| self.origin.body.slice( from, to ) )
            })
    }
    fn bin_parameter_str<'a>(&self, key: &str) -> Option<&str> {
        self.bin_parameter( key ).and_then( |data| str::from_utf8( data ) )
    }
}

pub fn middleware() -> ParamsBodyParser {
    ParamsBodyParser
}