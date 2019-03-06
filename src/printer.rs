use std::fs::{read_dir, remove_file};
use std::process::Command;
use std::sync::{Arc, Mutex};

use config;

/// Deletes file by filename
pub fn delete_file(filename: &str) -> Result<String, String> {
    match remove_file(filename) {
        Ok(_) => return Ok("Ok".to_string()),
        Err(err) => return Err(format!("Error: {:?}", err)),
    }
}


/// Prints file by filename via lp (on *nix only)
pub fn print_from_file(filename: &str, a_config: Arc<Mutex<config::Config>>) -> Result<String, String> {
    let config = { a_config.lock().unwrap().clone() };

    let _printing_process = match Command::new("lp")
        .args(&["-d", &config.printer, filename]).spawn() {
        Ok(child) => child,
        Err(err) => return Err(format!("Error running the printing process (lp): {}", err)),
    };

    Ok("Ok".to_string())
}

/// Returns output of the `$ lpstat` command
pub fn lpstat() -> String {
    match Command::new("lpstat")
        .output() {
        Ok(outp) => format!("lpstat:\n{}", String::from_utf8_lossy(&outp.stdout)),
        Err(err) => format!("lpstat error:\n{}", err),
    }
}


/// Returns output of the `$ lpstat -p` command
pub fn get_printers() -> String {
    match Command::new("lpstat").arg("-p")
        .output() {
        Ok(outp) => format!("lpstat -p:\n{}", String::from_utf8_lossy(&outp.stdout)),
        Err(err) => format!("lpstat error:\n{}", err),
    }
}

/// Cancels the job by its name or number
pub fn cancel(job: &str) -> Result<String, String> {
    let output = match Command::new("cancel")
        .arg(job)
        .output() {
        Ok(outp) => String::from(String::from_utf8_lossy(&outp.stdout)),
        Err(err) => return Err(format!("lprm error:\n {}", err)),
    };

    if output.len() < 3 {
        return Ok("Ok".to_string());
    }
    return Err(format!("Error on cancel: {}", output));
}

/// Returns list of pdf files, ready for sending to the Telegram or email
pub fn get_files() -> Result<String, String> {
    let entries = match read_dir(".") {
        Ok(data) => data,
        Err(err) => return Err(format!("Error on getting the list of the files: {}", err)),
    };

    let mut v_entries: Vec<String> = vec![];
    v_entries.push("Files:".to_string());
    for entry in entries {
        match entry {
            Ok(data) => {
                let extension = match data.path().extension() {
                    Some(path) => String::from(path.to_string_lossy()),
                    None => "".to_string()
                };
                if extension == "pdf".to_string() || extension.trim().parse::<i64>().is_ok() {
                    v_entries.push(String::from(data.file_name().to_string_lossy()));
                }
            }
            Err(_) => continue,
        };
    }

    Ok(v_entries.join("\n").to_string())
}