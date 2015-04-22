/// Здесь собраны общие типы ответов на запросы

/// Общий тип когда операция выполнилась, и нужно сообщить что просто всё хорошо
#[derive(RustcEncodable)]
pub struct OkInfo {
    ok: String
}

impl OkInfo {
    pub fn new( msg: &str ) -> OkInfo {
        OkInfo {
            ok: From::from( msg )
        }
    }
}

/// Ошибка в каком то поле в запросе
#[derive(RustcEncodable)]
pub struct FieldErrorInfo {
    field: String,
    reason: String
}

impl FieldErrorInfo {
    pub fn new( field: &str, reason: &str ) -> FieldErrorInfo {
        FieldErrorInfo {
            field: From::from( field ),
            reason: From::from( reason )
        }
    }
}

/// Ошибка в загружаемом файле
#[derive(RustcEncodable)]
pub struct PhotoErrorInfo {
    photo: String
}

impl PhotoErrorInfo {
    pub fn bad_image() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: From::from( "bad_image" )
        }
    }

    pub fn too_big() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: From::from( "too_big" )
        }
    }

    pub fn unknown_format() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: From::from( "unknown_format" )
        }
    }

    pub fn not_found() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: From::from( "not_found" )
        }
    }
}


/// Отказ в доступе
#[derive(RustcEncodable)]
pub struct AccessErrorInfo {
    access: String
}

impl AccessErrorInfo {
    pub fn new() -> AccessErrorInfo {
        AccessErrorInfo {
            access: From::from( "denied" )
        }
    }
}


#[derive(RustcEncodable)]
pub struct CountInfo {
    count: u32
}

impl CountInfo {
    pub fn new( count: u32 ) -> CountInfo {
        CountInfo {
            count: count
        }
    }
}
