extern crate teleprint;

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

use teleprint::config::read_config;
use teleprint::database::read_users;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut run_imap: bool = true;
    if args.len() > 1 {
        match args[1].as_str() {
            "--setup" => {
                teleprint::config::setup();
                return;
            }
            "--noimap" => run_imap = false,
            _ => {
                println!("Unknown argument, exiting");
                return;
            }
        }
    }
    let users_table = Arc::new(Mutex::new(read_users().unwrap()));
    let config = Arc::new(Mutex::new(read_config().unwrap()));

    if run_imap {
        let a_config = Arc::clone(&config);
        let a_users = Arc::clone(&users_table);
        let _imap_bot = thread::spawn(move || {
            teleprint::mailbot::run_bot(Arc::clone(&a_config),
                                        Arc::clone(&a_users));
        });
    }

    let tele_bot = thread::spawn(move || {
        teleprint::bot::run_bot(Arc::clone(&config),
                                Arc::clone(&users_table));
    });

    println!("{:?}", tele_bot.join());
}
