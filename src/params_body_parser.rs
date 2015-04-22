/// Небольшая middleware которая парсит html параметры
/// Написано кривовато, но из готовых никто бинарные данные не разбирает,
/// когда появится нормальная альтернатива заменю
/// Обновлено: Теперь парсит только бинарные параметры, остальные уходят в json

use std::collections::HashMap;
use std::str;
use std::io::{ Read };
use parse_utils;
use iron::mime::{ self, Mime, TopLevel, SubLevel };
use iron::prelude::*;
use iron::headers::ContentType;

const BODY_LIMIT: u64 = 5 * 1024 * 1024 ;

pub struct BinParam {
    pub filename: Option<String>,
    pub data: Vec<u8>
}

pub type BinParamsHash = HashMap<String, BinParam>;
pub enum BinParamsError {
    TooBig,
    NotMultipartFormData,
    IoError
}

pub type BinaryHashMap = HashMap<String, (usize, usize)>;
pub type StringHashMap = HashMap<String, Vec<String>>;

fn get_multipart_boundary( req: &mut Request ) -> Option<String> {
    req.headers.get::<ContentType>().and_then( |content_type| {
        match &content_type.0 {
            &Mime( TopLevel::Multipart, SubLevel::FormData, ref params ) => {

                let bounary_attr = mime::Attr::Ext( "boundary".to_string() );

                params.iter()
                    .find( |&&(ref attr,_)| *attr == bounary_attr )
                    .and_then( |p| {
                        let &(_, ref val) = p;
                        match val {
                            &mime::Value::Ext( ref value_str ) => Some( value_str.clone() ),
                            _ => None
                        }
                    })
            }

            _ => None
        }
    })
}

fn read_all_binary_parts( body: &[u8], boundary: &[u8], bin_hash: &mut BinParamsHash ) {
    for slice_idx in parse_utils::boundary_idx( body, boundary ) {
        read_binary_part( body, slice_idx, bin_hash );
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

fn read_binary_part( body: &[u8], (from, to) : (usize, usize), bin_hash: &mut BinParamsHash ) {
    let chunk = &body[from .. to];
    //делим их на описательную часть и сами данные
    let (desc, data) = try_opt!( parse_utils::split_seq_alt( chunk, b"\r\n\r\n", b"\n\n" ) );
    let desc_str = str::from_utf8( desc ).unwrap_or( "" );
    // находим имя параметра
    let name = try_opt!( parse_utils::str_between( desc_str, "name=\"", "\"" ) );

    let param_data_slice: &[u8] = &body[to - data.len() .. to - 4];
    let mut param_data: Vec<u8> = Vec::with_capacity( param_data_slice.len() );
    // TODO: переделать когда стабилизируют push_all
    // param_data.push_all( param_data_slice );
    for &x in param_data_slice {
        param_data.push( x );
    }

    let bin_param = BinParam {
        filename: parse_utils::str_between( desc_str, "filename=\"", "\"" )
                               .map( |s| s.to_string() ),
        data: param_data
    };

    bin_hash.insert( name.to_string(), bin_param );
}

pub trait ParamsBody {
    fn parse_bin_params( &mut self ) -> Result<BinParamsHash, BinParamsError>;
}

impl<'a, 'b> ParamsBody for Request<'a, 'b> {
    fn parse_bin_params( &mut self ) -> Result<BinParamsHash, BinParamsError> {
        match get_multipart_boundary( self ) {
            Some( boundary ) => {
                let mut bytes = Vec::new();
                let mut limited_body = self.body.by_ref().take( BODY_LIMIT );
                debug!( "start reading multiformat data" );
                match limited_body.read_to_end( &mut bytes ) {
                    Ok( readed ) => {
                        debug!( "readed = {}", readed );
                        if readed == BODY_LIMIT as usize {
                            debug!( "too BIG!" );
                            return Err( BinParamsError::TooBig );
                        }
                        let mut bin_hash = HashMap::new();
                        read_all_binary_parts( &bytes, boundary.as_bytes(), &mut bin_hash );
                        Ok( bin_hash )
                    },
                    Err(_) => {
                        debug!( "error while reading" );
                        return Err( BinParamsError::IoError )
                    }
                }

            },

            None => Err( BinParamsError::NotMultipartFormData )
        }
    }
}
