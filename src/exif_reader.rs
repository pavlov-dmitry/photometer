extern crate libc;
extern crate num;
use self::libc::{ size_t, c_int };
use self::num::rational::Ratio;
use std::collections::HashMap;
use std::c_str::CString;

pub type ExifEntries = HashMap<String, ExifValue>;

/// вычитывает exif в память в виде таблицы
pub fn from_memory( data: &[u8] ) -> Option<ExifEntries> {
    let exif_data = unsafe {
        exif_data_new_from_data( data.as_ptr(), data.len() as size_t )
    };
    if exif_data != 0 { 
        let byte_order = unsafe{ exif_data_get_byte_order( exif_data ) };
        let mut read_body = ReadBody{ 
            byte_order: byte_order, 
            ifd: 0, 
            entries: HashMap::new() 
        };
        unsafe{ 
            exif_data_foreach_content( exif_data, read_exif_content, &mut read_body );
            exif_data_free( exif_data ); 
        }
        Some( read_body.entries )
    }
    else {
        None
    }
}

///упрощенный доступ к опередленным параметрам exif-a
pub trait ExifValues {
    fn iso(&self) -> Option<u32>;
    fn focal_length(&self) -> Option<u16>;
    fn focal_length_35mm(&self) -> Option<u16>;
    fn aperture(&self) -> Option<f32>;
    fn shutter_speed(&self) -> Option<i32>;
    fn camera_model<'a>(&'a self)->Option<&'a str>;
}

impl ExifValues for ExifEntries {
    fn iso(&self) -> Option<u32> {
        self.get( &"ISOSpeedRatings".to_string() )
            .and_then( |v| v.as_short().map( |v| v.to_u32().unwrap() ) )
    }
    fn focal_length(&self) -> Option<u16> {
        self.get( &"FocalLength".to_string() )
            .and_then( |v| v.as_ratio() )
            .map( |r| r.to_integer() as u16 )
    }
    fn focal_length_35mm(&self) -> Option<u16> {
        self.get( &"FocalLengthIn35mmFilm".to_string() )
            .and_then( |v| v.as_short() )
    }
    fn aperture(&self) -> Option<f32> {
        self.get( &"FNumber".to_string() )
            .and_then( |v| v.as_ratio() )
            .and_then( |r| 
                if *r.denom() != 0 { 
                    Some( r.numer().to_f32().unwrap() / r.denom().to_f32().unwrap() )
                }  
                else { 
                    None 
                } 
            )
    }
    fn shutter_speed(&self) -> Option<i32> {
        self.get( &"ExposureTime".to_string() )
            .and_then( |v| v.as_ratio() )
            .map( |r| 
                if r.is_integer() {
                    r.to_integer() as i32
                }
                else {
                    r.denom().to_i32().unwrap().neg()
                }
            )
    }
    fn camera_model<'a>(&'a self)->Option<&'a str> {
        self.get( &"Model".to_string() )
            .and_then( |v| v.as_text() )
    }
}

#[allow(dead_code)]
#[deriving(Show)]
enum ExifValue {
    ExifByte( u8 ),
    ExifText( String ),
    ExifShort( u16 ),
    ExifLong( u32 ),
    ExifRatio( Ratio<u32> ),
    ExifSByte( i8 ),
    ExifUndefined,
    ExifSShort( i16 ),
    ExifSLong( i32 ),
    ExifSRatio( Ratio<i32> ),
    ExifFloat( f32 ),
    ExifDouble( f64 ),
    ErrorValue
}

impl ExifValue {
	#[allow(dead_code)]
    pub fn as_u8(&self) -> Option<u8>{  
        match self {
            &ExifByte( v ) => Some( v ),
            _ => None
        }
    }

	#[allow(dead_code)]
    pub fn as_text<'a>(&'a self) -> Option<&'a str> {
        match self {
            &ExifText( ref v ) => Some( v.as_slice() ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_short(&self) -> Option<u16> {
        match self {
            &ExifShort( v ) => Some( v ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_long(&self) -> Option<u32> {
        match self {
            &ExifLong( v ) => Some( v ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_ratio(&self) -> Option<Ratio<u32>> {
        match self {
            &ExifRatio( v ) => Some( v ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn is_undefined(&self) -> bool {
        match self {
            &ExifUndefined => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn as_sshort(&self) -> Option<i16> {
        match self {
            &ExifSShort( v ) => Some( v ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_slong(&self) -> Option<i32> {
        match self {
            &ExifSLong( v ) => Some( v ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_sratio(&self) -> Option<Ratio<i32>> {
        match self {
            &ExifSRatio( v ) => Some( v ),
            _ => None
        }
    }

	#[allow(dead_code)]
    pub fn as_float(&self) -> Option<f32> {
        match self {
            &ExifFloat( v ) => Some( v ),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_double(&self) -> Option<f64> {
        match self {
            &ExifDouble( v ) => Some( v ),
            _ => None
        }
    }
}

//как убрать этот варнинг в моих условиях я не понял, потому просто отключаю его
#[allow(improper_ctypes)] 
#[link(name = "exif", kind = "static")]
extern {
    fn exif_data_new_from_data ( data: *const u8, size: size_t) -> c_int;
    fn exif_data_free( ptr: c_int );
    fn exif_data_foreach_content (exif_data_ptr: c_int, func : extern fn(c_int, *mut ReadBody), user_data: *mut ReadBody);
    fn exif_content_foreach_entry( exif_content_ptr: c_int, func: extern fn( *mut ExifEntry, *mut ReadBody ), user_data: *mut ReadBody );
    fn exif_data_get_byte_order(exif_data_ptr: c_int) -> c_int;
    fn exif_tag_get_name_in_ifd(tag: i32, ifd: c_int) -> *const i8;
    fn exif_content_get_ifd(content: c_int) -> c_int;
    fn exif_get_short (b: *const u8, order: c_int) -> u16;
    fn exif_get_sshort (b: *const u8, order: c_int) -> i16;
    fn exif_get_long (b: *const u8, order: c_int) -> u32;
    fn exif_get_slong (b: *const u8, order: c_int) -> i32;
    fn exif_get_rational (b: *const u8, order: c_int) -> ExifRational;
    fn exif_get_srational (b: *const u8, order: c_int) -> ExifSRational;
}

#[repr(C)]
struct ExifRational {
    num: u32,
    den: u32
}

#[repr(C)]
struct ExifSRational {
    num: i32,
    den: i32
}

#[repr(C)]
struct ExifEntry {
    tag: i32,
    format: ExifFormat,
    components: u32,
    data: *const u8,
    size: uint,
    parent: c_int,
    private: c_int
}

//тут этот варнинг отключать смысла нет, но сцуко видать пока глючит компилятор
#[allow(dead_code)]
#[repr(C)]
enum ExifFormat {
    EXIF_FORMAT_BYTE = 1, 
    EXIF_FORMAT_ASCII = 2, 
    EXIF_FORMAT_SHORT = 3,
    EXIF_FORMAT_LONG = 4, 
    EXIF_FORMAT_RATIONAL = 5, 
    EXIF_FORMAT_SBYTE = 6, 
    EXIF_FORMAT_UNDEFINED = 7, 
    EXIF_FORMAT_SSHORT = 8, 
    EXIF_FORMAT_SLONG = 9, 
    EXIF_FORMAT_SRATIONAL = 10, 
    EXIF_FORMAT_FLOAT = 11, 
    EXIF_FORMAT_DOUBLE = 12
}

fn to_exif_value( entry: &ExifEntry, byte_order: c_int ) -> ExifValue {
    if entry.data.is_null() {
        return ErrorValue;
    }
    match entry.format {
        EXIF_FORMAT_BYTE => {
            let data = unsafe{ entry.data.as_ref().unwrap() };
            ExifByte( *data )
        },
        EXIF_FORMAT_ASCII => {
            let name_cstr = unsafe{ CString::new( entry.data as *const i8, false ) };
            match name_cstr.as_str() {
                Some( s ) => ExifText( s.to_string() ),
                None => ExifText( "bad ASCII".to_string() )
            }
        },
        EXIF_FORMAT_SHORT => ExifShort( unsafe{ exif_get_short( entry.data, byte_order ) } ),
        EXIF_FORMAT_LONG => ExifLong( unsafe{ exif_get_long( entry.data, byte_order ) } ),
        EXIF_FORMAT_RATIONAL => {
            let rat = unsafe{ exif_get_rational( entry.data, byte_order ) };
            ExifRatio( Ratio::new_raw( rat.num, rat.den ) )    
        },
        EXIF_FORMAT_SBYTE => {
            let data = unsafe{ entry.data.as_ref().unwrap() };
            ExifSByte( *data as i8 ) 
        },
        EXIF_FORMAT_SSHORT => ExifSShort( unsafe{ exif_get_sshort( entry.data, byte_order ) } ),
        EXIF_FORMAT_SLONG => ExifSLong( unsafe{ exif_get_slong( entry.data, byte_order ) } ),
        EXIF_FORMAT_SRATIONAL => {
            let rat = unsafe{ exif_get_srational( entry.data, byte_order ) };
            ExifSRatio( Ratio::new_raw( rat.num, rat.den ) )   
        },
        _ => ExifUndefined
    }
}

#[repr(C)]
struct ReadBody {
    byte_order: c_int,
    ifd: c_int,
    entries: ExifEntries 
}

extern fn read_exif_entry( e: *mut ExifEntry, b: *mut ReadBody ) {
    let entry = unsafe{ e.as_ref().unwrap() };
    let body = unsafe{ b.as_mut().unwrap() };
    let name_cstr = unsafe{ CString::new( exif_tag_get_name_in_ifd( entry.tag, body.ifd ), false ) };
    match name_cstr.as_str() {
        Some( name_utf8 ) => {
            let value = to_exif_value( entry, body.byte_order );
            body.entries.insert( name_utf8.to_string(), value );
        }
        None => {}
    }
}

extern fn read_exif_content( content: c_int, b: *mut ReadBody ) {
    let body = unsafe{ b.as_mut().unwrap() };
    body.ifd = unsafe{ exif_content_get_ifd( content ) };
    unsafe{ exif_content_foreach_entry( content, read_exif_entry, b ); }
}