use stuff::{ Stuff, StuffInstallable };
use std::sync::Arc;
use iron::typemap::Key;

pub trait MailWriter {
    // сочиняет письмо о подтверждении регистрации
    fn write_registration_accept_mail( &self, reg_key: &str ) -> String;
}

// саоздаёт экземпляр Сочинителя Писем для установки его в Stuff
pub fn create( root_url: &str ) -> MailWriterBody {
    MailWriterBody {
        root_url: Arc::new( root_url.to_string() )
    }
}

#[derive(Clone)]
pub struct MailWriterBody {
    root_url: Arc<String>
}

impl Key for MailWriterBody { type Value = MailWriterBody; }

impl StuffInstallable for MailWriterBody {
    fn install_to( &self, stuff: &mut Stuff ) {
        stuff.extensions.insert::<MailWriterBody>( (*self).clone() );
    }
}

trait MailWriterPrivate {
    fn get_body(&self) -> &MailWriterBody;
}

impl MailWriterPrivate for Stuff {
    fn get_body(&self) -> &MailWriterBody {
        self.extensions.get::<MailWriterBody>().unwrap()
    }   
}

impl MailWriter for Stuff {
    fn write_registration_accept_mail( &self, reg_key: &str ) -> String {
        let body = self.get_body();
        format!( "Вы начали регистрацию на Фотометре.
Что-бы завершить её, перейдите по этой ссылке {}/registration/{}",
            body.root_url.as_slice(), 
            reg_key
        )
    }
}
