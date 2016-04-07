use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{ from_row, from_value, ToValue, FromValue, Value, ConvIr };
use types::{
    Id,
    PhotoInfo,
    ShortPhotoInfo,
    ImageType,
    ShortInfo,
    CommonResult,
    EmptyResult,
    CommonError
};
use database::Database;
use std::fmt::Display;
use std::str;

pub trait DbPhotos {
    /// добавление фотографии в галлерею пользователя
    fn add_photo( &mut self, user_id: Id, info: &PhotoInfo ) -> CommonResult<Id>;
    /// получение информации о фото в галлерее
    fn get_gallery_photo_info( &mut self, reader_id: Id, photo_id: Id ) -> CommonResult<Option<PhotoInfo>>;
    /// получение информации о фото в галлерее
    fn get_publication_photo_info( &mut self, reader_id: Id, photo_id: Id ) -> CommonResult<Option<PhotoInfo>>;
    fn get_short_photo_info( &mut self, photo_id: Id ) -> CommonResult<Option<ShortPhotoInfo>>;
    ///возвращает список описаний фоточек
    fn get_photo_infos( &mut self, reader_id: Id, owner_id: Id, offset: u32, count: u32 ) -> CommonResult<Vec<PhotoInfo>>;
    ///вычисляет кол-во фоток пользователя за опеределнный период
    fn get_photo_infos_count( &mut self, owner_id: Id ) -> CommonResult<u32>;
    ///переименование фотографии
    fn rename_photo( &mut self, photo_id: Id, newname: &str ) -> CommonResult<()>;
    ///вычисляет кол-во неопубликованных фотографий пользователя
    fn get_unpublished_photos_count( &mut self, owner_id: Id ) -> CommonResult<u32>;
    ///возвращает список описаний фоточек которые еще не были опубликованы
    fn get_unpublished_photo_infos( &mut self, reader_id: Id, owner_id: Id, offset: u32, count: u32 ) -> CommonResult<Vec<PhotoInfo>>;
    /// возвращает описания фотографий от опеределенной публикации
    fn get_publication_photo_infos( &mut self, reader_id: Id, scheduled_id: Id ) -> CommonResult<Vec<PhotoInfo>>;
    /// увеличивает счётчик комментариев у фотографии
    fn increment_photo_comments_count( &mut self, photo_id: Id ) -> EmptyResult;
    /// возвращает последнее загруженное фото в галлерею пользователя
    fn get_last_upload_gallery_photo( &mut self, user_id: Id ) -> CommonResult<Option<Id>>;
    /// устанваливает предыдуюшую фотогафию в последовательности фотографий в галлерее
    fn set_prev_in_gallery( &mut self, photo_id: Id, prev_photo_id: Id ) -> EmptyResult;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `images` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `owner_id` bigint(20) unsigned DEFAULT '0',
            `upload_time` bigint(20) NOT NULL DEFAULT '0',
            `type` enum( 'jpg', 'png' ) NOT NULL DEFAULT 'jpg',
            `width` int(4) unsigned DEFAULT '0',
            `height` int(4) unsigned DEFAULT '0',
            `name` varchar(64) NOT NULL DEFAULT '',
            `comments_count` int(11) NOT NULL DEFAULT '0',
            `iso` int(11) unsigned DEFAULT '0',
            `shutter_speed` int(11) DEFAULT '0',
            `aperture` decimal(8,4) NOT NULL DEFAULT '0',
            `focal_length` int(4) unsigned DEFAULT '0',
            `focal_length_35mm` int(4) unsigned DEFAULT '0',
            `camera_model` varchar(64) NOT NULL DEFAULT '',
            `next_gallery_id` bigint(20) NOT NULL DEFAULT '0',
            `prev_gallery_id` bigint(20) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` ),
            KEY `owner_image` ( `owner_id`, `upload_time` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::photos::create_tables"
    )
}

const ISO_DEFAULT: u32 = 0;
const SHUTTER_SPEED_DEFAULT: i32 = 0;
const APERTURE_DEFAULT: f32 = 0.;
const FOCAL_LENGTH_DEFAULT: u16 = 0;
const FOCAL_LENGTH_35MM_DEFAULT: u16 = 0;
const CAMERA_MODEL_DEFAULT: &'static str = "";


impl DbPhotos for PooledConn {
    /// добавление фотографии в галлерею пользователя
    fn add_photo( &mut self, user_id: Id, info: &PhotoInfo ) -> CommonResult<Id> {
        add_photo_impl( self, user_id, info )
            .map_err( |e| fn_failed( "add_photo", e ) )
    }

    /// получение информации о фото
    fn get_gallery_photo_info( &mut self, reader_id: Id, photo_id: Id ) -> CommonResult<Option<PhotoInfo>> {
        get_gallery_photo_info_impl( self, reader_id, photo_id )
            .map_err( |e| fn_failed( "get_gallery_photo_info", e ) )
    }
    fn get_publication_photo_info( &mut self, reader_id: Id, photo_id: Id ) -> CommonResult<Option<PhotoInfo>> {
        get_publication_photo_info_impl( self, reader_id, photo_id )
            .map_err( |e| fn_failed( "get_publication_photo_info", e ) )
    }
    fn get_short_photo_info( &mut self, photo_id: Id ) -> CommonResult<Option<ShortPhotoInfo>> {
        get_short_photo_info_impl( self, photo_id )
            .map_err( |e| fn_failed( "get_short_photo_info", e ) )
    }

    ///возвращает список описаний фоточек
    fn get_photo_infos( &mut self, reader_id: Id, owner_id: Id, offset: u32, count: u32 ) -> CommonResult<Vec<PhotoInfo>> {
        get_photo_infos_impl( self, reader_id, owner_id, offset, count )
            .map_err( |e| fn_failed( "get_photo_infos", e ) )
    }

    ///вычисляет кол-во фоток пользователя за опеределнный период
    fn get_photo_infos_count( &mut self, owner_id: Id ) -> CommonResult<u32> {
        get_photo_infos_count_impl( self, owner_id )
            .map_err( |e| fn_failed( "get_photo_infos_count", e ) )
    }

    ///переименование фотографии
    fn rename_photo( &mut self, photo_id: Id, newname: &str ) -> CommonResult<()> {
        rename_photo_impl( self, photo_id, newname )
            .map_err( |e| fn_failed( "rename_photo", e ) )
    }

    ///вычисляет кол-во неопубликованных фотографий пользователя
    fn get_unpublished_photos_count( &mut self, owner_id: Id ) -> CommonResult<u32> {
        get_unpublished_photos_count_impl( self, owner_id )
            .map_err( |e| fn_failed( "get_unpublished_photos_count", e ) )
    }

    ///возвращает список описаний фоточек которые еще не были опубликованы
    fn get_unpublished_photo_infos( &mut self, reader_id: Id, owner_id: Id, offset: u32, count: u32 ) -> CommonResult<Vec<PhotoInfo>> {
        get_unpublished_photo_infos_impl( self, reader_id, owner_id, offset, count )
            .map_err( |e| fn_failed( "get_unpublished_photo_infos", e ) )
    }

    /// возвращает описания фотографий от опеределенной публикации
    fn get_publication_photo_infos( &mut self, reader_id: Id, scheduled_id: Id ) -> CommonResult<Vec<PhotoInfo>> {
        get_publication_photo_infos_impl( self, reader_id, scheduled_id )
            .map_err( |e| fn_failed( "get_publication_photo_infos", e ) )
    }

    /// увеличивает счётчик комментариев у фотографии
    fn increment_photo_comments_count( &mut self, photo_id: Id ) -> EmptyResult {
        increment_comments_count_impl( self, photo_id )
            .map_err( |e| fn_failed( "increment_photo_comments_count", e ) )
    }

    /// возвращает последнее загруженное фото в галлерею пользователя
    fn get_last_upload_gallery_photo( &mut self, user_id: Id ) -> CommonResult<Option<Id>> {
        get_last_upload_gallery_photo_impl( self, user_id )
            .map_err( |e| fn_failed( "get_last_upload_gallery_photo", e ) )
    }

    /// устанваливает предыдуюшую фотогафию в последовательности фотографий в галлерее
    fn set_prev_in_gallery( &mut self, photo_id: Id, prev_photo_id: Id ) -> EmptyResult {
        set_prev_in_gallery_impl( self, photo_id, prev_photo_id )
            .map_err( |e| fn_failed( "set_prev_in_gallery", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbPhotos func '{}' failed: {}", fn_name, e ) )
}


fn add_photo_impl( conn: &mut PooledConn, user_id: Id, info: &PhotoInfo ) -> mysql::Result<Id> {
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
        camera_model,
        next_gallery_id,
        prev_gallery_id )
        values( ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ? )" )
    );
    let camera_model: &str = match info.camera_model {
        Some( ref cam ) => cam,
        None => CAMERA_MODEL_DEFAULT
    };
    let params: &[ &ToValue ] =  &[
        &user_id,
        &info.upload_time,
        &info.image_type,
        &info.width,
        &info.height,
        &info.name,
        &info.iso.unwrap_or( ISO_DEFAULT ),
        &info.shutter_speed.unwrap_or( SHUTTER_SPEED_DEFAULT ),
        &info.aperture.unwrap_or( APERTURE_DEFAULT ),
        &info.focal_length.unwrap_or( FOCAL_LENGTH_DEFAULT ),
        &info.focal_length_35mm.unwrap_or( FOCAL_LENGTH_35MM_DEFAULT ),
        &camera_model,
        &info.next.unwrap_or( 0 ),
        &info.prev.unwrap_or( 0 )
    ];
    let sql_result = try!( stmt.execute( params ) );
    Ok( sql_result.last_insert_id() )
}

fn get_photo_info_impl( conn: &mut PooledConn, query: &str, reader_id: Id, photo_id: Id ) -> mysql::Result<Option< PhotoInfo>> {
    let mut stmt = try!( conn.prepare( &query ) );
    let mut sql_result = try!( stmt.execute( (reader_id, photo_id) ) );
    match sql_result.next() {
        None => Ok( None ),
        Some( sql_row ) => {
            let row_data = try!( sql_row );
            let mut values = row_data.unwrap().into_iter();
            Ok( Some( read_photo_info( &mut values ) ) )
        }
    }
}

fn get_gallery_photo_info_impl( conn: &mut PooledConn, reader_id: Id, photo_id: Id ) -> mysql::Result<Option< PhotoInfo>> {
    let query = format!(
        "SELECT {}
            i.next_gallery_id,
            i.prev_gallery_id
        FROM images AS i
        LEFT JOIN users AS u ON ( u.id = i.owner_id )
        WHERE u.id IS NOT NULL AND i.id = ?",
        PHOTO_INFO_FIELDS
    );
    get_photo_info_impl( conn, &query, reader_id, photo_id )
}

fn get_publication_photo_info_impl( conn: &mut PooledConn, reader_id: Id, photo_id: Id ) -> mysql::Result<Option< PhotoInfo>> {
    let query = format!(
        "SELECT {}
            p.next,
            p.prev
        FROM publication AS p
        LEFT JOIN images AS i ON( i.id = p.photo_id )
        LEFT JOIN users AS u ON ( u.id = i.owner_id )
        WHERE u.id IS NOT NULL AND i.id = ?",
        PHOTO_INFO_FIELDS
    );
    get_photo_info_impl( conn, &query, reader_id, photo_id )
}

fn get_short_photo_info_impl( conn: &mut PooledConn, photo_id: Id ) -> mysql::Result<Option<ShortPhotoInfo>> {
    let query = format!(
        "SELECT i.id,
                i.upload_time,
                i.type,
                i.owner_id,
                u.login
        FROM images AS i
        LEFT JOIN users AS u ON ( u.id = i.owner_id )
        WHERE u.id IS NOT NULL AND i.id = ?"
    );
    let mut stmt = try!( conn.prepare( &query ) );
    let mut sql_result = try!( stmt.execute( (photo_id,) ) );
    let result = match sql_result.next() {
        Some( sql_row ) => {
            let row_data = try!( sql_row );
            let (id, time, img_type, owner_id, owner_name) = from_row( row_data );
            let info = ShortPhotoInfo {
                id: id,
                upload_time: time,
                image_type: img_type,
                owner: ShortInfo {
                    id: owner_id,
                    name: owner_name
                }
            };
            Some( info )
        },
        None => None
    };
    Ok( result )
}

fn get_photo_infos_impl(
    conn: &mut PooledConn,
    reader_id: Id,
    owner_id: Id,
    offset: u32,
    count: u32
) -> mysql::Result<Vec<PhotoInfo>>
{
    let query = format!(
        "SELECT {fields}
            i.next_gallery_id,
            i.prev_gallery_id
        FROM images AS i
        LEFT JOIN users AS u ON ( u.id = i.owner_id )
        WHERE i.owner_id = ?
        ORDER BY i.upload_time DESC
        LIMIT ?
        OFFSET ?
        ;",
        fields = PHOTO_INFO_FIELDS );
    let params: &[ &ToValue ] = &[ &reader_id, &owner_id, &count, &offset ];
    get_photo_infos_general( conn, &query, params )
}

fn get_unpublished_photo_infos_impl(
    conn: &mut PooledConn,
    reader_id: Id,
    owner_id: Id,
    offset: u32,
    count: u32
) -> mysql::Result<Vec<PhotoInfo>>
{
    let query = format!(
        "SELECT {fields}
         i.next_gallery_id,
         i.prev_gallery_id
        FROM images AS i
        LEFT JOIN users AS u ON ( u.id = i.owner_id )
        LEFT JOIN publication AS p ON ( p.photo_id = i.id )
        WHERE i.owner_id = ?
          AND p.id is NULL
        ORDER BY i.upload_time DESC
        LIMIT ?
        OFFSET ?
        ;",
        fields = PHOTO_INFO_FIELDS );
    let params: &[ &ToValue ] = &[ &reader_id, &owner_id, &count, &offset ];
    get_photo_infos_general( conn, &query, params )
}

fn get_publication_photo_infos_impl( conn: &mut PooledConn,
                                     reader_id: Id,
                                     scheduled_id: Id ) -> mysql::Result<Vec<PhotoInfo>>
{
    let query = format!(
       "SELECT {fields}
            p.next,
            p.prev
        FROM publication AS p
        LEFT JOIN images AS i ON( i.id = p.photo_id )
        LEFT JOIN users AS u ON ( u.id = p.user_id )
        WHERE p.scheduled_id=?
        ORDER BY p.id ASC",
        fields = PHOTO_INFO_FIELDS
    );
    let params: &[ &ToValue ] = &[ &reader_id, &scheduled_id ];
    get_photo_infos_general( conn, &query, params )
}

fn get_photo_infos_general( conn: &mut PooledConn,
                            query: &str,
                            params: &[ &ToValue ] ) -> mysql::Result<Vec<PhotoInfo>>
{
    let mut stmt = try!( conn.prepare( &query ) );
    let result = try!( stmt.execute( params ) );
    //что-то с преобразованием на лету через собственный итертор я подупрел =(, пришлось тупо собирать в новый массив
    let photos : Vec<_> = result.filter_map( |sql_row|
        sql_row.ok().map( |sql_values| {
            let values = sql_values.unwrap().into_iter();
            read_photo_info( values )
        })
    ).collect();
    Ok( photos )
}

fn get_photo_infos_count_impl( conn: &mut PooledConn, owner_id: Id ) -> mysql::Result<u32> {
    let mut stmt = try!( conn.prepare( "
        SELECT COUNT(id)
        FROM
            images
        WHERE
            owner_id = ?
    " ) );
    let params: &[ &ToValue ] = &[ &owner_id ];
    let mut result = try!( stmt.execute( params ) );
    let sql_row = try!( result.next().unwrap() );
    let (count,) = from_row( sql_row );
    Ok( count )
}

fn get_unpublished_photos_count_impl( conn: &mut PooledConn, owner_id: Id ) -> mysql::Result<u32> {
    let mut stmt = try!( conn.prepare( "
        SELECT COUNT(i.id)
        FROM images AS i
        LEFT JOIN publication AS p ON ( p.photo_id = i.id )
        WHERE
            i.owner_id = ? AND
            p.id is NULL
    " ) );
    let params: &[ &ToValue ] = &[ &owner_id ];
    let mut result = try!( stmt.execute( params ) );
    let sql_row = try!( result.next().unwrap() );
    let (count,) = from_row( sql_row );
    Ok( count )
}

fn rename_photo_impl( conn: &mut PooledConn, photo_id: Id, newname: &str ) -> mysql::Result<()> {
    let newname = newname.to_string();
    let mut stmt = try!( conn.prepare( "UPDATE images SET name=? WHERE id=?" ) );
    let params: &[ &ToValue ] = &[ &newname, &photo_id ];
    let _ = try!( stmt.execute( params ) );
    Ok( () )
}

fn increment_comments_count_impl( conn: &mut PooledConn, photo_id: Id ) -> mysql::Result<()>  {
    try!( conn.prep_exec( "UPDATE images SET comments_count=comments_count+1 WHERE id=?",
                           (photo_id,) ) );
    Ok( () )
}

fn get_last_upload_gallery_photo_impl( conn: &mut PooledConn,
                                       user_id: Id ) -> mysql::Result<Option<Id>> {
    let mut stmt = try!( conn.prepare( "
         SELECT id
         FROM images
         WHERE owner_id=?
         ORDER BY upload_time DESC
         LIMIT 1
    "));
    let mut sql_result = try!( stmt.execute( (user_id,) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            let (id,) = from_row( row );
            if id != 0 {
                Some( id )
            }
            else {
                None
            }
        },
        None => None
    };
    Ok( result )
}

fn set_prev_in_gallery_impl( conn: &mut PooledConn,
                             photo_id: Id,
                             prev_photo_id: Id ) -> mysql::Result<()>
{
    try!( conn.prep_exec( "UPDATE images SET prev_gallery_id=? WHERE id=?",
                           (prev_photo_id, photo_id) ) );
    Ok( () )
}

const PHOTO_INFO_FIELDS: &'static str = "
    i.owner_id,
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
    i.camera_model,
    i.comments_count,
    ( SELECT COUNT( cc.id )
      FROM comments AS cc
      LEFT JOIN visited AS vv
           ON ( vv.content_type='comment'
                AND vv.content_id = cc.id
                AND vv.user_id=? )
      WHERE cc.comment_for='photo' AND cc.for_id=i.id AND vv.id is NULL ),";

fn read_photo_info<I: Iterator<Item = Value>>( mut values: I ) -> PhotoInfo
{
    let mut next = || values.next().expect("DbPhotos invalid columns count on read PhotoInfo");
    let neighbour = |value| {
        match from_value( value ) {
            0 => None,
            v@_ => Some( v )
        }
    };

    PhotoInfo {
        owner: ShortInfo {
            id: from_value( next() ),
            name: from_value( next() ),
        },
        id: from_value( next() ),
        upload_time: from_value( next() ),
        image_type: from_value( next() ),
        width: from_value( next() ),
        height: from_value( next() ),
        name: from_value( next() ),
        iso: if_not( from_value( next() ), ISO_DEFAULT ),
        shutter_speed: if_not( from_value( next() ), SHUTTER_SPEED_DEFAULT ),
        aperture: if_not( from_value( next() ), APERTURE_DEFAULT ),
        focal_length: if_not( from_value( next() ), FOCAL_LENGTH_DEFAULT ),
        focal_length_35mm: if_not( from_value( next() ), FOCAL_LENGTH_35MM_DEFAULT ),
        camera_model: if_not( from_value( next() ), CAMERA_MODEL_DEFAULT.to_string() ),
        comments_count: from_value( next() ),
        unreaded_comments: from_value( next() ),
        next: neighbour( next() ),
        prev: neighbour( next() )
    }
}

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
            &ImageType::Jpeg => Value::Bytes( JPEG_STR.bytes().collect() ),
            &ImageType::Png => Value::Bytes( PNG_STR.bytes().collect() )
        }
    }
}

pub struct ImageTypeIr
{
    val: ImageType,
    bytes: Vec<u8>
}

impl ConvIr<ImageType> for ImageTypeIr
{
    fn new(v: Value) -> mysql::Result<ImageTypeIr> {
        match v {
            Value::Bytes( bytes ) => {
                let value = match str::from_utf8( &bytes ) {
                    Ok( s ) => match s {
                        JPEG_STR => Some( ImageType::Jpeg ),
                        PNG_STR => Some( ImageType::Png ),
                        _ => None
                    },
                    _ => None
                };
                match value {
                    Some( t ) => Ok( ImageTypeIr{ val: t, bytes: bytes } ),
                    None => Err( mysql::Error::FromValueError( Value::Bytes( bytes ) ) )
                }
            },
            _ => Err(mysql::Error::FromValueError(v))
        }
    }
    fn commit(self) -> ImageType {
        self.val
    }
    fn rollback(self) -> Value {
        Value::Bytes( self.bytes )
    }
}

impl FromValue for ImageType {
    type Intermediate = ImageTypeIr;
}
