extern crate futures;
extern crate reqwest;
extern crate telebot;
extern crate tokio_core;

use std::sync::{Arc, Mutex};

use config;
use database;
use downloader;
use hash::generate_token;
use printer;

use self::futures::IntoFuture;
use self::futures::stream::Stream;
use self::telebot::functions::*;
use self::telebot::RcBot;
use self::tokio_core::reactor::Core;

//use std::fs::File;

#[derive(Debug, Deserialize)]
struct FileJS {
    file_id: String,
    file_size: i64,
    file_path: String,
}

#[derive(Debug, Deserialize)]
struct ResultFile {
    ok: bool,
    result: FileJS,
}

fn cmd_auth(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/auth").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user = match msg.from {
            Some(u) => u,
            None => return bot.message(admin, "Not a user. Error.".to_string()).send(),
        };

        if users_table.check_user(user.id) {
            return bot.message(user.id, "You already have access.".to_string()).send();
        }

        let user_name = format!("{} {} @{}, id: {}", user.first_name,
                                match user.last_name {
                                    Some(name) => name,
                                    None => String::from(""),
                                },
                                match user.username {
                                    Some(name) => name,
                                    None => String::from(""),
                                }, user.id
        );

        bot.message(admin, format!("User {} wants to auth", user_name)).send()
    });

    bot.register(handle);
}


fn cmd_add_user(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/adduser").and_then(move |(bot, mut msg)| {
        let mut users_table = a_users_table.lock().unwrap();
        let admin = users_table.get_admin() as i64;

        let sender = match msg.from {
            Some(data) => data.id,
            None => -1,
        };

        if !sender == admin {
            return bot.message(admin, format!("User {} tried to change users", sender)).send();
        }

        let _state = if let Some(text) = msg.text.take() {
            let mut user_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<i64>().ok());

            if let Some(user) = user_id.next() {
                //let mut user_table = &users_table;
                users_table.add_user(user);
                match database::write_database(&users_table) {
                    Ok(_) => return bot.message(admin, "Ok".to_string()).send(),
                    Err(err) => return bot.message(admin, format!("Error on writing config: {}", err)).send(),
                };
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}

fn cmd_add_token(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/addtoken").and_then(move |(bot, mut msg)| {
        let mut users_table = a_users_table.lock().unwrap();
        let admin = users_table.get_admin() as i64;

        let sender = match msg.from {
            Some(data) => data.id,
            None => -1,
        };

        if !sender == admin {
            return bot.message(admin, format!("User {} tried to change users", sender)).send();
        }

        let _state = if let Some(text) = msg.text.take() {
            let mut token_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

            if let Some(token) = token_id.next() {
                //let mut user_table = &users_table;
                users_table.add_token(token);
                match database::write_database(&users_table) {
                    Ok(_) => return bot.message(admin, "Ok".to_string()).send(),
                    Err(err) => return bot.message(admin, format!("Error on writing config: {}", err)).send(),
                };
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}

fn cmd_gen_token(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/gentoken").and_then(move |(bot, mut msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;

        let sender = match msg.from {
            Some(data) => data.id,
            None => -1,
        };

        if !sender == admin {
            return bot.message(admin, format!("User {} tried to change users", sender)).send();
        }

        let _state = if let Some(text) = msg.text.take() {
            let mut token_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

            if let Some(token) = token_id.next() {
                return bot.message(admin, format!("{}", generate_token(token))).send();
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}


fn cmd_del_token(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/deltoken").and_then(move |(bot, mut msg)| {
        let mut users_table = a_users_table.lock().unwrap();
        let admin = users_table.get_admin() as i64;

        let sender = match msg.from {
            Some(data) => data.id,
            None => -1,
        };

        if !sender == admin {
            return bot.message(admin, format!("User {} tried to change users", sender)).send();
        }

        let _state = if let Some(text) = msg.text.take() {
            let mut token_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

            if let Some(token) = token_id.next() {
                //let mut user_table = &users_table;
                users_table.del_token(token);
                match database::write_database(&users_table) {
                    Ok(_) => return bot.message(admin, "Ok".to_string()).send(),
                    Err(err) => return bot.message(admin, format!("Error on writing config: {}", err)).send(),
                };
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}


fn cmd_del_user(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/deluser").and_then(move |(bot, mut msg)| {
        let mut users_table = a_users_table.lock().unwrap();
        let admin = users_table.get_admin() as i64;

        let sender = match msg.from {
            Some(data) => data.id,
            None => -1,
        };

        if !sender == admin {
            return bot.message(admin, format!("User {} tried to change users", sender)).send();
        }

        let _state = if let Some(text) = msg.text.take() {
            let mut user_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<i64>().ok());

            if let Some(user) = user_id.next() {
                users_table.del_user(user);

                match database::write_database(&users_table) {
                    Ok(_) => return bot.message(admin, "Ok".to_string()).send(),
                    Err(err) => bot.message(admin, format!("Error on writing config: {}", err)).send(),
                };
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}


fn cmd_users(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/users").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to see users", user_id)).send();
        }


        bot.message(admin, format!("{:?}", users_table.vectorize().users)).send()
    });

    bot.register(handle);
}


fn cmd_tokens(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/tokens").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to see users", user_id)).send();
        }


        bot.message(admin, format!("{:?}", users_table.vectorize().mail_tokens)).send()
    });

    bot.register(handle);
}


fn cmd_print(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/print").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to print", user_id)).send();
        }

        let text = match msg.text {
            Some(data) => data,
            None => return bot.message(admin, "No text error".to_string()).send(),
        };

        let mut itr_filename = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

        let filename = match itr_filename.next() {
            Some(data) => data,
            None => return bot.message(admin, "No filename was specified. Error".to_string()).send()
        };

        match printer::print_from_file(&filename) {
            Ok(_state) => {
                return bot.message(admin, "The file was printed and cleaned successfully.".to_string()).send();
            }
            Err(err) => return bot.message(admin, format!("Error on printing the file: {}", err)).send()
        }
    });

    bot.register(handle);
}


fn cmd_files(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/files").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to see files", user_id)).send();
        }

        match printer::get_files() {
            Ok(data) => return bot.message(admin, data).send(),
            Err(err) => return bot.message(admin, format!("Error on getting files: {}", err)).send(),
        }
    });

    bot.register(handle);
}


fn cmd_get_file(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/getfile").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let token = &bot.inner.key;
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to get file", user_id)).send();
        }

        let text = match msg.text {
            Some(data) => data,
            None => return bot.message(admin, "No text error".to_string()).send(),
        };

        let mut itr_file_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

        let filename = match itr_file_id.next() {
            Some(data) => data,
            None => return bot.message(admin, "No filename was specified. Error".to_string()).send()
        };

        match send_document(&token, admin, &filename) {
            Ok(_) => print!(""),
            Err(err) => eprintln!("{:?}", err),
        };

        bot.message(admin, format!("/|\\ Your file\n | ")).send()
    });

    bot.register(handle);
}


fn cmd_delete_file(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/delfile").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to delete file", user_id)).send();
        }

        let text = match msg.text {
            Some(data) => data,
            None => return bot.message(admin, "No text error".to_string()).send(),
        };

        let mut itr_filename = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

        let filename = match itr_filename.next() {
            Some(data) => data,
            None => return bot.message(admin, "No filename was specified. Error".to_string()).send()
        };


        match printer::delete_file(&filename) {
            Ok(_) => bot.message(admin, "Ok".to_string()).send(),
            Err(err) => bot.message(admin, format!("Error on deleting file: {}", err)).send()
        }
    });

    bot.register(handle);
}


fn cmd_lpstat(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/lpstat").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to use lpstat", user_id)).send();
        }

        bot.message(admin, printer::lpstat()).send()
    });

    bot.register(handle);
}


fn cmd_cancel(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/cancel").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        if user_id != admin {
            return bot.message(admin, format!("{} tried to use cancel", user_id)).send();
        }

        let text = match msg.text {
            Some(data) => data,
            None => return bot.message(admin, "No text error".to_string()).send(),
        };

        let mut itr_job = text.split_whitespace().take(1).filter_map(|x| x.parse::<String>().ok());

        let job_name: String = match itr_job.next() {
            Some(data) => data.to_string(),
            None => return bot.message(admin, "No job was specified. Error".to_string()).send()
        };


        match printer::cancel(&job_name) {
            Ok(_) => bot.message(admin, "The job was canceled successfully.".to_string()).send(),
            Err(err) => bot.message(admin, format!("Error on canceling the job:\n{}", err)).send(),
        }
    });

    bot.register(handle);
}


fn cmd_help(bot: &RcBot, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let handle = bot.new_cmd("/help").and_then(move |(bot, msg)| {
        let users_table = { a_users_table.lock().unwrap().clone() };
        let admin = users_table.get_admin() as i64;
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return bot.message(admin, "Some error with user_id".to_string()).send(),
        };

        let helper = "
Use the commands below to manage users, tokens, files and printer:
* `/adduser <user_id>` -  add user to the access list
* `/addtoken <token>` - add token to the access list
* `/gentoken <name>` - generate token
* `/deluser <user_id>` - delete user from the access list
* `/deltoken <token>` - delete token from the access list
* `/print <filename>` - print the file
* `/users` - get users list
* `/tokens` - get tokens list
* `/files` - get files list
* `/getfile <filename>` - get file
* `/delfile <filename>` - delete file
* `/lpstat` - see lpstat output
* `/cancel <job ID or name>` - cancel the job
* `/help` - print the list of commands above";

        if user_id == admin {
            bot.message(admin, helper.to_string()).send()
        } else if users_table.check_user(user_id) {
            bot.message(user_id, "Just send PDF file".to_string()).send()
        } else {
            bot.message(user_id, "You must authenticate by `/auth` command.".to_string()).send()
        }
    });

    bot.register(handle);
}


/// Sends message
///
/// Needs Telegram Bot API token, chat_id and text
///
/// # Example
///
/// ```rust
/// send_message(&bot_token, admin, &format!("Mail user {} wants to print:", user_token)).unwrap();
/// ```
pub fn send_message(token: &String, chat_id: i64, text: &String) -> Result<(), String> {
    let mut url = match reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/sendMessage", token)) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(format!("{:?}", err));
        }
    };
    url.query_pairs_mut().append_pair("chat_id", format!("{}", chat_id).as_str()).append_pair("text", text.as_str());
    let response = reqwest::get(url.as_str());
    println!("{:?}", response);
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{:?}", err)),
    }
}

/// Sends message
///
/// Needs Telegram Bot API token, chat_id and filename
///
/// # Example
///
/// ```rust
/// send_document(&bot_token, admin, &format!("{}", filename)).unwrap();
/// ```
pub fn send_document(token: &String, chat_id: i64, filename: &String) -> Result<(), String> {
    let mut url = match reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/sendDocument", token)) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(format!("{:?}", err));
        }
    };
    let client = reqwest::Client::new();

    let form = match reqwest::multipart::Form::new().file("document", filename) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(format!("{:?}", err));
        }
    };

    url.query_pairs_mut().append_pair("chat_id", format!("{}", chat_id).as_str());

    let response = client.post(url).multipart(form).send();

    //let response = reqwest::get(url.as_str());
    println!("{:?}", response);
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{:?}", err)),
    }
}


fn get_link(token: &String, file_id: String) -> Result<String, String> {
    let mut url = match reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/getFile", token)) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(format!("{:?}", err));
        }
    };
    url.query_pairs_mut().append_pair("file_id", file_id.as_str());
    let response = reqwest::get(url.as_str());
    println!("{:?}", response);
    let mut result = match response {
        Ok(res) => res,
        Err(err) => return Err(format!("{:?}", err)),
    };
    let js_res = match result.json::<ResultFile>() {
        Ok(data) => data,
        Err(err) => return Err(format!("{:?}", err)),
    };

    Ok(js_res.result.file_path)
}


/// Runs Telegram bot
///
/// You should provide Config and UsersTable as shared state `Arc<Mutex>`
///
/// # Examples
///
/// ```rust
/// let users_table = Arc::new(Mutex::new(read_users().unwrap()));
/// let config = Arc::new(Mutex::new(read_config().unwrap()));
/// let tele_bot = thread::spawn(move || {
///        run_bot(Arc::clone(&config), Arc::clone(&users_table));
///  });
/// ```
pub fn run_bot(a_config: Arc<Mutex<config::Config>>, a_users_table: Arc<Mutex<database::UsersTable>>) {
    let mut lp = Core::new().unwrap();
    let config = { a_config.lock().unwrap().clone() };
    let bot: RcBot = RcBot::new(lp.handle(), &config.token).update_interval(1000);

    cmd_auth(&bot, Arc::clone(&a_users_table)); //          /auth
    cmd_add_user(&bot, Arc::clone(&a_users_table)); //      /adduser
    cmd_add_token(&bot, Arc::clone(&a_users_table)); //     /addtoken
    cmd_gen_token(&bot, Arc::clone(&a_users_table)); //     /gentoken
    cmd_del_user(&bot, Arc::clone(&a_users_table)); //      /deluser
    cmd_del_token(&bot, Arc::clone(&a_users_table)); //     /deltoken
    cmd_print(&bot, Arc::clone(&a_users_table)); //         /print
    cmd_users(&bot, Arc::clone(&a_users_table)); //         /users
    cmd_tokens(&bot, Arc::clone(&a_users_table)); //        /tokens
    cmd_files(&bot, Arc::clone(&a_users_table)); //         /files
    cmd_get_file(&bot, Arc::clone(&a_users_table)); //      /getfile
    cmd_delete_file(&bot, Arc::clone(&a_users_table)); //   /delfile
    cmd_lpstat(&bot, Arc::clone(&a_users_table)); //        /lpstat
    cmd_cancel(&bot, Arc::clone(&a_users_table)); //        /cancel
    cmd_help(&bot, Arc::clone(&a_users_table)); //          /help
    // cmd_from_file(&bot);

    let handle = (&bot).get_stream().and_then(|(bot, upd)| {
        let user_table = { a_users_table.lock().unwrap().clone() };
        let admin = user_table.get_admin() as i64;
        let tg_token = &bot.inner.key;

        let msg = match upd.message {
            Some(data) => data,
            None => return None,
        };
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return Some(bot.message(admin, "Some error with user_id".to_string()).send()),
        };

        if !user_table.check_user(user_id) {
            return Some(bot.message(user_id, "You don't have access to printer.".to_string()).send());
        }

        let file_id = match msg.document {
            Some(data) => data.file_id,
            None => return Some(bot.message(user_id, "Error: no file or unknown command".to_string()).send()),
        };

        let link = match get_link(&tg_token, file_id.clone()) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error on getting file path: {}", err);
                return None;
            }
        };


        let filename = match downloader::download_from_url(&format!("https://api.telegram.org/file/bot{}/{}", tg_token, link)) {
            Ok(data) => data,
            Err(err) => return Some(bot.message(admin, format!("Error in downloading file: {:?}", err)).send()),
        };

        match send_message(&tg_token.clone(), admin, &format!("User {} wants to print:\n{}", user_id, filename)) {
            Ok(_) => println!("Ok"),
            Err(err) => eprintln!("Error on sending message: {:?}", err),
        };

        Some(bot.message(admin, format!("{}", filename)).send())
        // bot.message(admin, format!("{}", filename)).send()
    });

    match lp.run(handle.for_each(|_| Ok(())).into_future()) {
        Ok(_) => println!("Ok"),
        Err(err) => eprintln!("{:?}", err),
    };
    //bot.register(handle);

    bot.run(&mut lp).unwrap();
}