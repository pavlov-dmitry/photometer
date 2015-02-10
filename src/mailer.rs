use typemap::Assoc;
use plugin::Extensible;
use std::sync::{ Arc, RWLock };
use std::io::process::Command;
use std::io::IoResult;
use std::io::fs::File;
use std::task;

use authentication::User;
use std::comm::{ Sender };
use db::mailbox::DbMailbox;
use types::{ EmptyResult };
use database::{ Databaseable };
use stuff::{ Stuff, StuffInstallable };

type MailSender = Sender<Mail>;

#[derive(Clone)]
struct MailerMiddleware {
    sender: MailSender
}

pub trait Mailer {
    fn send_mail( &mut self, user: &User, sender: &str, subject: &str, body: &str  ) -> EmptyResult;
}

impl<'a, 'b> Mailer for Request<'a, 'b>{
    fn send_mail( &mut self, user: &User, sender: &str, subject: &str, body: &str ) -> EmptyResult {
        // делаем запись в базе о новом оповещении
        {
            let db = try!( self.get_current_db_conn() );
            try!( db.send_mail( user.id, sender, subject, body ) );
        }
        //отсылаем в поток посылки почты новое письмо
        let tx = self.extensions().get::<MailerMiddleware, MailSender>().unwrap();
        let _ = tx.send_opt( Mail {
            to_addr: user.mail.clone(),
            to_name: user.name.clone(),
            sender_name: sender.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        } );
        Ok( () )
    }
}

struct MailContext {
    smtp_addr: String,
    from_addr: String,
    tmp_mail_file: String,
    auth_info: String
}

struct Mail {
    to_addr: String,
    to_name: String,
    sender_name: String,
    subject: String,
    body: String
}

impl Assoc<MailSender> for MailerMiddleware {}

//curl --url "smtps://smtp.gmail.com:465" --ssl-reqd --mail-from "photometer.org.ru@gmail.com" --mail-rcpt "voidwalker@mail.ru" --upload-file ./mail.txt --user "photometer.org.ru@gmail.com:ajnjvtnhbxtcrbq" --insecure
impl MailContext {
    pub fn new( smtp_addr: &str, from_addr: &str, pass: &str, tmp_file_path: &str ) -> MailContext {
        MailContext {
            smtp_addr: format!( "\"{}\"", smtp_addr ),
            from_addr: format!( "\"{}\"", from_addr ),
            tmp_mail_file: tmp_file_path.to_string(),
            auth_info: format!( "\"{}:{}\"", from_addr, pass )
        }
    }
    fn send_mail( &self, mail: Mail ) {
        //создаём текстовый файл со скриптом
        if let Err( e ) = self.make_mail_file( &mail ) {
            error!( "fail to create tmp mail file: {}", e );
            return;
        }
        //запускаем curl на передачу записанного письма
        let process = Command::new( "curl" )
            .arg( "--url" )
            .arg( self.smtp_addr.as_slice() )
            .arg( "--ssl-reqd" )
            .arg( "--mail-from" )
            .arg( self.from_addr.as_slice() )
            .arg( "--mail-rcpt" )
            .arg( format!( "\"{}\"", mail.to_addr ) )
            .arg( "--upload-file" )
            .arg( self.tmp_mail_file.as_slice() )
            .arg( "--user" )
            .arg( self.auth_info.as_slice() )
            .arg( "--insecure" )
            .output();

        let process = match process {
            Ok( process ) => process,
            Err( e ) => panic!( "fail to create 'curl' process: {}", e )
        };
        if process.status.success() == false {
            let err_string = String::from_utf8_lossy(process.error.as_slice());
            error!( "fail to send mail: {}", err_string );
        }
    } 
    fn make_mail_file( &self, mail: &Mail ) -> IoResult<()> {
        let mut file = try!( File::create( &Path::new( self.tmp_mail_file.as_slice() ) ) );
        try!( file.write_line( format!( "From: \"photometer\" <{}>", self.from_addr ).as_slice() ) );
        try!( file.write_line( format!( "To: \"{}\" <{}>", mail.to_name, mail.to_addr ).as_slice() ) );
        try!( file.write_line( format!( "Subject: {}", mail.subject ).as_slice() ) );
        try!( file.write_line( "" ) );
        try!( file.write_str( mail.body.as_slice() ) );
        Ok( () )
    }
}

impl Middleware for MailerMiddleware {
    fn invoke(&self, req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        req.extensions_mut().insert::<MailerMiddleware, MailSender>( self.sender.clone() );
        Ok( Continue )
    }
}

fn middleware( context: MailContext ) -> MailerMiddleware {
    let (tx, rx) = sync_channel();

    task::spawn( move || {  
        loop {
            context.send_mail( rx.recv() );
        }
    })/*.detach()*/;

    MailerMiddleware {
        sender: tx
    }
}
