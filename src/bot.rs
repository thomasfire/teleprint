extern crate futures;
extern crate reqwest;
extern crate telebot;
extern crate tokio_core;

use config::read_config;
use database;
use downloader;
use printer;

use self::futures::IntoFuture;
//use self::reqwest;
use self::futures::stream::Stream;
use self::telebot::RcBot;
use self::telebot::functions::*;
use self::tokio_core::reactor::Core;

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

fn cmd_auth(bot: &RcBot) {
    let handle = bot.new_cmd("/auth").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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


fn cmd_add_user(bot: &RcBot) {
    let handle = bot.new_cmd("/adduser").and_then(|(bot, mut msg)| {
        let admin = database::read_users().unwrap().get_admin() as i64;

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
                let mut user_table = database::read_users().unwrap();
                user_table.add_user(user);
                match database::write_config(&user_table) {
                    Ok(_) => return bot.message(admin, "Ok".to_string()).send(),
                    Err(err) => return bot.message(admin, format!("Error on writing config: {}", err)).send(),
                };
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}

fn cmd_del_user(bot: &RcBot) {
    let handle = bot.new_cmd("/deluser").and_then(|(bot, mut msg)| {
        let admin = database::read_users().unwrap().get_admin() as i64;

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
                let mut user_table = database::read_users().unwrap();
                user_table.del_user(user);

                match database::write_config(&user_table) {
                    Ok(_) => return bot.message(admin, "Ok".to_string()).send(),
                    Err(err) => bot.message(admin, format!("Error on writing config: {}", err)).send(),
                };
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}


fn cmd_users(bot: &RcBot) {
    let handle = bot.new_cmd("/users").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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


fn cmd_print(bot: &RcBot) {
    let handle = bot.new_cmd("/print").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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


fn cmd_files(bot: &RcBot) {
    let handle = bot.new_cmd("/files").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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


fn cmd_get_file(bot: &RcBot) {
    let handle = bot.new_cmd("/getfile").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
        let token = read_config().unwrap().token;
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

        let file_id = match itr_file_id.next() {
            Some(data) => data,
            None => return bot.message(admin, "No filename was specified. Error".to_string()).send()
        };

        //bot.document(admin).file(filename.as_str()).send()
        let mut url = reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/sendDocument", token)).unwrap();
        url.query_pairs_mut().append_pair("chat_id", format!("{}", admin).as_str())
            .append_pair("document", file_id.as_str());

        let response = reqwest::get(url.as_str());
        println!("{:?}", response);
        let result = match response {
            Ok(data) => data,
            Err(err) => return bot.message(admin, format!("Error on sending file: {:?}", err)).send(),
        };

        println!("{:?}", result);

        bot.message(admin, format!("/|\\ Your file\n | ")).send()
    });

    bot.register(handle);
}


fn cmd_delete_file(bot: &RcBot) {
    let handle = bot.new_cmd("/delfile").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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


fn cmd_lpstat(bot: &RcBot) {
    let handle = bot.new_cmd("/lpstat").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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


fn cmd_cancel(bot: &RcBot) {
    let handle = bot.new_cmd("/cancel").and_then(|(bot, msg)| {
        let users_table = database::read_users().unwrap();
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

fn send_message(token: String, chat_id: i64, text: String) -> Result<(), String> {
    let mut url = reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/sendMessage", token)).unwrap();
    url.query_pairs_mut().append_pair("chat_id", format!("{}", chat_id).as_str()).append_pair("text", text.as_str());
    let response = reqwest::get(url.as_str());
    println!("{:?}", response);
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{:?}", err)),
    }
}

fn get_link(token: &String, file_id: String) -> Result<String, String> {
    let mut url = reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/getFile", token)).unwrap();
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

pub fn run_bot() {
    let mut lp = Core::new().unwrap();
    let config = read_config().unwrap();
    let bot: RcBot = RcBot::new(lp.handle(), &config.token).update_interval(1000);

    cmd_auth(&bot);
    cmd_add_user(&bot);
    cmd_del_user(&bot);
    cmd_print(&bot);
    cmd_users(&bot);
    cmd_files(&bot);
    cmd_get_file(&bot);
    cmd_delete_file(&bot);
    cmd_lpstat(&bot);
    cmd_cancel(&bot);
    // cmd_from_file(&bot);

    let handle = (&bot).get_stream().and_then(|(bot, upd)| {
        let users_table = database::read_users().unwrap();
        let admin = users_table.get_admin() as i64;
        let tg_token = read_config().unwrap().token;

        let msg = match upd.message {
            Some(data) => data,
            None => return None,
        };
        let user_id = match msg.from {
            Some(data) => data.id,
            None => return Some(bot.message(admin, "Some error with user_id".to_string()).send()),
        };

        if !users_table.check_user(user_id) {
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

        match send_message(tg_token.clone(), admin, format!("User {} wants to print:\n{}", user_id, filename))  {
            Ok(_) => println!("Ok"),
            Err(err) => eprintln!("Error on sending message: {:?}", err),
        };

        match send_message(tg_token, admin, format!("{}", filename))  {
            Ok(_) => println!("Ok"),
            Err(err) => eprintln!("Error on sending message: {:?}", err),
        };


        Some(bot.message(admin, file_id).send())
        // bot.message(admin, format!("{}", filename)).send()
    });

    match lp.run(handle.for_each(|_| Ok(())).into_future()) {
        Ok(_) => println!("Ok"),
        Err(err) => eprintln!("{:?}", err),
    };
    //bot.register(handle);

    bot.run(&mut lp).unwrap();
}