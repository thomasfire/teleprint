extern crate sha1;

use std::io::Read;
use std::fs::File;
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