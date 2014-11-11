use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use sync::{ Arc };
use std::io;
use std::io::{ IoResult, USER_RWX };
use std::io::fs::{ mkdir_recursive, File };
use authentication::{ User };
use image;
use image::{ GenericImage };
use std::cmp::{ min };
use exif_reader;
use exif_reader::{ ExifValues };
use time::{ Timespec };

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


pub enum PhotoResult {
    /// Всё прошло отлично
    AllCorrect,
    /// произошла ошибка работы с файловой системой
    FsError( io::IoError ),
    /// ошибка в формате данных
    FormatError,
    /// ошибка размера данных
    FileSizeError
}


impl PhotoStore {
    /// инициализирует директории пользователя для хранения фотографий
    pub fn init_user_dir( &self, user: &str ) -> IoResult<()> {
        mkdir_recursive( &Path::new( format!( "{}/{}/{}", self.params.photos_dir, user, GALLERY_DIR ) ), USER_RWX )
    }
    /// добавляет фотографию привязанную к опеределнному событию
    pub fn add_new_photo( &self, user: &User, upload_time: &Timespec, img_data: &[u8] ) -> PhotoResult {
        if img_data.len() < self.params.max_photo_size_bytes {
            //TODO: remove after test
            match exif_reader::from_memory( img_data ) {
                Some( exif ) => {
                    println!( "camera={}", exif.camera_model() );
                    println!( "iso={}", exif.iso() );
                    println!( "shutter={}", exif.shutter_speed() );
                    println!( "focal_length={}", exif.focal_length() );
                    println!( "focal_length_35mm={}", exif.focal_length_35mm() );
                    println!( "aperture={}", exif.aperture() ); 
                },
                None => println!( "no exif" )
            }
            

            match image::load_from_memory( img_data, image::JPEG ) {
                Ok( mut img ) => {
                    let (w, h) = img.dimensions();
                    let crop_size = min( w, h );
                    let mut preview = img.crop( w / 2 - crop_size / 2, h / 2 - crop_size / 2, crop_size, crop_size );
                    preview = preview.resize_exact( self.params.preview_size, self.params.preview_size, image::Lanczos3 );
                    let fs_sequience = 
                        File::create( &self.make_filename( user, upload_time, true ) )
                        //превью будет пока в пнг формате, так как image сохраняет в джипег в очень слабом качестве
                        .and_then( |preview_file| preview.save( preview_file, image::PNG ) )
                        .and_then( |_| File::create( &self.make_filename( user, upload_time, false ) ) )
                        .and_then( |mut file| file.write( img_data ) );
                    match fs_sequience {
                        Ok(_) => AllCorrect,
                        Err( e ) => FsError( e )
                    }
                }
                _ => FormatError
            }
        }
        else {
            FileSizeError
        }
    }

    /// возвращает путь к файлу определенного пользователя с определнного события
    pub fn get_path_to( &self, user: &str, event: &str, filename: &str, ext: &str ) -> Path {
        Path::new( format!( "{}/{}/{}/{}.{}", self.params.photos_dir, user, event, filename, ext ) )
    }
    /// формирует имя файла в зависимости от пользователя и события
    fn make_filename( &self, user: &User, upload_time: &Timespec, is_preview: bool ) -> Path {
        let postfix = if is_preview { "_preview.png" } else { ".jpg" };
        Path::new( 
            format!( "{}/{}/{}/{}{}",
                self.params.photos_dir,
                user.name,
                GALLERY_DIR,
                upload_time.sec,
                postfix
            )  
        )
    }
}

pub fn files_router_path() -> &'static str {
    "/photo/:user/:event/:filename.:ext"
}

pub fn get_photo( req: &Request, res: &mut Response ) {
    let _ = res.send_file( 
        &req.photo_store().get_path_to( 
            req.param( "user" ),
            req.param( "event" ),
            req.param( "filename" ),
            req.param( "ext" ) 
        )
    );
}

impl Middleware for PhotoStore {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.map.insert( self.clone() );
        Ok( Continue )
    } 
}

pub trait PhotoStoreable {
    fn photo_store(&self) -> &PhotoStore;
}

impl<'a, 'b> PhotoStoreable for Request<'a, 'b> {
    fn photo_store( &self ) -> &PhotoStore {
        self.map.get::<PhotoStore>().unwrap()
    }
}
