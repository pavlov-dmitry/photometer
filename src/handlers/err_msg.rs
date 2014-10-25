// формирование стандартных описаний ошибок
use std::io::{ IoError };

#[inline]
pub fn param_not_found( prm: &str ) -> String {
    format!( "can not find parameter '{}'", prm )
}

#[inline]
pub fn invalid_type_param( prm: &str ) -> String {
    format!( "invalid type of parameter '{}'", prm )
}

#[inline]
pub fn not_a_string_param( prm: &str ) -> String {
    format!( "not a string param '{}'", prm )
}


pub fn fs_error( e: IoError ) -> String {
    format!( "filesystem error: {}", e )
}