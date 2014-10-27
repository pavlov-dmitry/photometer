use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use sync::{ Arc };
use std::io;
use std::io::{ IoResult, USER_RWX };
use std::io::fs::{ mkdir_recursive, File };
use authentication::{ User };
use photo_event::{ PhotoEvent, Weekly };
use image;
use image::{ GenericImage };
use std::cmp::{ min };

static WEEKLY_DIR : &'static str = "weekly";

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
        mkdir_recursive( &Path::new( format!( "{}/{}/{}", self.params.photos_dir, user, WEEKLY_DIR ) ), USER_RWX )
    }
    /// добавляет фотографию привязанную к опеределнному событию
    pub fn add_new_photo( &self, user: &User, event: &PhotoEvent, img_data: &[u8] ) -> PhotoResult {
        if img_data.len() < self.params.max_photo_size_bytes {
            match image::load_from_memory( img_data, image::JPEG ) {
                Ok( mut img ) => {
                    let (w, h) = img.dimensions();
                    let crop_size = min( w, h );
                    let mut preview = img.crop( w / 2 - crop_size / 2, h / 2 - crop_size / 2, crop_size, crop_size );
                    preview = preview.resize_exact( self.params.preview_size, self.params.preview_size, image::Lanczos3 );
                    let fs_sequience = 
                        File::create( &self.make_filename( user, event, true ) )
                        //превью будет пока в пнг формате, так как image сохраняет в джипег в очень слабом качестве
                        .and_then( |preview_file| preview.save( preview_file, image::PNG ) )
                        .and_then( |_| File::create( &self.make_filename( user, event, false ) ) )
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
    /// формирует имя файла в зависимости от пользователя и события
    fn make_filename( &self, user: &User, event: &PhotoEvent, is_preview: bool ) -> Path {
        let postfix = if is_preview { "_preview.png" } else { ".jpg" };
        match event {
            &Weekly{ year, week } => Path::new( 
                format!( "{}/{}/{}/{}_{}{}",
                    self.params.photos_dir,
                    user.name,
                    WEEKLY_DIR,
                    year,
                    week,
                    postfix
                )  
            )
        }
    }
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
        self.map.find::<PhotoStore>().unwrap()
    }
}


