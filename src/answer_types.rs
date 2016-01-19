/// Здесь собраны общие типы ответов на запросы

/// Общий тип когда операция выполнилась, и нужно сообщить что просто всё хорошо
#[derive(RustcEncodable)]
pub struct OkInfo {
    ok: String
}

impl OkInfo {
    pub fn new( msg: &str ) -> OkInfo {
        OkInfo {
            ok: String::from( msg )
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
            field: String::from( field ),
            reason: String::from( reason )
        }
    }
    pub fn empty( field: &str ) -> FieldErrorInfo {
        FieldErrorInfo::new( field, "empty" )
    }
    pub fn too_long( field: &str ) -> FieldErrorInfo {
        FieldErrorInfo::new( field, "too_long" )
    }
    pub fn not_found( field: &str ) -> FieldErrorInfo {
        FieldErrorInfo::new( field, "not_found" )
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
            photo: String::from( "bad_image" )
        }
    }

    pub fn too_big() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: String::from( "too_big" )
        }
    }

    pub fn unknown_format() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: String::from( "unknown_format" )
        }
    }

    pub fn not_found() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: String::from( "not_found" )
        }
    }

    pub fn already_published() -> PhotoErrorInfo {
        PhotoErrorInfo {
            photo: String::from( "already_published" )
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
            access: String::from( "denied" )
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

#[derive(RustcEncodable)]
pub struct PaginationInfo {
    pub current: u32,
    pub count: u32
}
