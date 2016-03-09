use iron::typemap::Key;
use std::thread;
use std::convert::From;
use std::time::Duration;

use authentication::User;
use std::sync::mpsc::{ Sender, channel, SendError };
use std::sync::{ Arc, Mutex };
use db::mailbox::DbMailbox;
use types::{ EmptyResult, CommonError };
use database::{ Databaseable };
use stuff::{ Stuff, StuffInstallable };

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{ self, SmtpTransportBuilder };
use lettre::transport::EmailTransport;

struct MailSender( Sender<Mail> );

const PHOTOMETER : &'static str = "Фотометр";

pub trait Mailer {
    fn send_mail( &mut self, user: &User, subject: &str, body: &str  ) -> EmptyResult;
    fn send_external_mail( &mut self, user: &User, subject: &str, body: &str ) -> EmptyResult;
    fn send_internal_mail( &mut self, user: &User, subject: &str, body: &str ) -> EmptyResult;
}

pub fn create( context: MailContext ) -> MailerBody {
    MailerBody {
        etalon_sender: Arc::new( Mutex::new( create_mail_thread( context ) ) )
    }
}

impl Mailer for Stuff {
    fn send_mail( &mut self, user: &User, subject: &str, body: &str ) -> EmptyResult {
        try!( self.send_internal_mail( user, subject, body ) );
        //FIXME: Временно на время тестов отключаем посылку писем во вне
        // try!( self.send_external_mail( user, subject, body ) );
        Ok( () )
    }
    fn send_external_mail( &mut self, _user: &User, _subject: &str, _body: &str ) -> EmptyResult {
        //FIXME: Временно на время тестов отключаем посылку писем во вне
        Ok( () )
    }
    fn send_internal_mail( &mut self, user: &User, subject: &str, body: &str ) -> EmptyResult {
        self.send_mail_internal( user, PHOTOMETER, subject, body )
    }
}

trait MailerPrivate {
    fn send_mail_external( &mut self, user: &User, subject: &str, body: &str ) -> EmptyResult;
    fn send_mail_internal( &mut self, user: &User, sender: &str, subject: &str, body: &str ) -> EmptyResult;
}

impl MailerPrivate for Stuff {
    fn send_mail_external( &mut self, user: &User, subject: &str, body: &str ) -> EmptyResult {
        // здесь реализовано ленивое создание посыльщика писем с кешированием
        // елси в этом контексте мы его уже создавали то просто используем
        // а елси не создавали то создаём копию для текущего потока с эталона и кэшируем его
        if self.extensions.contains::<MailSender>() == false {
            let tx = {
                let body = self.extensions.get::<MailerBody>().unwrap();
                let sender: &MailSender = &body.etalon_sender.lock().unwrap();
                let &MailSender( ref tx ) = sender;
                tx.clone()
            };
            //кэшируем
            self.extensions.insert::<MailSender>( MailSender( tx ) );
        }
        //отсылаем в поток посылки почты новое письмо
        let &MailSender( ref tx ) = self.extensions.get::<MailSender>().unwrap();
        try!( tx.send( Mail {
            to_addr: user.mail.clone(),
            to_name: user.name.clone(),
            subject: subject.to_string(),
            body: body.to_string(),
        } ) );
        Ok( () )
    }

    fn send_mail_internal( &mut self, user: &User, sender: &str, subject: &str, body: &str ) -> EmptyResult {
        // делаем запись в базе о новом оповещении
        let db = try!( self.get_current_db_conn() );
        try!( db.send_mail_to( user.id, sender, subject, body ) );
        Ok( () )
    }
}

impl From<SendError<Mail>> for CommonError {
    fn from(err: SendError<Mail>) -> CommonError {
        CommonError( format!( "error sending mail to mailer channel: {}", err ) )
    }
}

#[derive(Clone)]
pub struct MailerBody {
    etalon_sender: Arc<Mutex<MailSender>>
}

pub struct MailContext {
    smtp_addr: String,
    from_addr: String,
    pass: String
}

// сделал pub потому что иначе компилятор не даёт его использовать в FromError
pub struct Mail {
    to_addr: String,
    to_name: String,
    //sender_name: String,
    subject: String,
    body: String
}

impl Key for MailerBody { type Value = MailerBody; }
impl Key for MailSender { type Value = MailSender; }

impl MailContext {
    pub fn new( smtp_addr: &str, from_addr: &str, pass: &str ) -> MailContext {
        MailContext {
            smtp_addr: smtp_addr.to_owned(),
            from_addr: from_addr.to_owned(),
            pass: pass.to_owned()
        }
    }
    fn send_mail( &self, mail: Mail ) {
        let email = EmailBuilder::new()
            .to( (&mail.to_addr as &str, &mail.to_name as &str) )
            .from( &self.from_addr as &str )
            .subject( &mail.subject )
            .body( &mail.body )
            .build();

        let email = match email {
            Ok( email ) => email,
            Err( desc ) => {
                error!( "error creating email: {}", desc );
                return;
            }
        };

        let mut try_count = 0;
        while try_count < 6 {
            let builder = SmtpTransportBuilder::new( (&self.smtp_addr as &str, smtp::SUBMISSION_PORT) );
            let builder = match builder {
                Ok( builder ) => builder,
                Err( e ) => {
                    error!( "error creating SmtpTransportBuilder with addr: {}, error description: {}", self.smtp_addr, e );
                    return;
                }
            };

            let mut sender = builder
                .credentials( &self.from_addr, &self.pass )
                .build();

            match sender.send( email.clone() ) {
                Ok( _ ) => {
                    debug!( "mail to '{}' with subject='{}' successfully sended.",
                             mail.to_addr,
                             mail.subject );
                    break;
                },
                Err( e ) => {
                    error!( "error sending email to '{}' description: {}", mail.to_addr, e );
                    thread::sleep( Duration::from_secs( 10 ) );
                    try_count += 1;
                }
            }
        }

    }
}

impl StuffInstallable for MailerBody {
    fn install_to( &self, stuff: &mut Stuff ) {
        stuff.extensions.insert::<MailerBody>( self.clone() );
    }
}

fn create_mail_thread( context: MailContext ) -> MailSender {
    let (tx, rx) = channel();

    thread::spawn( move || {
        loop {
            match rx.recv() {
                Ok( mail ) => context.send_mail( mail ),
                // Если все те кто могли послать ушли то и мы закрываемся
                Err( _ ) => break
            }
        }
        info!( "mail send loop closed" );
    });

    MailSender( tx )
}
