use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{ from_row, ToValue };
use types::{ Id, EmptyResult, CommonResult, CommonError };
use std::fmt::Display;
use database::Database;
use authentication::User;

pub trait DbPublication {
    /// публикует фото
    fn public_photo( &mut self,
                      scheduled: Id,
                      user: Id,
                      photo: Id,
                      visible: bool,
                      time: u64,
                      prev: Option<Id> ) -> EmptyResult;
    /// открывает на просмотр определнную группу фото
    fn make_publication_visible( &mut self, scheduled: Id ) -> EmptyResult;
    /// кол-во уже опубликованных фото
    fn get_published_photo_count( &mut self, scheduled: Id ) -> CommonResult<u32>;
    /// возвращает идентификаторы пользователей которые не опубликовались
    fn get_unpublished_users( &mut self, scheduled: Id ) -> CommonResult<Vec<User>>;
    /// проверяет на неопубликованность пользователя
    fn is_unpublished_user( &mut self, scheduled: Id, user: Id ) -> CommonResult<bool>;
    /// проверят опубликованно ли это фото или нет
    fn is_photo_published( &mut self, photo: Id ) -> CommonResult<bool>;
    /// возвращает идентификатор фотографии последней в публикации
    fn get_last_pubcation_photo( &mut self, scheduled: Id ) -> CommonResult<Option<Id>>;
    /// устанавливает следующую фотографию в публикации
    fn set_next_publication_photo( &mut self, photo_id: Id, next_id: Id ) -> EmptyResult;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `publication` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `time` bigint(20) NOT NULL DEFAULT '0',
            `scheduled_id` bigint(20) NOT NULL DEFAULT '0',
            `user_id` bigint(20) NOT NULL DEFAULT '0',
            `photo_id` bigint(20) NOT NULL DEFAULT '0',
            `visible` BOOL NOT NULL DEFAULT false,
            `next` bigint(20) NOT NULL DEFAULT '0',
            `prev` bigint(20) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` ),
            KEY `publication_idx` ( `scheduled_id`, `visible` ) USING BTREE,
            KEY `photo_idx` ( `photo_id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::publications::create_tables"
    )
}

impl DbPublication for PooledConn {
    /// публикует фото
    fn public_photo( &mut self,
                      scheduled: Id,
                      user: Id,
                      photo: Id,
                      visible: bool,
                      time: u64,
                      prev: Option<Id> ) -> EmptyResult
    {
        public_photo_impl( self, scheduled, user, photo, visible, time, prev )
            .map_err( |e| fn_failed( "public_photo", e ) )
    }

    /// открывает на просмотр определнную группу фото
    fn make_publication_visible( &mut self, scheduled: Id ) -> EmptyResult {
        make_publication_visible_impl( self, scheduled )
            .map_err( |e| fn_failed( "make_publication_visible", e ) )
    }

    /// кол-во уже опубликованных фото
    fn get_published_photo_count( &mut self, scheduled: Id ) -> CommonResult<u32> {
        get_published_photo_count_impl( self, scheduled )
            .map_err( |e| fn_failed( "get_publicated_photo_count", e ) )
    }

    /// возвращает идентификаторы пользователей которые не проголосовали
    fn get_unpublished_users( &mut self, scheduled: Id ) -> CommonResult<Vec<User>> {
        get_unpublished_users_impl( self, scheduled )
            .map_err( |e| fn_failed( "get_unpublished_users", e ) )
    }
    /// проверяет на неопубликованность пользователя
    fn is_unpublished_user( &mut self, scheduled: Id, user: Id ) -> CommonResult<bool> {
        is_unpublished_user_impl( self, scheduled, user )
            .map_err( |e| fn_failed( "is_unpublished_user", e ) )
    }
    /// проверят опубликованно ли это фото или нет
    fn is_photo_published( &mut self, photo: Id ) -> CommonResult<bool> {
        is_photo_published_impl( self, photo )
            .map_err( |e| fn_failed( "is_photo_published", e ) )
    }
    /// возвращает идентификатор фотографии последней в публикации
    fn get_last_pubcation_photo( &mut self, scheduled: Id ) -> CommonResult<Option<Id>> {
        get_last_pubcation_photo_impl( self, scheduled )
            .map_err( |e| fn_failed( "get_last_pubcation_photo", e ) )
    }
    /// устанавливает следующую фотографию в публикации
    fn set_next_publication_photo( &mut self, photo_id: Id, next_id: Id ) -> EmptyResult {
        set_next_publication_photo_impl( self, photo_id, next_id )
            .map_err( |e| fn_failed( "set_next_publication_photo", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbPublication {} failed: {}", fn_name, e ) )
}

fn public_photo_impl( conn: &mut PooledConn,
                      scheduled: Id,
                      user: Id,
                      photo: Id,
                      visible: bool,
                      time: u64,
                      prev: Option<Id> ) -> mysql::Result<()>
{
    let mut stmt = try!( conn.prepare("
        INSERT INTO publication (
            time,
            scheduled_id,
            user_id,
            photo_id,
            visible,
            prev
        )
        VALUES( ?, ?, ?, ?, ?, ? )
        ON DUPLICATE KEY UPDATE photo_id=?
    "));
    let prev = prev.unwrap_or( 0 );
    let params: &[ &ToValue ] = &[
        &time,
        &scheduled,
        &user,
        &photo,
        &visible,
        &prev,
        &photo
    ];
    try!( stmt.execute( params ));
    Ok( () )
}

fn make_publication_visible_impl( conn: &mut PooledConn, scheduled: Id ) -> mysql::Result<()> {
    let mut stmt = try!( conn.prepare( "
        UPDATE publication
        SET visible=true
        WHERE scheduled_id = ?
    "));

    let params: &[ &ToValue ] = &[ &scheduled ];
    try!( stmt.execute( params ) );
    Ok( () )
}

fn get_published_photo_count_impl( conn: &mut PooledConn, scheduled: Id ) -> mysql::Result<u32> {
    let mut stmt = try!( conn.prepare( "
SELECT
    COUNT(id)
FROM
    publication
WHERE
    scheduled_id=?" ) );
    let params: &[ &ToValue ] = &[ &scheduled ];
    let mut result = try!( stmt.execute( params ) );
    let row = try!( result.next().unwrap() );
    let (count,) = from_row( row );
    Ok( count )
}

fn get_unpublished_users_impl( conn: &mut PooledConn, scheduled: Id ) -> mysql::Result<Vec<User>> {
    let mut stmt = try!( conn.prepare(
        "SELECT
            `g`.`user_id`, `u`.`login`, `u`.`mail`, `u`.`join_time`
        FROM
            `group_members` AS `g`
        LEFT JOIN
            `users` AS `u` ON ( `u`.`id` = `g`.`user_id` )
        LEFT JOIN
            `publication` AS `p` ON ( `p`.`user_id` = `u`.`id` AND `p`.`scheduled_id` = ? )
        WHERE
            `p`.`id` IS NULL
    "));
    let params: &[ &ToValue ] = &[ &scheduled ];
    let result = try!( stmt.execute( params ) );
    let mut users = Vec::new();
    for row in result {
        let row = try!( row );
        let (id, name, mail, time) = from_row( row );
        users.push( User {
            id: id,
            name: name,
            mail: mail,
            join_time: time
        });
    }
    Ok( users )
}

fn is_unpublished_user_impl( conn: &mut PooledConn, scheduled: Id, user: Id ) -> mysql::Result<bool> {
    let mut stmt = try!( conn.prepare(
        "SELECT
            COUNT( `id` )
        FROM
            `publication`
        WHERE
            `scheduled_id` = ? AND
            `user_id` = ?
        "
    ));
    let params: &[ &ToValue ] = &[ &scheduled, &user ];
    let mut result = try!( stmt.execute( params ) );
    let row = try!( result.next().unwrap() );
    let (count,): (i32,) = from_row( row );
    Ok( count == 0 )
}

fn is_photo_published_impl( conn: &mut PooledConn, photo: Id ) -> mysql::Result<bool> {
    let mut stmt = try!( conn.prepare(
        "SELECT
             `id`
         FROM `publication`
         WHERE `photo_id`=?"
    ));
    let result = try!( stmt.execute( (photo,) ) );
    let count = result.count();
    Ok( count != 0 )
}

fn get_last_pubcation_photo_impl( conn: &mut PooledConn, scheduled: Id ) -> mysql::Result<Option<Id>> {
    let mut stmt = try!( conn.prepare(
        "SELECT `photo_id`
         FROM `publication`
         WHERE `scheduled_id`=?
         ORDER BY `time` DESC
         LIMIT 1"
    ));
    let mut sql_result = try!( stmt.execute( (scheduled,) ) );
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

fn set_next_publication_photo_impl( conn: &mut PooledConn, photo_id: Id, next_id: Id ) -> mysql::Result<()> {
    try!( conn.prep_exec( "UPDATE publication SET next=? WHERE photo_id=?",
                          (next_id, photo_id) ) );
    Ok( () )
}
