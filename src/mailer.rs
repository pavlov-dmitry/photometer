use iron::typemap::Key;
use std::old_io::process::Command;
use std::old_io::IoResult;
use std::old_io::fs::File;
use std::old_io::stdio::stderr;
use std::thread::Thread;
use std::error::FromError;

use authentication::User;
use std::sync::mpsc::{ Sender, channel, SendError };
use db::mailbox::DbMailbox;
use types::{ EmptyResult };
use database::{ Databaseable };
use stuff::{ Stuff, StuffInstallable };

type MailSender = Sender<Mail>;

#[derive(Clone)]
struct MailerBody {
    sender: MailSender
}

pub trait Mailer {
    fn send_mail( &mut self, user: &User, sender: &str, subject: &str, body: &str  ) -> EmptyResult;
}

impl Mailer for Stuff {
    fn send_mail( &mut self, user: &User, sender: &str, subject: &str, body: &str ) -> EmptyResult {
        // делаем запись в базе о новом оповещении
        {
            let db = try!( self.get_current_db_conn() );
            try!( db.send_mail_to( user.id, sender, subject, body ) );
        }
        //отсылаем в поток посылки почты новое письмо
        let tx = self.extensions.get::<MailerBody>().unwrap();
        try!( tx.send( Mail {
            to_addr: user.mail.clone(),
            to_name: user.name.clone(),
            sender_name: sender.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        } ) );
        Ok( () )
    }
}

impl FromError<SendError<Mail>> for String {
    fn from_error(err: SendError<Mail>) -> String {
        format!( "error sending mail to mailer channel: {}", err )
    }
}

struct MailContext {
    smtp_addr: String,
    from_addr: String,
    tmp_mail_file: String,
    auth_info: String
}

// сделал pub потому что иначе компилятор не даёт его использовать в FromError
pub struct Mail {
    to_addr: String,
    to_name: String,
    sender_name: String,
    subject: String,
    body: String
}

impl Key for MailerBody { type Value = MailSender; }

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
            let _ = writeln!( &mut stderr(), "fail to create tmp mail file: {}", e );
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
            let _ = writeln!( &mut stderr(), "fail to send mail: {}", err_string );
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

impl StuffInstallable for MailerBody {
    fn install_to( &self, stuff: &mut Stuff ) {
        stuff.extensions.insert::<MailerBody>( self.sender.clone() );
    }
}

fn create( context: MailContext ) -> MailerBody {
    let (tx, rx) = channel();

    Thread::spawn( move || {  
        loop {
            match rx.recv() {
                Ok( mail ) => context.send_mail( mail ),
                // Если все те кто могли послать ушли то и мы закрываемся
                Err( _ ) => break
            }
        }
    });

    MailerBody {
        sender: tx
    }
}
