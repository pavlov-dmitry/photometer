use params_body_parser::{ ParamsBody };
use nickel::{ Request };
use std::str;
use super::err_msg;
use std::from_str::{ from_str, FromStr };

pub trait GetParamable {
    fn get_param( &self, prm: &str ) -> Result<&str, String>;
    fn get_param_bin( &self, prm: &str ) -> Result<&[u8], String>;
    fn get_param_uint( &self, prm: &str ) -> Result<uint, String>;
    fn get2params<T1: FromStr, T2: FromStr>( &self, prm1: &str, prm2: &str ) -> Result<(T1, T2), String>;
}

impl<'a, 'b> GetParamable for Request<'a, 'b> {
    //инкапсулирует поиск параметра сначало в текстовом виде, потом в бинарном
    fn get_param( &self, prm: &str ) -> Result<&str, String> {
        match self.parameter( prm ) {
            Some( s ) => Ok( s.as_slice() ),
            None => match self.bin_parameter( prm ) {
                Some( b ) => match str::from_utf8( b ) {
                    Some( s ) => Ok( s ),
                    None => Err( err_msg::not_a_string_param( prm ) )
                },
                None => Err( err_msg::param_not_found( prm ) )
            }
        }
    }
    fn get_param_bin( &self, prm: &str ) -> Result<&[u8], String> {
        self.bin_parameter( prm ).ok_or( err_msg::invalid_type_param( prm ) )
    }
    fn get_param_uint( &self, prm: &str ) -> Result<uint, String> {
        self.get_param( prm )
            .and_then( |s| from_str::<uint>( s ).ok_or( err_msg::invalid_type_param( prm ) ) )
    }
    fn get2params<T1: FromStr, T2: FromStr>( &self, prm1: &str, prm2: &str ) -> Result<(T1, T2), String> {
        self.get_param( prm1 )
            .and_then( |str1| from_str::<T1>( str1 ).ok_or( err_msg::invalid_type_param( prm1 ) ) )
            .and_then( |p1| {
                let val = self.get_param( prm2 )
                    .and_then( |str2| from_str::<T2>( str2 ).ok_or( err_msg::invalid_type_param( prm2 ) ) );
                match val {
                    Ok( p2 ) => Ok( (p1, p2) ),
                    Err( e ) => Err( e )
                }
            })
    }
}
