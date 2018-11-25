extern crate reqwest;

use std::io::Read;
use::io_tools::write_bytes_to_file;
use hash;

pub fn download_from_url(url: &str) -> Result<String, String> {
    let mut resp = match reqwest::get(url) {
        Ok(data) => data,
        Err(err) => return Err(format!("{:?}", err)),
    };

    let mut content: Vec<u8> = Vec::new();
    match resp.read_to_end(&mut content) {
        Ok(_) => println!("Downloaded file"),
        Err(err) => return Err(format!("Error on downloading to file: {}", err))
    };
    let hashsum = hash::hash_data(&content);

    match write_bytes_to_file(&format!("{}.pdf", hashsum), content) {
        Ok(_) => Ok(format!("{}.pdf", hashsum)),
        Err(err) => Err(format!("Error on writing to file: {}", err)),
    }


}

