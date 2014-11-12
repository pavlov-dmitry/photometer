use params_body_parser::{ ParamsBody };
use nickel::{ Request };
use std::str;
use super::err_msg;
use std::from_str::{ from_str, FromStr };

pub trait GetParamable {
    fn get_param( &self, prm: &str ) -> Result<&str, String>;
    fn get_param_bin( &self, prm: &str ) -> Result<&[u8], String>;
    fn get_param_uint( &self, prm: &str ) -> Result<uint, String>;
}

//TODO: проверить на следующей версии раста, а пока ICE =(
pub trait GetManyParams {
    fn get_prm<'a, T: FromParams<'a>>( &'a self, prm: &str ) -> Result<T, String>;
    fn get2params<'a, T1: FromParams<'a>, T2: FromParams<'a>>( &'a self, prm1: &str, prm2: &str ) -> Result<(T1, T2), String>;
    fn get3params<'a, T1: FromParams<'a>, T2: FromParams<'a>, T3: FromParams<'a>>( 
        &'a self, prm1: &str, prm2: &str, prm3: &str ) -> Result<(T1, T2, T3), String>;
}

pub trait FromParams<'a> {
    fn from_params( params: &'a GetParamable, prm: &str ) -> Result<Self, String>;
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

impl<'a, Params: GetParamable> GetManyParams for Params {
    fn get_prm<'a, T: FromParams<'a>>( &'a self, prm: &str ) -> Result<T, String> {
        FromParams::from_params( self, prm )
    }
    fn get2params<'a, T1: FromParams<'a>, T2: FromParams<'a>>( &'a self, prm1: &str, prm2: &str ) -> Result<(T1, T2), String> {
        match FromParams::from_params( self, prm1 ) {
            Ok( p1 ) => match FromParams::from_params( self, prm2 ) {
                Ok( p2 ) => Ok( (p1, p2) ),
                Err( e ) => Err( e )
            },
            Err( e ) => Err( e )
        }
    }

    fn get3params<'a, T1: FromParams<'a>, T2: FromParams<'a>, T3: FromParams<'a>>( 
        &'a self, prm1: &str, prm2: &str, prm3: &str ) -> Result<(T1, T2, T3), String> {
        match FromParams::from_params( self, prm1 ) {
            Ok( p1 ) => match FromParams::from_params( self, prm2 ) {
                Ok( p2 ) => match FromParams::from_params( self, prm3 ) {
                    Ok( p3 ) => Ok( ( p1, p2, p3 ) ),
                    Err( e ) => Err( e )
                },
                Err( e ) => Err( e )
            },
            Err( e ) => Err( e )
        }   
    }
}

impl<'a> FromParams<'a> for &'a [u8] {
    fn from_params( params: &'a GetParamable, prm: &str ) -> Result<&'a [u8], String> {
        params.get_param_bin( prm )
    }   
}

impl<'a> FromParams<'a> for &'a str {
    fn from_params( params: &'a GetParamable, prm: &str ) -> Result<&'a str, String> {
        params.get_param( prm )
    }   
}

impl<'a, T: FromStr> FromParams<'a> for T {
    fn from_params( params: &'a GetParamable, prm: &str ) -> Result<T, String> {
        params.get_param( prm )
            .and_then( |s| from_str::<T>( s ).ok_or( err_msg::invalid_type_param( prm ) ) )
    }
}
