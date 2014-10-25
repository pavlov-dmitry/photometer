use params_body_parser::{ ParamsBody };
use nickel::{ Request };
use std::str;
use super::err_msg;
use std::from_str::{ from_str };

pub trait GetParamable {
    fn get_param( &self, prm: &str ) -> Result<&str, String>;
    fn get_param_bin( &self, prm: &str ) -> Result<&[u8], String>;
    fn get_param_uint( &self, prm: &str ) -> Result<uint, String>;
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
}
