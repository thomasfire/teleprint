extern crate teleprint;

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

use teleprint::config::read_config;
use teleprint::database::read_users;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--setup" => {
                teleprint::config::setup();
                return;
            }
            _ => {
                println!("Unknown argument, exiting");
                return;
            }
        }
    }
    let users_table = Arc::new(Mutex::new(read_users().unwrap()));
    let config = Arc::new(Mutex::new(read_config().unwrap()));

    let a_config = Arc::clone(&config);
    let a_users = Arc::clone(&users_table);

    let tele_bot = thread::spawn(move || {
        teleprint::bot::run_bot(Arc::clone(&config),
                                Arc::clone(&users_table));
    });
    let imap_bot = thread::spawn(move || {
        teleprint::mailbot::run_bot(Arc::clone(&a_config),
                                    Arc::clone(&a_users));
    });

    println!("{:?}\n\n{:?}", tele_bot.join(), imap_bot.join());
}
