extern crate imap;
extern crate mailparse;
extern crate native_tls;

use std::net::TcpStream;
use std::option::Option;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use bot;
use config::Config;
use database;
use hash;
use io_tools;

use self::native_tls::{TlsConnector, TlsStream};

#[derive(Debug)]
struct Message {
    body: Option<Vec<u8>>,
    header: Option<Vec<u8>>,
    text: Option<Vec<u8>>,
}

#[derive(Debug)]
struct ProccessedMessage {
    text: Option<String>,
    filename: Option<String>,
}

#[derive(Debug, PartialEq)]
enum SelectError {
    BrokenPipe,
    Other,
}

#[derive(Debug, PartialEq)]
enum GetLatestError {
    Select(SelectError),
    Search,
    Fetch,
    Store,
}

fn vectorize(data: Option<&[u8]>) -> Option<Vec<u8>> {
    match data {
        Some(d) => Some(Vec::from(d)),
        None => None
    }
}

fn get_latest(session: &mut imap::Session<TlsStream<TcpStream>>) -> Result<Vec<Message>, GetLatestError> {
    let mut messages: Vec<Message> = vec![];
    match session.select("INBOX") {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error on selecting INBOX: {:?}", err);
            return Err(GetLatestError::Select(SelectError::BrokenPipe));
        }
    };
    let news = match session.search("UNSEEN") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error on getting latest messages: {:?}", err);
            return Err(GetLatestError::Search);
        }
    };

    for x in news {
        let buff = match session.fetch(format!("{}", x), "RFC822") {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error getting {} message: {:?}", x, err);
                continue;
            }
        };
        //println!("{:?}", buff);
        if buff.len() > 0 {
            messages.push(Message {
                body: vectorize(buff[0].body()),
                header: vectorize(buff[0].header()),
                text: vectorize(buff[0].text()),
            });
            match session.store(format!("{}", x), "+FLAGS.SILENT (\\Seen)") {
                Ok(_) => print!(""),
                Err(err) => {
                    eprintln!("Error on marking as seen: {}", err);
                    messages.pop();
                }
            };
        }
    }


    Ok(messages)
}

fn init(a_config: &Arc<Mutex<Config>>) -> Result<imap::Session<TlsStream<TcpStream>>, String> {
    let config = { a_config.lock().unwrap().clone() };
    let (server, port, user, password) = (config.imap.server.clone(), config.imap.port,
                                          config.imap.user.clone(), config.imap.password.clone());

    let tls = match TlsConnector::builder().build() {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Tls error: {:?}", err);
            return Err(format!("{:?}", err));
        }
    };

    let client = match imap::connect((server.as_str(), port),
                                     server.as_str(), &tls) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("IMAP connection error: {:?}", err);
            return Err(format!("{:?}", err));
        }
    };

    match client.login(&user, &password) {
        Ok(data) => Ok(data),
        Err(err) => {
            eprintln!("IMAP session error: {:?}", err);
            return Err(format!("{:?}", err));
        }
    }
}


fn process(message_body: Vec<u8>) -> Option<ProccessedMessage> {
    let parsed = match mailparse::parse_mail(message_body.as_slice()) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{:?}", err);
            return None;
        }
    };
    let mut proccessed = ProccessedMessage { text: None, filename: None };
    for x in parsed.subparts {
        if x.ctype.mimetype.as_str() == "application/pdf" {
            match x.get_body_raw() {
                Ok(data) => {
                    let filename = format!("{}.pdf", hash::hash_data(&data));
                    match io_tools::write_bytes_to_file(&filename, data) {
                        Ok(_) => proccessed.filename = Some(filename),
                        Err(err) => eprintln!("Error on writing file: {:?}", err),
                    }
                }
                Err(err) => eprintln!("Error on getting the body: {:?}", err),
            }
        } else {
            for y in x.subparts {
                if y.ctype.mimetype.as_str() == "text/plain" {
                    match y.get_body() {
                        Ok(data) => proccessed.text = Some(data.trim().to_string()),
                        Err(err) => eprintln!("Error on getting the body: {:?}", err),
                    }
                }
            }
        }
    }
    return Some(proccessed);
}


fn send_file(filename: String, user_token: String, a_config: Arc<Mutex<Config>>, a_users_table: Arc<Mutex<database::UsersTable>>) -> Result<(), String> {
    let (admin, bot_token, access) = {
        let users_table = a_users_table.lock().unwrap();
        (users_table.get_admin().clone(),
         a_config.lock().unwrap().token.clone(),
         users_table.check_token(user_token.clone()))
    };

    if !access {
        return Err(format!("No access: {}", &user_token));
    }

    println!("We are at file send2;");
    let _child = thread::spawn(move || {
        println!("We are at thread;");
        match bot::send_message(&bot_token, admin, &format!("Mail user {} wants to print:", user_token)) {
            Ok(_) => print!(""),
            Err(err) => eprintln!("Error on sending message: {:?}", err),
        };

        match bot::send_message(&bot_token, admin, &format!("{}", filename)) {
            Ok(_) => print!(""),
            Err(err) => eprintln!("Error on sending message: {:?}", err),
        };

        match bot::send_document(&bot_token, admin, &format!("{}", filename)) {
            Ok(_) => print!(""),
            Err(err) => eprintln!("Error on sending document: {:?}", err),
        };
    });

    Ok(())
}

fn react(message: ProccessedMessage, a_config: Arc<Mutex<Config>>, a_users_table: Arc<Mutex<database::UsersTable>>) {
    println!("{:?}", message);
    let text = match message.text {
        Some(data) => data,
        None => return,
    };

    if text.len() < 2 {
        return;
    }


    let result = match message.filename {
        Some(filename) => send_file(filename, text, a_config, a_users_table),
        None => return,
    };

    match result {
        Ok(_) => return,
        Err(err) => eprintln!("{}", err),
    }
}


/// Runs IMAP bot
///
/// You should provide `Config` and `UsersTable` as shared state `Arc<Mutex>`
///
/// # Examples
///
/// ```rust
/// let users_table = Arc::new(Mutex::new(read_users().unwrap()));
/// let config = Arc::new(Mutex::new(read_config().unwrap()));
/// let imap_bot = thread::spawn(move || {
///        run_bot(Arc::clone(&config), Arc::clone(&users_table));
///  });
/// ```
pub fn run_bot(config: Arc<Mutex<Config>>, users_table: Arc<Mutex<database::UsersTable>>) {
    let mut session = match init(&config) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("IMAP init error: {:?}", err);
            return;
        }
    };
    println!("Session ok");

    'main_loop: loop {
        let buff = match get_latest(&mut session) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Get latest error: {:?}", err);
                if err == GetLatestError::Select(SelectError::BrokenPipe) {
                    session = match init(&config) {
                        Ok(data) => data,
                        Err(err) => {
                            eprintln!("IMAP init error: {:?}", err);
                            continue 'main_loop;
                        }
                    };
                    println!("Session ok");
                }
                continue 'main_loop;
            }
        };
        for x in buff {
            let parsed = match x.body {
                Some(data) => {
                    process(data)
                }
                None => continue,
            };
            match parsed {
                Some(data) => react(data, Arc::clone(&config), Arc::clone(&users_table)),
                None => continue,
            };
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}