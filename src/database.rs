/// модуль работы с БД
use mysql::conn::{ MyOpts };
use mysql::conn::pool::{ MyPool, MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value, from_value_opt, ToValue, FromValue, Value };
use std::default::{ Default };
use std::slice::{ Items };

use nickel::{ Request, Response, Continue, MiddlewareResult, Middleware };
use time::{ Timespec };
use types::{ Id, PhotoInfo, ImageType };

#[deriving(Clone)]
pub struct Database {
    pool: MyPool
}

pub struct DatabaseConn {
    connection: MyResult<MyPooledConn>
}

pub type DBResult<T> = Result<T, String>;

const ISO_DEFAULT: u32 = 0;
const SHUTTER_SPEED_DEFAULT: i32 = 0;
const APERTURE_DEFAULT: f32 = 0.;
const FOCAL_LENGTH_DEFAULT: u16 = 0;
const FOCAL_LENGTH_35MM_DEFAULT: u16 = 0;
const CAMERA_MODEL_DEFAULT: &'static str = "";

impl DatabaseConn {
    fn get_conn(&mut self) -> DBResult<&mut MyPooledConn> {
        self.connection.as_mut().map_err( |e| format!( "Database creating connection failed: {}", e ) )
    }
    /// выбирает id пользователя по имени и паролю
    pub fn get_user( &mut self, name: &str, pass: &str ) -> DBResult<Option<Id>> {
        let connection = try!( self.get_conn() );
        DatabaseConn::get_user_impl( connection, name, pass )
            .map_err( |e| format!( "Database func 'get_user' failed: {}", &e ) )
    }
    fn get_user_impl( conn: &mut MyPooledConn, name: &str, pass: &str ) -> MyResult<Option<Id>> {
        let name = name.to_string(); // помогает убрать internal compiler error
        let pass = pass.to_string();
        let mut stmt = try!( conn.prepare( "select id from users where login=? and password=?" ) );
        let mut sql_result = try!( stmt.execute( &[ &name, &pass ] ) );
        sql_result.next().map_or( Ok( None ),
            |row| row.and_then( |r| Ok( Some( from_value::<Id>( &r[0] ) ) ) )
        )
    }
    /// добавляет нового пользователя в БД
    pub fn add_user( &mut self, name: &str, pass: &str ) -> DBResult<()> {
        let name = name.to_string();
        let pass = pass.to_string();
        self.get_conn()
            .and_then( |connection| connection.prepare( "INSERT INTO users (login, password) VALUES(?, ?);" )
                .and_then( |ref mut stmt| stmt.execute( &[ &name, &pass ] ).and( Ok( () ) ) )
                .map_err( |e| format!( "Database func 'add_user' failed: {}", e ) )
            )
    }
    /// проверяет наличие имени в БД
    pub fn user_exists(&mut self, name: &str,) -> DBResult<bool> {
        let connection = try!( self.get_conn() );
        DatabaseConn::user_exists_impl( connection, name )
            .map_err( |e| format!( "Database func 'user_exists' failed: {}", &e ) )
    }
    fn user_exists_impl( conn: &mut MyPooledConn, name: &str  ) -> MyResult<bool> {
        let name = name.to_string();
        let mut stmt = try!( conn.prepare( "select id from users where login=?" ) );
        let sql_result = try!( stmt.execute( &[ &name ] ) );
        Ok( sql_result.count() == 1 )
    }
    /// добавление фотографии в галлерею пользователя
    pub fn add_photo( &mut self, user_id: Id, info: &PhotoInfo ) -> DBResult<()> {
        let connection = try!( self.get_conn() );
        DatabaseConn::add_photo_impl( connection, user_id, info )
            .map_err( |e| format!( "Database func 'add_photo' failed: {}", &e ) )
    }
    fn add_photo_impl( conn: &mut MyPooledConn, user_id: Id, info: &PhotoInfo ) -> MyResult<()> {
        let mut stmt = try!( conn.prepare( 
            "insert into images (
            owner_id, 
            upload_time, 
            type,
            width,
            height,
            name,
            iso,
            shutter_speed,
            aperture,
            focal_length,
            focal_length_35mm,
            camera_model )
            values( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ? )" ) 
        );
        try!( stmt.execute( &[
            &user_id,
            &info.upload_time.sec,
            &info.image_type,
            &info.width,
            &info.height,
            &info.name,
            &info.iso.unwrap_or( ISO_DEFAULT ),
            &info.shutter_speed.unwrap_or( SHUTTER_SPEED_DEFAULT ),
            &info.aperture.unwrap_or( APERTURE_DEFAULT ),
            &info.focal_length.unwrap_or( FOCAL_LENGTH_DEFAULT ),
            &info.focal_length_35mm.unwrap_or( FOCAL_LENGTH_35MM_DEFAULT ),
            info.camera_model.as_ref().unwrap_or( &CAMERA_MODEL_DEFAULT.to_string() )
        ]));
        Ok( () )
    }

    /// получение информации о фото
    pub fn get_photo_info( &mut self, photo_id: Id ) -> DBResult<Option<(String, PhotoInfo)>> {
        let connection = try!( self.get_conn() );
        DatabaseConn::get_photo_info_impl( connection, photo_id )
            .map_err( |e| format!( "Database func 'get_photo_info' failed: {}", &e ) )
    }
    fn get_photo_info_impl( conn: &mut MyPooledConn, photo_id: Id ) -> MyResult<Option<(String, PhotoInfo)>> {
        let mut stmt = try!( conn.prepare( "SELECT 
            u.login, 
            i.id,
            i.upload_time,
            i.type,
            i.width,
            i.height,
            i.name,
            i.iso,
            i.shutter_speed,
            i.aperture,
            i.focal_length,
            i.focal_length_35mm,
            i.camera_model
            FROM images AS i LEFT JOIN users AS u ON ( u.id = i.owner_id )
            WHERE u.id IS NOT NULL AND i.id = ?" ) 
        );
        let mut sql_result = try!( stmt.execute( &[ &photo_id ] ) );
        match sql_result.next() {
            None => Ok( None ), 
            Some( sql_row ) => {
                let row_data = try!( sql_row );
                let mut values = row_data.iter();
                Ok ( Some ( (
                    from_value( values.next().unwrap() ),
                    read_photo_info( &mut values )
                ) ) )
            }
        } 
    }

    ///возвращает список описаний фоточек
    pub fn get_photo_infos( &mut self, owner_id: Id, start: Timespec, end: Timespec, offset: u32, count: u32 ) -> DBResult<Vec<PhotoInfo>> {
        let connection = try!( self.get_conn() );
        DatabaseConn::get_photo_infos_impl( connection, owner_id, start, end, offset, count )
            .map_err( |e| format!( "Database func 'get_photo_infos' failed: {}", &e ) )
    }
    pub fn get_photo_infos_impl( 
        conn: &mut MyPooledConn, 
        owner_id: Id, 
        start: Timespec, 
        end: Timespec, 
        offset: u32, 
        count: u32 
    ) -> MyResult<Vec<PhotoInfo>> {
        let mut stmt = try!( conn.prepare( "SELECT 
           id,
           upload_time,
           type,
           width,
           height,
           name,
           iso,
           shutter_speed,
           aperture,
           focal_length,
           focal_length_35mm,
           camera_model
           FROM images
           WHERE owner_id = ? AND upload_time BETWEEN ? AND ?
           ORDER BY upload_time ASC
           LIMIT ? OFFSET ?;
        " ) );
        let result = try!( stmt.execute( &[ &owner_id, &start.sec, &end.sec, &count, &offset ] ) );
        //что-то с преобразованием на лету через собственный итертор я подупрел =(, пришлось тупо собирать в новый массив
        Ok( 
            result.filter_map( |sql_row| 
                sql_row.ok().map( |sql_values| {
                    let mut values = sql_values.iter();
                    read_photo_info( &mut values )
                })
            ).collect()
        )
    }

    ///вычисляет кол-во фоток пользователя за опеределнный период
    pub fn get_photo_infos_count( &mut self, owner_id: Id, start: Timespec, end: Timespec ) -> DBResult<u32> {
        let connection = try!( self.get_conn() );
        DatabaseConn::get_photo_infos_count_impl( connection, owner_id, start, end )
            .map_err( |e| format!( "Database func 'get_photo_infos_count' failed: {}", &e ) )
    }
    pub fn get_photo_infos_count_impl( conn: &mut MyPooledConn, owner_id: Id, start: Timespec, end: Timespec ) -> MyResult<u32> {
        let mut stmt = try!( conn.prepare( "SELECT COUNT(id) FROM images WHERE owner_id = ? AND upload_time BETWEEN ? AND ?" ) ); 
        let mut result = try!( stmt.execute( &[ &owner_id, &start.sec, &end.sec ] ) );
        let sql_row = try!( result.next().unwrap() );
        Ok( from_value( &sql_row[ 0 ] ) )
    }

    ///переименование фотографии
    pub fn rename_photo( &mut self, photo_id: Id, newname: &str ) -> DBResult<()> {
        let connection = try!( self.get_conn() );
        DatabaseConn::rename_photo_impl( connection, photo_id, newname )
            .map_err( |e| format!( "Database func 'rename_photo' failed: {}", &e ) )
    }
    fn rename_photo_impl( conn: &mut MyPooledConn, photo_id: Id, newname: &str ) -> MyResult<()> {
        let newname = newname.to_string();
        let mut stmt = try!( conn.prepare( "UPDATE images SET name=? WHERE id=?" ) );
        let _ = try!( stmt.execute( &[ &newname, &photo_id ] ) );
        Ok( () )
    }
}

fn read_photo_info( values: &mut Items<Value> ) -> PhotoInfo {
    PhotoInfo {
        id: from_value( values.next().unwrap() ),
        upload_time: Timespec::new( from_value( values.next().unwrap() ), 0 ),
        image_type: from_value( values.next().unwrap() ),
        width: from_value( values.next().unwrap() ),
        height: from_value( values.next().unwrap() ),
        name: from_value( values.next().unwrap() ),
        iso: if_not( from_value( values.next().unwrap() ), ISO_DEFAULT ),
        shutter_speed: if_not( from_value( values.next().unwrap() ), SHUTTER_SPEED_DEFAULT ),
        aperture: if_not( from_value( values.next().unwrap() ), APERTURE_DEFAULT ),
        focal_length: if_not( from_value( values.next().unwrap() ), FOCAL_LENGTH_DEFAULT ),
        focal_length_35mm: if_not( from_value( values.next().unwrap() ), FOCAL_LENGTH_35MM_DEFAULT ),
        camera_model: if_not( from_value( values.next().unwrap() ), CAMERA_MODEL_DEFAULT.to_string() )  
    }
}

/*trait OptNot {
    fn opt_not( self, bad_value: Self ) -> Option<Self>;
}

impl<T: PartialEq> OptNot for T {
    fn opt_not( self, bad_value: T ) -> Option<T> {
        if self != bad_value {
            Some( self ) 
        }
        else {
            None
        }
    }
}*/

fn if_not<T: PartialEq>( val: T, bad_value: T ) -> Option<T> {
    if val != bad_value {
        Some( val ) 
    }
    else {
        None
    }
}

const JPEG_STR : &'static str = "jpg";
const PNG_STR : &'static str = "png";

impl ToValue for ImageType {
    fn to_value(&self) -> Value {
        match self {
            &ImageType::Jpeg => JPEG_STR.to_value(),
            &ImageType::Png => PNG_STR.to_value()
        }
    }
}

impl FromValue for ImageType {
    fn from_value(v: &Value) -> ImageType {
        from_value_opt::<ImageType>( v ).expect( "fail converting ImageType from db value!" )
    }
    fn from_value_opt(v: &Value) -> Option<ImageType> {
        from_value_opt::<String>( v )
            .and_then( |string| match string.as_slice() {
                JPEG_STR => Some( ImageType::Jpeg ),
                PNG_STR => Some( ImageType::Png ),
                _ => None
            })
    }
}

impl Database {
    fn init(&self) -> Result<(), String> {
        let result = self.pool
            .query( "set names utf8;" );
        match result {
            Ok(_)=>Ok( () ),
            Err( e ) => Err( format!( "Database::init failed: {}", e ) )
        }
    }
}

pub fn create_db_connection( 
    db_name: String, 
    user: String, 
    pass: String,
    min_connections: uint,
    max_connections: uint
) -> Result<Database, String> {
    let opts = MyOpts{
        db_name: Some( db_name ),
        user: Some( user ), 
        pass: Some( pass ),
        ..Default::default()
    };

    let pool = MyPool::new_manual( min_connections, max_connections, opts );
    match pool {
        Ok( pool ) => {
            let db = Database{ pool: pool };
            match db.init() { // тут я что-то подупрел с fn map, скопилировалось только с match
                Ok(_) => Ok( db ),
                Err( e ) => Err( e )
            }
        },
        Err( e ) => Err( format!( "Connection to db failed: {}", e ) )
    }
}

impl Middleware for Database {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.map.insert( self.clone() );
        Ok( Continue )
    }
}

pub trait Databaseable {
    fn db(&self) -> DatabaseConn;
}

impl<'a, 'b> Databaseable for Request<'a, 'b> {
    fn db(&self) -> DatabaseConn {
        DatabaseConn{ connection: self.map.get::<Database>().unwrap().pool.get_conn() }
    }
}