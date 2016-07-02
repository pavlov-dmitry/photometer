use mysql;
use mysql::conn::pool::{ Pool };
use database::Database;
use std::fmt::Display;
use mysql::value::{
    from_row
};
use types::{ EmptyResult, CommonResult, CommonError };

pub trait DbMeta {
    /// версия схемы БД
    fn get_db_version( &self ) -> CommonResult<u32>;
    /// установить версию БД ( если её там нет )
    fn set_db_version( &self, ver: u32 ) -> EmptyResult;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `_meta_` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `db_version` bigint(20) NOT NULL DEFAULT '0',
            PRIMARY KEY ( `id` )
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;"
        ,
        "db::meta::create_tables"
    )
}

impl DbMeta for Pool {
    /// версия схемы БД
    fn get_db_version( &self ) -> CommonResult<u32> {
        get_db_version_impl( self )
            .map_err( |e| fn_failed( "get_db_version", e ) )
    }
    /// установить версию БД ( если её там нет )
    fn set_db_version( &self, ver: u32 ) -> EmptyResult {
        set_db_version_impl( self, ver )
            .map_err( |e| fn_failed( "set_db_version", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbMeta {} failed: {}", fn_name, e ) )
}

fn set_db_version_impl( conn: &Pool, ver: u32 ) -> mysql::Result<()> {
    try!( conn.prep_exec(
        "INSERT INTO _meta_ (id, db_version) VALUES (?, ?)
        ON DUPLICATE KEY UPDATE id=id",
        (1, ver)
    ) );
    Ok( () )
}

fn get_db_version_impl( conn: &Pool ) -> mysql::Result<u32> {
    let mut sql_result = try!( conn.prep_exec(
        "SELECT db_version FROM _meta_ WHERE id=?",
        (1,)
    ) );
    let row = sql_result.next().expect( "no version row in db" );
    let row = try!( row );
    let (ver,) = from_row( row );
    Ok( ver )
}
