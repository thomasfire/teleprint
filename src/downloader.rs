extern crate reqwest;

use std::io::Read;
use::io_tools::write_bytes_to_file;
use hash;

/// Downloads (pdf) file from url, returns filename if Ok().
///
/// File is saved to the <SHA1 of the file>.pdf, such as "4e24631bfb9aaa3fbf8b4dc9b549de1dec0c8b4a.pdf"
///
/// # Examples
///
/// ```rust
/// let result = download_from_url("http://edu.ifmo.ru/file/subspec/3143/up_09.03.04_nip.pdf");
/// match result {
///     Ok(filename) => println("{}", filename), // 28f158d186263820686c8341d510595ace3b27ce.pdf
///     Err(err) => eprintln("{}", err),
/// };
/// ```
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

