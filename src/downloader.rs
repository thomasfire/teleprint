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
    resp.read_to_end(&mut content);
    let hashsum = hash::hash_data(&content);

    write_bytes_to_file(&format!("{}.pdf", hashsum), content);

    Ok(format!("{}.pdf", hashsum))
}

