use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use sync::{ Arc };
use std::io::{ IoResult, USER_RWX  };
use std::io::fs::{ mkdir_recursive, File };
use authentication::{ User };
use photo_event::{ PhotoEvent, Weekly };

static WEEKLY_DIR : &'static str = "weekly";

pub fn middleware( photo_dir: &String, max_photo_size_bytes: uint ) -> PhotoStore {
    PhotoStore {
        photos_dir: Arc::new( (*photo_dir).clone() ),
        max_photo_size_bytes: max_photo_size_bytes
    }   
}

#[deriving(Clone)]
pub struct PhotoStore {
    photos_dir : Arc<String>,
    pub max_photo_size_bytes : uint
}


impl PhotoStore {
    pub fn init_user_dir( &self, user: &str ) -> IoResult<()> {
        mkdir_recursive( &Path::new( format!( "{}/{}/{}", *self.photos_dir, user, WEEKLY_DIR ) ), USER_RWX )
    }
    pub fn add_new_photo( &self, user: &User, event: &PhotoEvent, img_data: &[u8] ) -> IoResult<()> {
        File::create( &self.make_filename( user, event ) )
            .and_then( |mut file| file.write( img_data ) )
    }
    fn make_filename( &self, user: &User, event: &PhotoEvent ) -> Path {
        match event {
            &Weekly{ year, week } => Path::new( 
                format!( "{}/{}/{}/{}_{}.jpg",
                    *self.photos_dir,
                    user.name,
                    WEEKLY_DIR,
                    year,
                    week
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


