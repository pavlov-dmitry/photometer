// формирование стандартных описаний ошибок
use std::io::{ IoError };
use std::fmt::Show;

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

#[inline]
pub fn fs_error( e: IoError ) -> String {
    format!( "filesystem error: {}", e )
}

#[inline]
pub fn invalid_path_param( prm: &str ) -> String {
    format!( "invalid path param type '{}'", prm )
}

#[inline]
pub fn parsing_error_param( prm: &str ) -> String {
	format!( "error parsing param '{}'", prm )
}

#[inline]
pub fn fn_failed<E: Show>( fn_name: &str, e: E ) -> String {
	format!( "error {} failed: {}", fn_name, e )
}