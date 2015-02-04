use mysql::error::{ MyError };
use std::error::FromError;
use std::fmt;
use std::fmt::{ Display, Formatter };

pub type DbHandlerResult<T> = Result<T, DbHandlerError>;

pub enum DbHandlerError {
    MySql( MyError ),
    Common( String )
}

impl FromError<MyError> for DbHandlerError {
    fn from_error( err: MyError ) -> DbHandlerError {
        DbHandlerError::MySql( err )
    }
}

impl FromError<String> for DbHandlerError {
    fn from_error( err: String ) -> DbHandlerError {
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