use stuff::{ Stuff, StuffInstallable };
use std::sync::Arc;
use iron::typemap::Key;
use events;
use types::Id;

/// возвращает для всех писем (тема, само_письмо)
pub trait MailWriter {
    /// РЕГИСТРАЦИЯ
    /// сочиняет письмо о подтверждении регистрации
    fn write_registration_accept_mail( &self, reg_key: &str ) -> (String, String);
    /// сочиняет приветственное письмо
    fn write_welcome_mail( &self ) -> (String, String);

    /// СОЗДАНИЕ ГРУППЫ
    /// cочиняет письмо о создании новой группы
    fn write_group_creation_mail( &self, group_name: &str, scheduled_id: Id ) -> (String, String);
    /// сочиняет письмо о том что никто не захотел в твою группу
    fn write_nobody_need_your_group_mail( &self, group_name: &str ) -> (String, String);
    /// сочинаяет письмо о том что группа с таким именем уже существует
    fn write_group_name_already_exists_mail( &self, group_name: &str ) -> (String, String);
    /// сочиняет письмо что группа создана, и всё хорошо
    fn write_welcome_to_group_mail( &self, group_name: &str ) -> (String, String);

    /// ПУБЛИКАЦИЯ
    /// сочиняет письмо о том что пора выкладываться
    fn write_time_for_publication_mail( &self,
        event_name: &str,
        user_name: &str,
        scheduled_id: Id
    ) -> (String, String);
    /// сочиняет письмо о том что опоздал конечно, но выложиться можешь
    fn write_late_publication_mail( &self,
        event_name: &str,
        user_name: &str,
        scheduled_id: Id
    ) -> (String, String);
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
        ( From::from( "Регистрация" ), mail )
    }

    /// сочиняет приветственное письмо
    fn write_welcome_mail( &self ) -> (String, String) {
        let body = self.get_body();
        let mail = format!( "Добро пожаловать на Фотометр!
Попробуйте загрузить парочку фотографий в собственную галлерею {}/gallery",
                             &body.root_url );
        ( From::from( "Добро пожаловать." ), mail )
    }


    /// cочиняет письмо о создании новой группы
    fn write_group_creation_mail( &self, group_name: &str, scheduled_id: Id ) -> (String, String) {
        let body = self.get_body();
        let subject = format!( "Создание новой группы `{}`", group_name );
        let mail = format!(
"Вас приглашают создать новую группу `{}`.
Узнать подробности и принять решение о присоединении вы можете пройдя по этой ссылке {}{}.
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
        let mail = From::from(
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


    /// сочиняет письмо о том что пора выкладываться
    fn write_time_for_publication_mail( &self,
        event_name: &str,
        user_name: &str,
        scheduled_id: Id
    ) -> (String, String) {
        let body = self.get_body();
        let subject = format!( "Пора выкладывать {}", event_name );
        let mail = format!(
"Привет {}!
Настало время публиковать фотографии для '{}'.
Ты можешь сделать перейдя по вот этой ссылке: {}{}",
            user_name,
            event_name,
            &body.root_url,
            events::make_event_action_link( scheduled_id )
        );
        ( subject, mail )
    }
    /// сочиняет письмо о том что опоздал конечно, но выложиться можешь
    fn write_late_publication_mail( &self,
        event_name: &str,
        user_name: &str,
        scheduled_id: Id
    ) -> (String, String) {
        let body = self.get_body();
        let subject = format!( "Выкладываемся с опозданием {}", event_name );
        let mail = format!(
"Ну что, {}, не получилось вовремя опубликовать свою фотографию? Ну ничего, не растраивайся!
Ты всё равно можешь это сделать по вот этой ссылке {}{}. Возможно она уже не будет участвовать в конкурсах,
но хотя бы не будет этого слюнявого Гомера.",
            user_name,
            &body.root_url,
            events::make_event_action_link( scheduled_id )
        );
        ( subject, mail )
    }
}
