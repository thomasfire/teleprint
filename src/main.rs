extern crate teleprint;

use std::env;

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
    teleprint::bot::run_bot();
}
