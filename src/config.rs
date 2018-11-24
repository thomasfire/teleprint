extern crate toml;
use io_tools;


/// config.toml must contain line
///
/// ```toml
/// token = "TELEGRAM_BOT_KEY"
/// ```
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
}


/// Reads `config.toml` and returns Result with Users on Ok()
///
/// # Examples
///
/// ```rust
/// let users = read_users().unwrap();
/// ```
pub fn read_config() -> Result<Config, String> {
    if !io_tools::exists("config.toml") {
        panic!("No `config.toml` file, create it and write `token = \"TELEGRAM_BOT_KEY\"` ");
    }
    let config_str = io_tools::read_str("config.toml");
    let config: Config = match toml::from_str(&config_str) {
        Ok(value) => value,
        Err(err) => {
            println!("Something goes wrong while reading the users: {}", err);
            return Err(format!("{:?}", err));
        }
    };

    Ok(config)
}



