use std::sync::{ Arc };
use std::fs::{ self, File };
use std::io::{ self, Write };
use std::path::{ Path };
use authentication::{ User };
use image;
use image::{ GenericImage, DynamicImage, ImageResult, ImageError };
use std::cmp::{ min, max };
use time::{ Timespec };
use types::{ ImageType };
use iron::typemap::Key;
use iron::middleware::BeforeMiddleware;
use iron::prelude::*;
use parse_utils::{ GetMsecs };

static GALLERY_DIR : &'static str = "gallery";

pub fn middleware( photo_dir: &String, preview_size: usize ) -> PhotoStore {
    PhotoStore {
        params: Arc::new(
            Params {
                photos_dir: (*photo_dir).clone(),
                preview_size: preview_size as u32
            }
        )
    }
}

struct Params {
    photos_dir : String,
    preview_size : u32,
}

#[derive(Clone)]
pub struct PhotoStore {
    params : Arc<Params>
}

pub enum PhotoStoreError {
    /// ошибка работы с фотографией
    Image( ImageError ),
    /// произошла ошибка работы с файловой системой
    Fs( io::Error ),
    /// ошибка в формате данных
    Format
}

pub type PhotoStoreResult<T> = Result<T, PhotoStoreError>;

fn to_image_format( t: &ImageType ) -> image::ImageFormat {
    match t {
        &ImageType::Jpeg => image::JPEG,
        &ImageType::Png => image::PNG
    }
}

impl PhotoStore {
    /// инициализирует директории пользователя для хранения фотографий
    pub fn init_user_dir( &self, user: &str ) -> io::Result<()> {
        fs::create_dir_all( &format!( "{}/{}/{}", self.params.photos_dir, user, GALLERY_DIR ) )
    }
    /// добавляет фотографию привязанную к опеределнному событию
    pub fn add_new_photo( &self, user: &User, upload_time: &Timespec, img_type: ImageType, img_data: &[u8] ) -> PhotoStoreResult<(u32, u32)> {
        match image::load_from_memory_with_format( img_data, to_image_format( &img_type ) ) {
            Ok( mut img ) => {
                let (w, h) = img.dimensions();
                let crop_size = min( w, h );
                let preview_filename = self.make_filename( &user.name,
                                                            upload_time.msecs(),
                                                            &img_type,
                                                            true );
                let preview_filename = Path::new( &preview_filename );

                let photo_filename = self.make_filename( &user.name,
                                                          upload_time.msecs(),
                                                          &img_type,
                                                          false );
                let photo_filename = Path::new( &photo_filename );

                //TODO: переписать по красивше
                try!( self.save_preview(
                    &mut img,
                    preview_filename,
                    ( w / 2 - crop_size / 2, h / 2 - crop_size / 2 ),
                    ( crop_size, crop_size )));

                let fs_sequience = File::create( &photo_filename )
                    .and_then( |mut file| file.write_all( img_data ) );

                match fs_sequience {
                    Ok(_) => Ok( (w, h) ),
                    Err( e ) => Err( PhotoStoreError::Fs( e ) )
                }
            }
            Err( e ) => {
                info!( "can`t load image: {}", e );
                Err( PhotoStoreError::Format )
            }
        }
    }

    pub fn save_preview( &self, img: &mut DynamicImage, filename: &Path, (tlx, tly): (u32, u32), (w, h) : (u32, u32) ) -> ImageResult<()> {
        let mut preview = img.crop( tlx, tly, w, h );
        preview = preview.resize( self.params.preview_size, self.params.preview_size, image::Nearest );
        let mut preview_file = try!( File::create( &filename ) );
        let _ = try!( preview.save( &mut preview_file, image::PNG ) );
        Ok( () )
    }

    pub fn make_crop(
        &self, user: &String,
        upload_time: u64,
        image_type: ImageType,
        (tlx, tly): (u32, u32),
        (brx, bry) : (u32, u32)
    ) -> PhotoStoreResult<()> {
        let filename = self.make_filename( user, upload_time, &image_type, false );
        let filename = Path::new( &filename );

        match image::open( filename ) {
            Ok( mut img ) => {
                let (w, h) = img.dimensions();
                let (tlx, tly) = ( min( w, tlx ), min( h, tly ) );
                let (brx, bry) = ( min( w, brx ), min( h, bry ) );
                let (brx, bry) = ( max( tlx, brx ), max( tly, bry ) );
                let top_left = ( min( tlx, brx ), min( tly, bry ) );
                let dimensions = ( brx - tlx, bry - tly );

                let preview_filename = self.make_filename( user, upload_time, &image_type, true );
                let preview_filename = Path::new( &preview_filename );

                self.save_preview( &mut img, preview_filename, top_left, dimensions )
                    .map_err( |e| PhotoStoreError::Image( e ) )
            },
            _ => Err( PhotoStoreError::Format )
        }
    }

    /// формирует имя файла в зависимости от пользователя и события
    pub fn make_filename( &self, user: &String, upload_time: u64, tp: &ImageType, is_preview: bool ) -> String {
        let extension = if *tp == ImageType::Png || is_preview { "png" } else { "jpg" };
        let postfix = if is_preview { "_preview" } else { "" };
        //Path::new(
            format!( "{}/{}/{}/{}{}.{}",
                self.params.photos_dir,
                user,
                GALLERY_DIR,
                upload_time,
                postfix,
                extension
            )
        //)
    }
}

impl From<ImageError> for PhotoStoreError {
    fn from(err: ImageError) -> PhotoStoreError {
        PhotoStoreError::Image( err )
    }
}

impl Key for PhotoStore { type Value = PhotoStore; }

impl BeforeMiddleware for PhotoStore {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<PhotoStore>( self.clone() );
        Ok( () )
    }
}

pub trait PhotoStoreable {
    fn photo_store(&self) -> &PhotoStore;
}

impl<'a, 'b> PhotoStoreable for Request<'a, 'b> {
    fn photo_store( &self ) -> &PhotoStore {
        self.extensions.get::<PhotoStore>().unwrap()
    }
}
