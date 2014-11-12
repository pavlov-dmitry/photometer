use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use sync::{ Arc };
use std::io;
use std::io::{ IoResult, USER_RWX };
use std::io::fs::{ mkdir_recursive, File };
use authentication::{ User };
use image;
use image::{ GenericImage };
use std::cmp::{ min };
use time::{ Timespec };
use photo_info::{ ImageType };
use database::{ Databaseable, Id };

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
    AllCorrect( u32, u32 ),
    /// произошла ошибка работы с файловой системой
    FsError( io::IoError ),
    /// ошибка в формате данных
    FormatError,
    /// ошибка размера данных
    FileSizeError
}

fn to_image_format( t: ImageType ) -> image::ImageFormat {
    match t {
        ImageType::Jpeg => image::JPEG,
        ImageType::Png => image::PNG
    }
}

impl PhotoStore {
    /// инициализирует директории пользователя для хранения фотографий
    pub fn init_user_dir( &self, user: &str ) -> IoResult<()> {
        mkdir_recursive( &Path::new( format!( "{}/{}/{}", self.params.photos_dir, user, GALLERY_DIR ) ), USER_RWX )
    }
    /// добавляет фотографию привязанную к опеределнному событию
    pub fn add_new_photo( &self, user: &User, upload_time: &Timespec, img_type: ImageType, img_data: &[u8] ) -> PhotoResult {
        if img_data.len() < self.params.max_photo_size_bytes {
            match image::load_from_memory( img_data, to_image_format( img_type ) ) {
                Ok( mut img ) => {
                    let (w, h) = img.dimensions();
                    let crop_size = min( w, h );
                    let mut preview = img.crop( w / 2 - crop_size / 2, h / 2 - crop_size / 2, crop_size, crop_size );
                    preview = preview.resize_exact( self.params.preview_size, self.params.preview_size, image::Lanczos3 );
                    let fs_sequience = 
                        File::create( &self.make_filename( &user.name, upload_time, img_type, true ) )
                        //превью будет пока в пнг формате, так как image сохраняет в джипег в очень слабом качестве
                        .and_then( |preview_file| preview.save( preview_file, image::PNG ) )
                        .and_then( |_| File::create( &self.make_filename( &user.name, upload_time, img_type, false ) ) )
                        .and_then( |mut file| file.write( img_data ) );
                    match fs_sequience {
                        Ok(_) => AllCorrect( w, h ),
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
    pub fn make_filename( &self, user: &String, upload_time: &Timespec, tp: ImageType, is_preview: bool ) -> Path {
        let extension = if tp == ImageType::Png || is_preview { "png" } else { "jpg" };
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

pub fn files_router_path() -> &'static str {
    "/photo/:filename.:ext"
}

pub fn get_photo( req: &Request, res: &mut Response ) {
    match from_str::<Id>( req.param( "filename" ) ) {
        Some( id ) => {
            match req.db().get_photo_info( id ).unwrap_or_else( |e| panic!( e ) ) {
                Some( (user, info) ) => {
                    let _ = res.send_file( 
                        &req.photo_store().make_filename(
                            &user,
                            &info.upload_time,
                            info.image_type,
                            false
                        )
                    );
                }
                None => {}
            }
        }
        None => {}
    }
}

pub fn preview_router_path() -> &'static str {
    "/preview/:filename.:ext"
}

pub fn get_preview( req: &Request, res: &mut Response ) {
    match from_str::<Id>( req.param( "filename" ) ) {
        Some( id ) => {
            match req.db().get_photo_info( id ).unwrap_or_else( |e| panic!( e ) ) {
                Some( (user, info) ) => {
                    let _ = res.send_file( 
                        &req.photo_store().make_filename(
                            &user,
                            &info.upload_time,
                            info.image_type,
                            true
                        )
                    );
                }
                None => {}
            }
        }
        None => {}
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
        self.map.get::<PhotoStore>().unwrap()
    }
}
