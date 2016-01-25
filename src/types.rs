use std::fmt;
use std::fmt::{ Display, Formatter };
use rustc_serialize::{ Encodable, Encoder };

pub type Id = u64;
// NOTE: Обернул строку в свой тип для реализации преобразований к
// нему через trait From
#[derive(Debug)]
pub struct CommonError( pub String );
pub type CommonResult<T> = Result<T, CommonError>;
pub type EmptyResult = CommonResult<()>;

impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let &CommonError( ref e ) = self;
        write!(f, "{}", e )
    }
}

// NOTE: функция утилита для минимизаций обёртовавыний в коде
#[inline]
pub fn common_error<T>( s: String ) -> CommonResult<T> {
    Err( CommonError( s ) )
}

#[derive(Debug, RustcEncodable)]
pub struct PhotoInfo {
    pub id: Id,
    pub upload_time: u64,
    pub image_type: ImageType,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub iso: Option<u32>,
    pub shutter_speed: Option<i32>,
    pub aperture: Option<f32>,
    pub focal_length: Option<u16>,
    pub focal_length_35mm: Option<u16>,
    pub camera_model: Option<String>,
    pub owner: ShortInfo
}

#[derive(PartialEq, Clone, Debug)]
pub enum ImageType {
    Jpeg,
    Png
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &ImageType::Jpeg => write!(f, "{}", "jpg".to_string() ),
            &ImageType::Png => write!(f, "{}", "png".to_string() )
        }
    }
}

impl Encodable for ImageType {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        format!( "{}", self ).encode( s )
    }
}

#[derive(RustcEncodable)]
pub struct MailInfo {
    pub id: Id,
    pub creation_time: i64,
    pub sender_name: String,
    pub subject: String,
    pub body: String,
    pub readed: bool
}

#[derive(Clone, Debug, RustcEncodable)]
pub struct ShortInfo {
    pub id: Id,
    pub name: String
}

#[derive(Clone, Debug, RustcEncodable)]
pub struct CommentInfo {
    pub id: Id,
    pub creation_time: u64,
    pub edit_time: u64,
    pub creator: ShortInfo,
    pub text: String,
    pub is_editable: bool,
    pub is_new: bool
}
