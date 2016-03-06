use mysql;
use std::convert::From;
use std::fmt;
use std::fmt::{ Display, Formatter };

pub type DbHandlerResult<T> = Result<T, DbHandlerError>;

pub enum DbHandlerError {
    MySql( mysql::Error ),
    Common( String )
}

impl From<mysql::Error> for DbHandlerError {
    fn from( err: mysql::Error ) -> DbHandlerError {
        DbHandlerError::MySql( err )
    }
}

impl From<String> for DbHandlerError {
    fn from( err: String ) -> DbHandlerError {
        DbHandlerError::Common( err )
    }
}

impl Display for DbHandlerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &DbHandlerError::MySql( ref e ) => write!(f, "{}", e ),
            &DbHandlerError::Common( ref e ) => write!(f, "{}", e )
        }
    }
}
