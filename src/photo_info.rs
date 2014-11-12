use time::{ Timespec };

pub struct PhotoInfo {
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

#[deriving(PartialEq)]
pub enum ImageType {
    Jpeg,
    Png
}