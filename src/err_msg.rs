// формирование стандартных описаний ошибок
use std::io;
use std::fmt::Display;
use types::CommonError;
use image::ImageError;

#[inline]
pub fn param_not_found( prm: &str ) -> CommonError {
    CommonError( format!( "can not find parameter '{}'", prm ) )
}


#[inline]
pub fn invalid_type_param( prm: &str ) -> CommonError {
    CommonError( format!( "invalid type of parameter '{}'", prm ) )
}

/* из-за перехода на json эти функции пока не нужны
#[inline]
pub fn not_a_string_param( prm: &str ) -> String {
    format!( "not a string param '{}'", prm )
}
*/

#[inline]
pub fn fs_error( e: io::Error ) -> CommonError {
    CommonError( format!( "filesystem error: {}", e ) )
}

impl From<ImageError> for CommonError {
    fn from(err: ImageError) -> CommonError {
        CommonError( format!( "error while working with image: {}", err ) )
    }
}

// #[inline]
// pub fn invalid_path_param( prm: &str ) -> CommonError {
//     CommonError( format!( "invalid path param type '{}'", prm ) )
// }

// #[inline]
// pub fn parsing_error_param( prm: &str ) -> CommonError {
//     CommonError( format!( "error parsing param '{}'", prm ) )
// }

#[inline]
pub fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "error {} failed: {}", fn_name, e ) )
}

/*pub fn fn_failed<E: Display>( fn_name: &str, e: E ) -> String {
	format!( "error {} failed: {}", fn_name, e )
}*/
