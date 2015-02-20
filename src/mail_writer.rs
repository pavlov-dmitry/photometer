use stuff::{ Stuff, StuffInstallable };
use std::sync::Arc;
use iron::typemap::Key;
use events;
use types::Id;

/// возвращает для всех писем (тема, само_письмо)
pub trait MailWriter {
    /// сочиняет письмо о подтверждении регистрации 
    fn write_registration_accept_mail( &self, reg_key: &str ) -> (String, String);

    /// cочиняет письмо о создании новой группы
    fn write_group_creation_mail( &self, group_name: &str, scheduled_id: Id ) -> (String, String);
    /// сочиняет письмо о том что никто не захотел в твою группу
    fn write_nobody_need_your_group_mail( &self, group_name: &str ) -> (String, String);
    /// сочинаяет письмо о том что группа с таким именем уже существует
    fn write_group_name_already_exists_mail( &self, group_name: &str ) -> (String, String);
    /// сочиняет письмо что группа создана, и всё хорошо
    fn write_welcome_to_group_mail( &self, group_name: &str ) -> (String, String);
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
    fn write_registration_accept_mail( &self, reg_key: &str ) -> (String, String) {
        let body = self.get_body();
        let mail = format!( "Вы начали регистрацию на Фотометре.
Что-бы завершить её, перейдите по этой ссылке {}/registration/{}",
            &body.root_url,
            reg_key
        );
        ( String::from_str( "Регистрация" ), mail )
    }

    /// cочиняет письмо о создании новой группы
    fn write_group_creation_mail( &self, group_name: &str, scheduled_id: Id ) -> (String, String) {
        let body = self.get_body();
        let subject = format!( "Создание новой группы `{}`", group_name );
        let mail = format!(  
"Вас приглашают создать новую группу `{}`.
Узнать подробности и принять решение о присоединении вы можете пройдя по этой ссылке {}/{}.
У вас есть сутки чтобы принять решение.",
            group_name,
            &body.root_url,
            events::make_event_action_link( scheduled_id )
        );
        (subject, mail)
    }
    /// сочиняет письмо о том что никто не захотел в твою группу
    fn write_nobody_need_your_group_mail( &self, group_name: &str ) -> (String, String) {
        let subject = format!( "Группа '{}' не создана", group_name );
        let mail = String::from_str( 
            "К сожалению ни один из приглашенных вами пользователей не согласился создать группу." 
        );
        ( subject, mail )
    }
    /// сочинаяет письмо о том что группа с таким именем уже существует
    fn write_group_name_already_exists_mail( &self, group_name: &str ) -> (String, String) {
        let subject = format!( "Группа с именем '{}' уже существует", group_name );
        let mail = format!( 
"Группа с именем '{}' была уже создана за время пока вы решали создавать вашу группу или нет. 
Создайте новую группу с другим именем или присоединитесь к существующей", 
            group_name
        );
        ( subject, mail )
    }
    /// сочиняет письмо что группа создана, и всё хорошо
    fn write_welcome_to_group_mail( &self, group_name: &str ) -> (String, String) {
        let subject = format!( "Добро пожаловать в группу {}", group_name );
        let mail = format!( "Группа с именем '{}' создана. Развлекайтесь!", group_name );
        ( subject, mail )
    }
}
