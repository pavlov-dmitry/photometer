use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use std::sync::{ Arc };
use std::io;
use std::io::{ IoResult, USER_RWX };
use std::io::fs::{ mkdir_recursive, File };
use authentication::{ User };
use image;
use image::{ GenericImage, DynamicImage };
use std::cmp::{ min, max };
use time::{ Timespec };
use types::{ ImageType };
use typemap::Assoc;
use plugin::Extensible;

static GALLERY_DIR : &'static str = "gallery";

pub fn middleware( photo_dir: &String, max_photo_size_bytes: uint, preview_size: uint ) -> PhotoStore {
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
    max_photo_size_bytes : uint, 
    preview_size : u32,
}

#[deriving(Clone)]
pub struct PhotoStore {
    params : Arc<Params>
}

pub enum PhotoStoreError {
    /// произошла ошибка работы с файловой системой
    Fs( io::IoError ),
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
    pub fn init_user_dir( &self, user: &str ) -> IoResult<()> {
        mkdir_recursive( &Path::new( format!( "{}/{}/{}", self.params.photos_dir, user, GALLERY_DIR ) ), USER_RWX )
    }
    /// добавляет фотографию привязанную к опеределнному событию
    pub fn add_new_photo( &self, user: &User, upload_time: &Timespec, img_type: ImageType, img_data: &[u8] ) -> PhotoStoreResult<(u32, u32)> {
        if img_data.len() < self.params.max_photo_size_bytes {
            match image::load_from_memory( img_data, to_image_format( &img_type ) ) {
                Ok( mut img ) => {
                    let (w, h) = img.dimensions();
                    let crop_size = min( w, h );
                    //let mut preview = img.crop( w / 2 - crop_size / 2, h / 2 - crop_size / 2, crop_size, crop_size );
                    //preview = preview.resize_exact( self.params.preview_size, self.params.preview_size, image::Lanczos3 );
                    let preview_filename = self.make_filename( &user.name, upload_time, &img_type, true );
                    let fs_sequience = 
                        self.save_preview( 
                            &mut img, 
                            preview_filename, 
                            ( w / 2 - crop_size / 2, h / 2 - crop_size / 2 ), 
                            ( crop_size, crop_size ) 
                        )
                        .and_then( |_| File::create( &self.make_filename( &user.name, upload_time, &img_type, false ) ) )
                        .and_then( |mut file| file.write( img_data ) );
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

    pub fn save_preview( &self, img: &mut DynamicImage, filename: Path, (tlx, tly): (u32, u32), (w, h) : (u32, u32) ) -> IoResult<()> {
        let mut preview = img.crop( tlx, tly, w, h );
        preview = preview.resize( self.params.preview_size, self.params.preview_size, image::Nearest );
        let preview_file = try!( File::create( &filename ) );
        let _ = try!( preview.save( preview_file, image::PNG ) );
        Ok( () )
    }

    pub fn make_crop( 
        &self, user: &String, 
        upload_time: Timespec, 
        image_type: ImageType, 
        (tlx, tly): (u32, u32), 
        (brx, bry) : (u32, u32) 
    ) -> PhotoStoreResult<()> {
        match image::open( &self.make_filename( user, &upload_time, &image_type, false ) ) {
            Ok( mut img ) => {
                let (w, h) = img.dimensions();
                let (tlx, tly) = ( min( w, tlx ), min( h, tly ) );
                let (brx, bry) = ( min( w, brx ), min( h, bry ) );
                let (brx, bry) = ( max( tlx, brx ), max( tly, bry ) );
                let top_left = ( min( tlx, brx ), min( tly, bry ) );
                let dimensions = ( brx - tlx, bry - tly );
                self.save_preview( &mut img, self.make_filename( user, &upload_time, &image_type, true ), top_left, dimensions )
                    .map_err( |e| PhotoStoreError::Fs( e ) )
            },
            _ => Err( PhotoStoreError::Format )
        }
    }

    /// формирует имя файла в зависимости от пользователя и события
    pub fn make_filename( &self, user: &String, upload_time: &Timespec, tp: &ImageType, is_preview: bool ) -> Path {
        let extension = if *tp == ImageType::Png || is_preview { "png" } else { "jpg" };
        let postfix = if is_preview { "_preview" } else { "" };
        Path::new( 
            format!( "{}/{}/{}/{}{}.{}",
                self.params.photos_dir,
                user,
                GALLERY_DIR,
                upload_time.sec,
                postfix,
                extension
            )  
        )
    }
}

impl Assoc<PhotoStore> for PhotoStore {}

impl Middleware for PhotoStore {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.extensions_mut().insert::<PhotoStore, PhotoStore>( self.clone() );
        Ok( Continue )
    } 
}

pub trait PhotoStoreable {
    fn photo_store(&self) -> &PhotoStore;
}

impl<'a, 'b> PhotoStoreable for Request<'a, 'b> {
    fn photo_store( &self ) -> &PhotoStore {
        self.extensions().get::<PhotoStore, PhotoStore>().unwrap()
    }
}
