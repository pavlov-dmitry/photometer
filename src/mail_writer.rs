use stuff::{ Stuff, StuffInstallable };
use std::sync::Arc;
use iron::typemap::Key;
use events;
use types::Id;
//TODO: Возможно стоит шаблоны писем вынести из кода в отдельные файлы.

/// возвращает для всех писем (тема, само_письмо)
pub trait MailWriter {
    /// РЕГИСТРАЦИЯ
    /// сочиняет письмо о подтверждении регистрации
    fn write_registration_accept_mail( &self, reg_key: &str ) -> (String, String);
    /// сочиняет приветственное письмо
    fn write_welcome_mail( &self ) -> (String, String);

    /// СОЗДАНИЕ ГРУППЫ
    /// cочиняет письмо о создании новой группы
    fn write_group_creation_started_mail( &self, group_name: &str, scheduled_id: Id) -> (String, String);
    /// сочиняет письмо с приглашением создать новую группу
    fn write_group_invite_mail( &self, group_name: &str, scheduled_id: Id ) -> (String, String);
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
                                         scheduled_id: Id ) -> (String, String);
    /// сочиняет письмо о том что опоздал конечно, но выложиться можешь
    fn write_late_publication_mail( &self,
                                     event_name: &str,
                                     user_name: &str,
                                     scheduled_id: Id ) -> (String, String);


    /// ГОЛОСОВАНИЕ
    /// письмо о том что началось новое голосование
    fn write_group_voiting_started_mail( &self,
                                          event_name: &str,
                                          group_name: &str,
                                          scheduled_id: Id ) -> (String, String);
    /// письмо о том что голосание закончилось и принято то за что голосовали
    fn write_group_voiting_accepted_mail( &self,
                                           event_name: &str,
                                           group_name: &str,
                                           scheduled_id: Id ) -> (String, String);
    /// письмо о том что голосование закончилось и группа проголосавала против
    fn write_group_voiting_denied_mail( &self,
                                         event_name: &str,
                                         group_name: &str,
                                         scheduled_id: Id ) -> (String, String);

    /// ПРИГЛАШЕНИЕ ПОЛЬЗОВАТЕЛЯ В ГРУППУ
    // письмо о том что вас приглашают в группу
    fn write_invite_to_group_mail( &self,
                                    inviter_name: &str,
                                    group_name: &str,
                                    scheduled_id: Id ) -> (String, String);

    // письмо о том что пользователю было выслано письмо о регистрации, с сылкой со слежением
    fn write_user_invited_to_group_mail( &self,
                                          invited_name: &str,
                                          group_name: &str,
                                          scheduled_id: Id ) -> (String, String);

    // письмо о том что полсле согласия пользователя, слово должна сказать группа
    fn write_time_for_group_join_voit_mail( &self,
                                            group_name: &str,
                                            scheduled_id: Id ) -> (String, String);
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
Что-бы завершить её, перейдите по этой ссылке {}/#activate/{}",
            &body.root_url,
            reg_key
        );
        ( String::from( "Регистрация" ), mail )
    }

    /// сочиняет приветственное письмо
    fn write_welcome_mail( &self ) -> (String, String) {
        let mail = format!( "Добро пожаловать на Фотометр!\n
Для того, чтобы использовать фотометр по полной программе, Вам надо объединиться в другими людьми. Для этого, нужно или создать новую группу, или присоединиться к уже существующей.\n
Создать новую группу вы можете выбрав выпадающее меню с вашим именем в шапке сайта. Присоединиться к уже существующей можно только по приглашению одного из её участников." );
        ( String::from( "Добро пожаловать." ), mail )
    }

    /// cочиняет письмо о создании новой группы
    fn write_group_creation_started_mail( &self, group_name: &str, scheduled_id: Id) -> (String, String) {
        let subject = format!( "Создание новой группы \"{}\"", group_name );
        let mail = format!(
"Приглашения о создании группы **{}** разосланы.
Узнать подробности и следить за процессом присоединения вы можете пройдя по этой [ссылке]({}).
Если группа будет организована, вы получите новое сообщение, то же самое будет если она не будет организована по какой-то причине.",
            group_name,
            events::make_event_link( scheduled_id )
        );
        (subject, mail)
    }

    /// cочиняет письмо о создании новой группы
    fn write_group_invite_mail( &self, group_name: &str, scheduled_id: Id ) -> (String, String) {
        let subject = format!( "Создание новой группы \"{}\"", group_name );
        let mail = format!(
"Вас приглашают создать новую группу **{}**.
Узнать подробности и принять решение о присоединении вы можете пройдя по этой [ссылке]({}).",
            group_name,
            events::make_event_link( scheduled_id )
        );
        (subject, mail)
    }
    /// сочиняет письмо о том что никто не захотел в твою группу
    fn write_nobody_need_your_group_mail( &self, group_name: &str ) -> (String, String) {
        let subject = format!( "Группа '{}' не создана", group_name );
        let mail = String::from(
            "К сожалению ни один из приглашенных вами пользователей не согласился создать группу."
        );
        ( subject, mail )
    }
    /// сочинаяет письмо о том что группа с таким именем уже существует
    fn write_group_name_already_exists_mail( &self, group_name: &str ) -> (String, String) {
        let subject = format!( "Группа с именем '{}' уже существует", group_name );
        let mail = format!(
"Группа с именем **{}** была уже создана за время пока вы решали создавать вашу группу или нет.
Создайте новую группу с другим именем или присоединитесь к существующей",
            group_name
        );
        ( subject, mail )
    }
    /// сочиняет письмо что группа создана, и всё хорошо
    fn write_welcome_to_group_mail( &self, group_name: &str ) -> (String, String) {
        let subject = format!( "Добро пожаловать в группу \"{}\"", group_name );
        let mail = format!( "Группа с именем **{}** создана. Развлекайтесь!", group_name );
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
"Привет, **{}**!
Настало время публиковать фотографии для **{event}**.
Ты можешь это сделать перейдя по вот этой ссылке: [{event}]({}{})",
            user_name,
            &body.root_url,
            events::make_event_link( scheduled_id ),
            event = event_name
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
            events::make_event_link( scheduled_id )
        );
        ( subject, mail )
    }

    /// письмо о том что началось новое голосование
    fn write_group_voiting_started_mail( &self,
                                          event_name: &str,
                                          group_name: &str,
                                          scheduled_id: Id ) -> (String, String)
    {
        let subject = format!( "Старт голосования за '{}'", event_name );
        let mail = format!( "В группе **{}** грядут изменения: {event}. Группа нуждается в твоём мнении! Вырази его перейдя по ссылке: [{event}]({})",
                             group_name,
                             events::make_event_link( scheduled_id ),
                             event = event_name );
        (subject, mail)
    }
    /// письмо о том что голосание закончилось и принято то за что голосовали
    fn write_group_voiting_accepted_mail( &self,
                                           event_name: &str,
                                           group_name: &str,
                                           scheduled_id: Id ) -> (String, String)
    {
        let subject = format!( "Утверждено '{}'", event_name );
        let mail = format!( "В группе **{}** утверждено изменение: {event}. Вот ссылка на результаты голосования [{event}]({})",
                             group_name,
                             events::make_event_link( scheduled_id ),
                             event = event_name );
        (subject, mail)
    }
    /// письмо о том что голосование закончилось и группа проголосавала против
    fn write_group_voiting_denied_mail( &self,
                                         event_name: &str,
                                         group_name: &str,
                                         scheduled_id: Id ) -> (String, String)
    {
        let subject = format!( "Отклонено '{}'", event_name );
        let mail = format!( "В группе **{}** отклонено изменение: {event}. Вот ссылка на результаты голосования [{event}]({})",
                             group_name,
                             events::make_event_link( scheduled_id ),
                             event = event_name );
        (subject, mail)
    }

    // письмо о том что вас приглашают в группу
    fn write_invite_to_group_mail(
        &self,
        inviter_name: &str,
        group_name: &str,
        scheduled_id: Id ) -> (String, String)
    {
        let subject = format!( "Приглашение в группу '{}'", group_name );
        let mail = format!( "Пользователь **{}** приглашает вас присоединиться в группу **{}**. Прочитать про группу и принять решение, можно перейдя по [вот этой ссылке]({}).",
                             inviter_name,
                             group_name,
                             events::make_event_link( scheduled_id ) );
        (subject, mail)
    }

    // письмо о том что пользователю было выслано письмо о регистрации, с сылкой со слежением
    fn write_user_invited_to_group_mail( &self,
                                          invited_name: &str,
                                          group_name: &str,
                                          scheduled_id: Id ) -> (String, String)
    {
        let subject = format!( "'{}' приглашен в группу '{}'", invited_name, group_name );
        let mail = format!( "Приглашение в группу **{}** для пользователя **{}** выслано, добавить комментарий и следить за прогрессом можно перейдя [вот по этой ссылке]({}).",
                             group_name,
                             invited_name,
                             events::make_event_link( scheduled_id ) );
        (subject, mail)
    }

    // письмо о том что полсле согласия пользователя, слово должна сказать группа
    fn write_time_for_group_join_voit_mail( &self,
                                            group_name: &str,
                                            scheduled_id: Id ) -> (String, String)
    {
        let subject = format!( "Голосование группы '{}' за ваше принятие", group_name );
        let mail = format!( "После вашего согласия присоединиться к группе **{}**, теперь группа должна принять решение о вашем вступлении. Следить за их решением можно перейдя [вот по этой ссылке]({}).",
                             group_name,
                             events::make_event_link( scheduled_id ) );
        (subject, mail)
    }
}
