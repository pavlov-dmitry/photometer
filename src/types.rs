use time::{ Timespec };
use std::fmt;
use std::fmt::{ Display, Formatter };

pub type Id = u64;
pub type CommonResult<T> = Result<T, String>;
pub type EmptyResult = CommonResult<()>;

#[derive(Debug)]
pub struct PhotoInfo {
    pub id: Id,
    pub upload_time: Timespec,
    pub image_type: ImageType,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub iso: Option<u32>,
    pub shutter_speed: Option<i32>,
    pub aperture: Option<f32>,
    pub focal_length: Option<u16>,
    pub focal_length_35mm: Option<u16>,
    pub camera_model: Option<String>
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

pub struct MailInfo {
    pub id: Id,
    pub creation_time: Timespec,
    pub sender_name: String,
    pub subject: String,
    pub body: String,
    pub readed: bool
}