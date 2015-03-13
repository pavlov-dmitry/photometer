use mysql::conn::pool::{ MyPooledConn };
use mysql::error::{ MyResult };
use mysql::value::{ from_value, ToValue };
use types::{ Id, CommonResult, EmptyResult };
use std::fmt::Display;
use database::Database;

use authentication::User;

pub trait DbUsers {
    /// выбирает id пользователя по имени и паролю
    fn get_user( &mut self, name: &str, pass: &str ) -> CommonResult<Option<User>>;
    /// добавляет нового пользователя в БД
    fn add_user( &mut self, name: &str, pass: &str, mail: &str, regkey: &str ) -> CommonResult<User>;
    /// активирует пользователя по определённому регистрационному ключу
    fn activate_user( &mut self, regkey: &str ) -> CommonResult<Option<User>>;
    /// проверяет наличие имени в БД
    fn user_exists(&mut self, name: &str, mail: &str) -> CommonResult<bool>;
    fn user_id_exists(&mut self, id: Id ) -> CommonResult<bool>;
    /// возвращает пользователя по Id
    fn user_by_id( &mut self, id: Id ) -> CommonResult<Option<User>>;
    /// возвращает пользователей по Id
    fn users_by_id( &mut self, ids: &[Id] ) -> CommonResult<Vec<User>>;
}

pub fn create_tables( db: &Database ) -> EmptyResult {
    db.execute(  
        "CREATE TABLE IF NOT EXISTS `users` (
            `id` bigint(20) NOT NULL AUTO_INCREMENT,
            `login` varchar(16) NOT NULL DEFAULT '',
            `password` varchar(32) NOT NULL DEFAULT '',
            `activated` BOOL NOT NULL DEFAULT false,
            `mail` varchar(256) NOT NULL DEFAULT '',
            `regkey` varchar(256) NOT NULL DEFAULT '',
            PRIMARY KEY (`id`),
            KEY `login_idx` (`login`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
        ",
        "db::users::create_tables"
    )
}

impl DbUsers for MyPooledConn {
    /// выбирает id пользователя по имени и паролю
    fn get_user( &mut self, name: &str, pass: &str ) -> CommonResult<Option<User>> {
        get_user_impl( self, name, pass )
            .map_err( |e| fn_failed( "get_user", e ) )
    }
    
    /// добавляет нового пользователя в БД
    fn add_user( &mut self, name: &str, pass: &str, mail: &str, regkey: &str ) -> CommonResult<User> {
        add_user_impl( self, name, pass, mail, regkey )
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
}

fn fn_failed<E: Display>( fn_name: &str, e: E ) -> String {
    format!( "DbUsers func '{}' failed: {}", fn_name, e )
}

fn get_user_impl( conn: &mut MyPooledConn, name: &str, pass: &str ) -> MyResult<Option<User>> {
    let name = name.to_string(); // помогает убрать internal compiler error
    let pass = pass.to_string();
    let mut stmt = try!( conn.prepare( 
        "SELECT 
            `login`, 
            `id`, 
            `mail`
        FROM `users` 
        WHERE 
            `login`=? AND 
            `password`=? AND 
            `activated`=true" 
    ) );
    let mut sql_result = try!( stmt.execute( &[ &name, &pass ] ) );
    match sql_result.next() {
        None => Ok( None ),
        Some( row ) => {
            let row = try!( row );
            let user = User {
                name: from_value( &row[ 0 ] ),
                id: from_value( &row[ 1 ] ),
                mail: from_value( &row[ 2 ] )
            };
            Ok( Some( user ) )
        }
    }
}

fn add_user_impl( conn: &mut MyPooledConn, name: &str, pass: &str, mail: &str, regkey: &str ) -> MyResult<User> {
    let name = name.to_string();
    let pass = pass.to_string();
    let mut stmt = try!( conn.prepare( 
        "INSERT INTO users (
            login, 
            password, 
            mail, 
            regkey
        ) VALUES(?, ?, ?, ?);" 
    ) );
    let result = try!( stmt.execute( &[ &name, &pass, &mail, &regkey ] ) );
    Ok( User {
        name: name.to_string(),
        id: result.last_insert_id(),
        mail: mail.to_string()
    } )
}

fn activate_user_impl( conn: &mut MyPooledConn, regkey: &str ) -> MyResult<Option<User>> {
    let maybe_user = {
        let mut stmt = try!( conn.prepare(
            "SELECT 
                `id`, 
                `login`, 
                `mail` 
            FROM `users` 
            WHERE 
                `regkey`=? AND 
                `activated`=false"
        ) );
        let mut result = try!( stmt.execute( &[ &regkey ] ) );    
        match result.next()  {
            Some( row ) => {
                let row = try!( row );
                Some( User {
                    id: from_value( &row[ 0 ] ),
                    name: from_value( &row[ 1 ] ),
                    mail: from_value( &row[ 2 ] )
                } )
            }
            None => None
        }
    };
    if let Some( ref user ) = maybe_user {
        let mut stmt = try!( conn.prepare(
            "UPDATE users SET activated=true WHERE id=?"
        ) );
        try!( stmt.execute( &[ &user.id ] ) );
    } 
    Ok( maybe_user )
}

fn user_exists_impl( conn: &mut MyPooledConn, name: &str, mail: &str ) -> MyResult<bool> {
    let name = name.to_string();
    let mut stmt = try!( conn.prepare( "select id from users where login=? OR mail=?" ) );
    let sql_result = try!( stmt.execute( &[ &name, &mail ] ) );
    Ok( sql_result.count() == 1 )
}

fn user_id_exists_impl( conn: &mut MyPooledConn, id: Id  ) -> MyResult<bool> {
    let mut stmt = try!( conn.prepare( "select id from users where id=? AND activated=true" ) );
    let sql_result = try!( stmt.execute( &[ &id ] ) );
    Ok( sql_result.count() == 1 )
}

fn user_by_id_impl( conn: &mut MyPooledConn, id: Id ) -> MyResult<Option<User>> {
    let mut stmt = try!( conn.prepare( "select login, mail from users where id=? AND activated=true" ) );
    let mut sql_result = try!( stmt.execute( &[ &id ] ) );
    match sql_result.next() {
        None => Ok( None ),
        Some( row ) => {
            let row = try!( row );
            let user = User {
                name: from_value( &row[ 0 ] ),
                id: id,
                mail: from_value( &row[ 1 ] )
            };
            Ok( Some( user ) )
        }
    }
}

fn users_by_id_impl( conn: &mut MyPooledConn, ids: &[Id] ) -> MyResult<Vec<User>> {
    let mut query = format!( "select id, login, mail from users where id=? AND activated=true" );
    for _ in range( 1, ids.len() ) {
        query.push_str( ", ?" );
    }

    let mut stmt = try!( conn.prepare( query.as_slice() ) );
    let mut values: Vec<&ToValue> = Vec::new();
    for id in ids.iter() {
        values.push( id );
    }

    let mut users = Vec::new();
    let sql_result = try!( stmt.execute( &values ) );
    for row in sql_result {
        let row = try!( row );
        let user = User {
            id: from_value( &row[ 0 ] ),
            name: from_value( &row[ 1 ] ),
            mail: from_value( &row[ 2 ] )
        };
        users.push( user );
    }
    Ok( users )
}