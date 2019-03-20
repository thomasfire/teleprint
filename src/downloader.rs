extern crate reqwest;
extern crate thread_tryjoin;

use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use self::thread_tryjoin::TryJoinHandle;

use io_tools::write_bytes_to_file;
use hash;

/// Downloads (pdf) file from url, returns filename if Ok().
///
/// File is saved to the <SHA1 of the file>.pdf, such as "4e24631bfb9aaa3fbf8b4dc9b549de1dec0c8b4a.123456"
///
/// # Examples
///
/// ```rust
/// let result = download_from_url("http://edu.ifmo.ru/file/subspec/3143/up_09.03.04_nip.pdf", Some(123456));
/// match result {
///     Ok(filename) => println("{}", filename), // 28f158d186263820686c8341d510595ace3b27ce.123456
///     Err(err) => eprintln("{}", err),
/// };
/// ```
pub fn download_from_url(url: &str, user_id: Option<i64>) -> Result<String, String> {
    let content = Arc::new(Mutex::new(Vec::new()));

    let extension = match user_id {
        Some(v) => format!("{}", v),
        None => "pdf".to_string(),
    };

    let url_copy = url.to_string();
    let content_a = Arc::clone(&content);

    let downloader = thread::spawn(move || {
        let mut resp = match reqwest::get(&url_copy) {
            Ok(data) => data,
            Err(err) => return Err(format!("{:?}", err)),
        };

        match resp.read_to_end(&mut content_a.lock().unwrap()) {
            Ok(_) => println!("Downloaded file"),
            Err(err) => return Err(format!("Error on downloading to file: {}", err))
        };
        Ok("Ok downloaded")
    });

    if downloader.try_timed_join(Duration::from_secs(10)).is_err() {
        return Err("Downloading is too slow, or there was an error".to_string());
    }

    let cont = content.lock().unwrap().clone();
    let hashsum = hash::hash_data(&cont);

    let filename = format!("{}.{}.pdf", hashsum, extension);

    match write_bytes_to_file(&filename, cont) {
        Ok(_) => Ok(filename.clone()),
        Err(err) => Err(format!("Error on writing to file: {}", err)),
    }
}

