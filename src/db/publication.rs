use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{ from_row, ToValue };
use types::{ Id, EmptyResult, CommonResult, CommonError };
use std::fmt::Display;
use database::Database;
use authentication::User;

pub trait DbPublication {
    /// публикует фото
    fn public_photo( &mut self, scheduled: Id, user: Id, photo: Id, visible: bool, time: u64 ) -> EmptyResult;
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
            PRIMARY KEY ( `id` ),
            KEY `publication_idx` ( `scheduled_id`, `visible` ) USING BTREE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::publications::create_tables"
    )
}

impl DbPublication for PooledConn {
    /// публикует фото
    fn public_photo( &mut self, scheduled: Id, user: Id, photo: Id, visible: bool, time: u64 ) -> EmptyResult {
        public_photo_impl( self, scheduled, user, photo, visible, time )
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
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbPublication {} failed: {}", fn_name, e ) )
}

fn public_photo_impl( conn: &mut PooledConn,
                      scheduled: Id,
                      user: Id,
                      photo: Id,
                      visible: bool,
                      time: u64 ) -> mysql::Result<()>
{
    let mut stmt = try!( conn.prepare("
        INSERT INTO publication (
            time,
            scheduled_id,
            user_id,
            photo_id,
            visible
        )
        VALUES( ?, ?, ?, ?, ? )
        ON DUPLICATE KEY UPDATE photo_id=?
    "));
    let params: &[ &ToValue ] = &[
        &time,
        &scheduled,
        &user,
        &photo,
        &visible,
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
            `g`.`user_id`, `u`.`login`, `u`.`mail`
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
        let (id, name, mail) = from_row( row );
        users.push( User {
            id: id,
            name: name,
            mail: mail
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
