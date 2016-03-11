use mysql;
use mysql::conn::pool::{ PooledConn };
use mysql::value::{ Value, ToValue, from_row };
use types::{ Id, ShortInfo, CommonResult, EmptyResult, CommonError };
use std::fmt::Display;
use database::Database;

use authentication::User;

pub trait DbUsers {
    /// выбирает id пользователя по имени и паролю
    fn get_user( &mut self, name: &str, pass: &str ) -> CommonResult<Option<User>>;
    /// добавляет нового пользователя в БД
    fn add_user( &mut self, name: &str, pass: &str, mail: &str, regkey: &str, time: u64 ) -> CommonResult<User>;
    /// активирует пользователя по определённому регистрационному ключу
    fn activate_user( &mut self, regkey: &str ) -> CommonResult<Option<User>>;
    /// проверяет наличие имени в БД
    fn user_exists(&mut self, name: &str, mail: &str) -> CommonResult<bool>;
    fn user_id_exists(&mut self, id: Id ) -> CommonResult<bool>;
    /// возвращает пользователя по Id
    fn user_by_id( &mut self, id: Id ) -> CommonResult<Option<User>>;
    /// возвращает пользователей по Id
    fn users_by_id( &mut self, ids: &[Id] ) -> CommonResult<Vec<User>>;
    /// возвращает пользователя по имени
    // fn user_by_name( &mut self, name: &str ) -> CommonResult<Option<User>>;
    /// возвращает имена полльзователей имена которых похожи на шаблон
    fn get_users_like( &mut self, pattern: &str, offset: u32, count: u32 ) -> CommonResult<Vec<ShortInfo>>;
    /// возвращает регистрационный ключ пользователя
    fn get_reg_key( &mut self, name: &str ) -> CommonResult<Option<String>>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(
        "CREATE TABLE IF NOT EXISTS `users` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `join_time` bigint(20) NOT NULL DEFAULT '0',
            `login` varchar(16) NOT NULL DEFAULT '',
            `password` varchar(32) NOT NULL DEFAULT '',
            `activated` BOOL NOT NULL DEFAULT false,
            `mail` varchar(256) NOT NULL DEFAULT '',
            `regkey` varchar(256) NOT NULL DEFAULT '',
            PRIMARY KEY (`id`),
            UNIQUE KEY `login_idx` (`login`),
            UNIQUE KEY `regkey_idx` (`regkey`),
            UNIQUE KEY `email_idx` (`email`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::users::create_tables"
    )
}

impl DbUsers for PooledConn {
    /// выбирает id пользователя по имени и паролю
    fn get_user( &mut self, name: &str, pass: &str ) -> CommonResult<Option<User>> {
        get_user_impl( self, name, pass )
            .map_err( |e| fn_failed( "get_user", e ) )
    }

    /// добавляет нового пользователя в БД
    fn add_user( &mut self, name: &str, pass: &str, mail: &str, regkey: &str, time: u64 ) -> CommonResult<User> {
        add_user_impl( self, name, pass, mail, regkey, time )
            .map_err( |e| fn_failed( "add_user", e ) )
    }
    /// активирует пользователя по определённому регистрационному ключу
    fn activate_user( &mut self, regkey: &str ) -> CommonResult<Option<User>> {
        activate_user_impl( self, regkey )
            .map_err( |e| fn_failed( "activate_user", e ) )
    }
    /// проверяет наличие имени в БД
    fn user_exists(&mut self, name: &str, mail: &str) -> CommonResult<bool> {
        user_exists_impl( self, name, mail )
            .map_err( |e| fn_failed( "user_exists", e ) )
    }
    /// проверяет наличие имени в БД
    fn user_id_exists(&mut self, id: Id ) -> CommonResult<bool> {
        user_id_exists_impl( self, id )
            .map_err( |e| fn_failed( "user_id_exists", e ) )
    }
    /// возвращает пользователя по Id
    fn user_by_id( &mut self, id: Id ) -> CommonResult<Option<User>> {
        user_by_id_impl( self, id )
            .map_err( |e| fn_failed( "user_by_id", e ) )
    }
    /// возвращает пользователя по Id
    fn users_by_id( &mut self, ids: &[Id] ) -> CommonResult<Vec<User>> {
        users_by_id_impl( self, ids )
            .map_err( |e| fn_failed( "users_by_id", e ) )
    }
    /// возвращает пользователя по имени
    // fn user_by_name( &mut self, name: &str ) -> CommonResult<Option<User>> {
    //     user_by_name_impl( self, name )
    //         .map_err( |e| fn_failed( "user_by_name", e ) )
    // }
    /// возвращает имена полльзователей имена которых похожи на шаблон
    fn get_users_like( &mut self, pattern: &str, offset: u32, count: u32 ) -> CommonResult<Vec<ShortInfo>> {
        get_users_like_impl( self, pattern, offset, count )
            .map_err( |e| fn_failed( "get_users_like", e ) )
    }
    /// возвращает регистрационный ключ пользователя
    fn get_reg_key( &mut self, name: &str ) -> CommonResult<Option<String>> {
        get_reg_key_impl( self, name )
            .map_err( |e| fn_failed( "get_reg_key", e ) )
    }
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> CommonError {
    CommonError( format!( "DbUsers func '{}' failed: {}", fn_name, e ) )
}

fn get_user_impl( conn: &mut PooledConn, name: &str, pass: &str ) -> mysql::Result<Option<User>> {
    let name = name.to_string(); // помогает убрать internal compiler error
    let pass = pass.to_string();
    let mut stmt = try!( conn.prepare(
        "SELECT
            `login`,
            `id`,
            `mail`,
            `join_time`
        FROM `users`
        WHERE
            `login`=? AND
            `password`=? AND
            `activated`=true"
    ) );
    let params: &[&ToValue] = &[ &name, &pass ];
    let mut sql_result = try!( stmt.execute( params ) );
    match sql_result.next() {
        None => Ok( None ),
        Some( row ) => {
            let row = try!( row );
            let (name, id, mail, join_time) = from_row( row );
            let user = User {
                name: name,
                id: id,
                mail: mail,
                join_time: join_time
            };
            Ok( Some( user ) )
        }
    }
}

fn add_user_impl( conn: &mut PooledConn, name: &str, pass: &str, mail: &str, regkey: &str, time: u64 ) -> mysql::Result<User> {
    let name = name.to_string();
    let pass = pass.to_string();
    let mut stmt = try!( conn.prepare(
        "INSERT INTO users (
            login,
            password,
            mail,
            regkey,
            join_time
        ) VALUES(?, ?, ?, ?, ?);"
    ) );
    let params: &[ &ToValue ] = &[ &name, &pass, &mail, &regkey, &time ];
    let result = try!( stmt.execute( params ) );
    Ok( User {
        name: name.to_string(),
        id: result.last_insert_id(),
        mail: mail.to_string(),
        join_time: time
    } )
}

fn activate_user_impl( conn: &mut PooledConn, regkey: &str ) -> mysql::Result<Option<User>> {
    let maybe_user = {
        let mut stmt = try!( conn.prepare(
            "SELECT
                `id`,
                `login`,
                `mail`,
                `join_time`
            FROM `users`
            WHERE
                `regkey`=? AND
                `activated`=false"
        ) );
        let mut result = try!( stmt.execute( (regkey,) ) );
        match result.next()  {
            Some( row ) => {
                let row = try!( row );
                let (id, name, mail, time) = from_row( row );
                Some( User {
                    id: id,
                    name: name,
                    mail: mail,
                    join_time: time
                } )
            }
            None => None
        }
    };
    if let Some( ref user ) = maybe_user {
        let mut stmt = try!( conn.prepare(
            "UPDATE users SET activated=true WHERE id=?"
        ) );
        try!( stmt.execute( (user.id,) ) );
    }
    Ok( maybe_user )
}

fn user_exists_impl( conn: &mut PooledConn, name: &str, mail: &str ) -> mysql::Result<bool> {
    let name = name.to_string();
    let mut stmt = try!( conn.prepare( "select id from users where login=? OR mail=?" ) );
    let params: &[ &ToValue ] = &[ &name, &mail ];
    let sql_result = try!( stmt.execute( params ) );
    Ok( sql_result.count() != 0 )
}

fn user_id_exists_impl( conn: &mut PooledConn, id: Id  ) -> mysql::Result<bool> {
    let mut stmt = try!( conn.prepare( "select id from users where id=? AND activated=true" ) );
    let sql_result = try!( stmt.execute( (id,) ) );
    Ok( sql_result.count() == 1 )
}

fn user_by_id_impl( conn: &mut PooledConn, id: Id ) -> mysql::Result<Option<User>> {
    let mut stmt = try!( conn.prepare(
        "SELECT login,
                mail,
                join_time
         FROM users
         WHERE id=?
           AND activated=true"
    ) );
    let mut sql_result = try!( stmt.execute( (id,) ) );
    match sql_result.next() {
        None => Ok( None ),
        Some( row ) => {
            let row = try!( row );
            let (name, mail, time) = from_row( row );
            let user = User {
                name: name,
                id: id,
                mail: mail,
                join_time: time
            };
            Ok( Some( user ) )
        }
    }
}

fn users_by_id_impl( conn: &mut PooledConn, ids: &[Id] ) -> mysql::Result<Vec<User>> {
    let mut query = format!(
        "SELECT id,
                login,
                mail,
                join_time
         FROM users
         WHERE id IN ( ? "
    );
    for _ in 1 .. ids.len() {
        query.push_str( ", ?" );
    }
    query.push_str( " ) AND activated=true" );

    let mut stmt = try!( conn.prepare( &query ) );
    let mut values: Vec<Value> = Vec::new();
    for id in ids.iter() {
        values.push( id.to_value() );
    }

    let mut users = Vec::new();
    let sql_result = try!( stmt.execute( values ) );
    for row in sql_result {
        let row = try!( row );
        let (id, name, mail, time) = from_row( row );
        let user = User {
            id: id,
            name: name,
            mail: mail,
            join_time: time
        };
        users.push( user );
    }
    Ok( users )
}

// fn user_by_name_impl( conn: &mut PooledConn, name: &str ) -> mysql::Result<Option<User>> {
//     let mut stmt = try!( conn.prepare(
//         "SELECT id,
//                 mail,
//                 join_time
//          FROM users
//          WHERE login=?
//            AND activated=true"
//     ) );
//     let mut sql_result = try!( stmt.execute( (name,) ) );
//     match sql_result.next() {
//         None => Ok( None ),
//         Some( row ) => {
//             let row = try!( row );
//             let (id, mail, time) = from_row( row );
//             let user = User {
//                 name: String::from( name ),
//                 id: id,
//                 mail: mail,
//                 join_time: time
//             };
//             Ok( Some( user ) )
//         }
//     }
// }

fn get_users_like_impl( conn: &mut PooledConn,
                        pattern: &str,
                        offset: u32,
                        count: u32 ) -> mysql::Result<Vec<ShortInfo>>
{
    let mut stmt = try!( conn.prepare( "
        SELECT
            `id`,
            `login`
        FROM
            `users`
        WHERE
            `login` LIKE ? AND
            `activated`=true
        ORDER BY
            `login` ASC
        LIMIT ?
        OFFSET ?
    "));
    let sql_result = try!( stmt.execute( (pattern, count, offset) ) );
    let mut results = Vec::new();
    for row in sql_result {
        let row = try!( row );
        let (id, name) = from_row( row );
        let user = ShortInfo {
            id: id,
            name: name,
        };
        results.push( user );
    }
    Ok( results )
}

fn get_reg_key_impl( conn: &mut PooledConn, name: &str ) -> mysql::Result<Option<String>> {
    let mut stmt = try!( conn.prepare(
        "SELECT `regkey`
         FROM `users`
         WHERE `login`=?" ) );
    let mut sql_result = try!( stmt.execute( (name,) ) );
    let result = match sql_result.next() {
        Some( row ) => {
            let row = try!( row );
            let (regkey,) = from_row( row );
            Some( regkey )
        }
        None => None
    };
    Ok( result )
}
