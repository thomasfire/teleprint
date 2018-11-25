extern crate toml;
use io_tools;
use printer::get_printers;


/// config.toml must contain line
///
/// ```toml
/// token = "TELEGRAM_BOT_KEY"
/// ```
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub printer: String,
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
        panic!("No `config.toml` file, run `$ teleprint --setup` ");
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



/// Writes Config to the `config.toml`, returns Result
///
/// # Examples
///
/// ```rust
/// let config = Config {
///     token: String::from("ava24efsef345"),
///     printer: String::from("Your-Printer"),
/// };
/// write_config(config).unwrap();
/// ```
pub fn write_config(config: &Config) -> Result<(), String> {
    let conf_str = match toml::to_string(config) {
        Ok(value) => value,
        Err(err) => {
            println!("Something went wrong while parsing the config: {}", err);
            panic!("{}", err);
        }
    };


    match io_tools::write_to_file("config.toml", conf_str) {
        Ok(_) => return Ok(()),
        Err(err) => {
            println!("An error occured while writing to the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };
}


pub fn setup() {
    let m_token = io_tools::read_std_line("Enter Telegram API token: ");
    println!("\nHere is your printers:\n{}\n", get_printers());
    let m_printer = io_tools::read_std_line("Enter name of the printer: ");

    match write_config(&Config {token: m_token, printer: m_printer}) {
        Ok(_) => println!("Ok"),
        Err(err) => panic!("{:?}", err),
    };
}