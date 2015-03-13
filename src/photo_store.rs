use std::sync::{ Arc };
use std::old_io;
use std::old_io::IoResult;
#[allow(deprecated)]
use std::old_io::fs::File;
use std::old_path::posix::Path;
use std::fs;
use std::io;
use authentication::{ User };
use image;
use image::{ GenericImage, DynamicImage };
use std::cmp::{ min, max };
use time::{ Timespec };
use types::{ ImageType };
use iron::typemap::Key;
use iron::middleware::BeforeMiddleware;
use iron::prelude::*;

static GALLERY_DIR : &'static str = "gallery";

pub fn middleware( photo_dir: &String, max_photo_size_bytes: usize, preview_size: usize ) -> PhotoStore {
    PhotoStore {
        params: Arc::new( 
            Params {
                photos_dir: (*photo_dir).clone(),
                max_photo_size_bytes: max_photo_size_bytes,
                preview_size: preview_size as u32
            }
        )
    }   
}

struct Params {
    photos_dir : String,
    max_photo_size_bytes : usize, 
    preview_size : u32,
}

#[derive(Clone)]
pub struct PhotoStore {
    params : Arc<Params>
}

pub enum PhotoStoreError {
    /// произошла ошибка работы с файловой системой
    Fs( old_io::IoError ),
    /// ошибка в формате данных
    Format,
    /// ошибка размера данных
    FileSize
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
    #[allow(deprecated)]
    pub fn add_new_photo( &self, user: &User, upload_time: &Timespec, img_type: ImageType, img_data: &[u8] ) -> PhotoStoreResult<(u32, u32)> {
        if img_data.len() < self.params.max_photo_size_bytes {
            match image::load_from_memory_with_format( img_data, to_image_format( &img_type ) ) {
                Ok( mut img ) => {
                    let (w, h) = img.dimensions();
                    let crop_size = min( w, h );
                    //let mut preview = img.crop( w / 2 - crop_size / 2, h / 2 - crop_size / 2, crop_size, crop_size );
                    //preview = preview.resize_exact( self.params.preview_size, self.params.preview_size, image::Lanczos3 );
                    let preview_filename = self.make_filename( &user.name, upload_time, &img_type, true );
                    let fs_sequience = 
                        self.save_preview( 
                            &mut img, 
                            Path::new( preview_filename ), 
                            ( w / 2 - crop_size / 2, h / 2 - crop_size / 2 ), 
                            ( crop_size, crop_size ) 
                        )
                        .and_then( |_| File::create( 
                            &Path::new( &self.make_filename( &user.name, upload_time, &img_type, false ) )
                        ) )
                        .and_then( |mut file| file.write_all( img_data ) );
                    match fs_sequience {
                        Ok(_) => Ok( (w, h) ),
                        Err( e ) => Err( PhotoStoreError::Fs( e ) )
                    }
                }
                _ => Err( PhotoStoreError::Format )
            }
        }
        else {
            Err( PhotoStoreError::FileSize )
        }
    }

    #[allow(deprecated)]
    pub fn save_preview( &self, img: &mut DynamicImage, filename: Path, (tlx, tly): (u32, u32), (w, h) : (u32, u32) ) -> IoResult<()> {
        let mut preview = img.crop( tlx, tly, w, h );
        preview = preview.resize( self.params.preview_size, self.params.preview_size, image::Nearest );
        let mut preview_file = try!( File::create( &filename ) );
        let _ = try!( preview.save( &mut preview_file, image::PNG ) );
        Ok( () )
    }

    pub fn make_crop( 
        &self, user: &String, 
        upload_time: Timespec, 
        image_type: ImageType, 
        (tlx, tly): (u32, u32), 
        (brx, bry) : (u32, u32) 
    ) -> PhotoStoreResult<()> {
        let filename = self.make_filename( user, &upload_time, &image_type, false );
        match image::open( &Path::new( filename ) ) {
            Ok( mut img ) => {
                let (w, h) = img.dimensions();
                let (tlx, tly) = ( min( w, tlx ), min( h, tly ) );
                let (brx, bry) = ( min( w, brx ), min( h, bry ) );
                let (brx, bry) = ( max( tlx, brx ), max( tly, bry ) );
                let top_left = ( min( tlx, brx ), min( tly, bry ) );
                let dimensions = ( brx - tlx, bry - tly );
                let preview_filename = self.make_filename( user, &upload_time, &image_type, true );
                self.save_preview( &mut img, Path::new( preview_filename ), top_left, dimensions )
                    .map_err( |e| PhotoStoreError::Fs( e ) )
            },
            _ => Err( PhotoStoreError::Format )
        }
    }

    /// формирует имя файла в зависимости от пользователя и события
    pub fn make_filename( &self, user: &String, upload_time: &Timespec, tp: &ImageType, is_preview: bool ) -> String {
        let extension = if *tp == ImageType::Png || is_preview { "png" } else { "jpg" };
        let postfix = if is_preview { "_preview" } else { "" };
        //Path::new( 
            format!( "{}/{}/{}/{}{}.{}",
                self.params.photos_dir,
                user,
                GALLERY_DIR,
                upload_time.sec,
                postfix,
                extension
            )  
        //)
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

impl<'a> PhotoStoreable for Request<'a> {
    fn photo_store( &self ) -> &PhotoStore {
        self.extensions.get::<PhotoStore>().unwrap()
    }
}
