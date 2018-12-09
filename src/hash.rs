extern crate rand;
extern crate sha1;

use std::fs::File;
use std::io::Read;
use self::rand::prelude::*;
use self::sha1::Sha1;

fn read_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer: Vec<u8> = vec![];
    f.read_to_end(&mut buffer).expect("Couldn`t read to string");
    buffer
}

pub fn hash_file(filename: &str) -> String {
    let mybytes = read_bytes(&filename);
    let mut hasher = Sha1::new();
    hasher.update(&mybytes);
    hasher.digest().to_string()
}


pub fn hash_data(data: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.digest().to_string()
}

pub fn generate_token(name: String) -> String {
    let mut rng = rand::thread_rng();
    let mut data = Vec::from(name.clone());
    for _ in 0..64 {
        data.push(rng.gen::<u8>());
    }
    format!("{}{}", &name, hash_data(&data))
}