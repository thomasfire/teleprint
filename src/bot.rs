extern crate failure;
extern crate futures;
extern crate telebot;
extern crate tokio_core;

use std::option::Option;

use config::read_config;
use database;
use printer;

use self::failure::Error;
use self::futures::Future;
use self::futures::stream::{AndThen, OrElse, Stream, Then};
use self::telebot::functions::*;
use self::telebot::RcBot;
use self::tokio_core::reactor::Core;

enum NotAUser {
    Telegram(Error),
    NotAUserError,
}

fn cmd_auth(bot: &RcBot) {
    let handle = bot.new_cmd("/auth").and_then(|(bot, msg)| {
        let admin = database::read_users().unwrap().get_admin() as i64;
        let user = match msg.from {
            Some(u) => Ok(u),
            None => Err(NotAUser::NotAUserError),
        };

        let user_name = match user {
            Ok(u) => format!("{} {} @{}, id: {}", u.first_name,
                             match u.last_name {
                                 Some(name) => name,
                                 None => String::from(""),
                             },
                             match u.username {
                                 Some(name) => name,
                                 None => String::from(""),
                             }, u.id
            ),
            Err(_err) => format!("Not a user. Error."),
        };

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

        let state = if let Some(text) = msg.text.take() {
            let mut user_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<i64>().ok());

            if let Some(user) = user_id.next() {
                let mut user_table = database::read_users().unwrap();
                user_table.add_user(user);
                database::write_config(&user_table);
                return bot.message(admin, "Ok".to_string()).send();
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

        let state = if let Some(text) = msg.text.take() {
            let mut user_id = text.split_whitespace().take(1).filter_map(|x| x.parse::<i64>().ok());

            if let Some(user) = user_id.next() {
                let mut user_table = database::read_users().unwrap();
                user_table.del_user(user);
                database::write_config(&user_table);
                return bot.message(admin, "Ok".to_string()).send();
            }
        };

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}

fn cmd_from_file(bot: &RcBot) {
    let handle = bot.unknown_cmd().and_then(|(bot, mut msg)| {
        let admin = database::read_users().unwrap().get_admin() as i64;
        

        bot.message(admin, "Error".to_string()).send()
    });

    bot.register(handle);
}

pub fn run_bot() {
    let mut lp = Core::new().unwrap();
    let config = read_config().unwrap();
    let bot: RcBot = RcBot::new(lp.handle(), &config.token).update_interval(1000);

    cmd_auth(&bot);
    cmd_add_user(&bot);
    cmd_del_user(&bot);
    bot.run(&mut lp).unwrap();
}