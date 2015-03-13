// формирование стандартных описаний ошибок
use std::io;
use std::fmt::Display;
use std::old_io;

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
pub fn fs_error( e: io::Error ) -> String {
    format!( "filesystem error: {}", e )
}

#[inline]
pub fn old_fs_error( e: old_io::IoError ) -> String {
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
pub fn fn_failed<E: Display>( fn_name: &str, e: E ) -> String {
	format!( "error {} failed: {}", fn_name, e )
}

/*pub fn fn_failed<E: Display>( fn_name: &str, e: E ) -> String {
	format!( "error {} failed: {}", fn_name, e )
}*/