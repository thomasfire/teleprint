extern crate toml;

use std::collections::HashSet;
use std::iter::FromIterator;

use io_tools;

/// Structure, that contains admin ID, vector of users and vector of mail tokens.
/// Usable with TOML
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Users {
    pub users: Vec<i64>,
    pub admin: i64,
    pub mail_tokens: Vec<String>,
}


/// Structure, that contains admin ID and HashSets of users and mail tokens.
/// Usable on working with users/tokens.
#[derive(Clone, Debug)]
pub struct UsersTable {
    users: HashSet<i64>,
    admin: i64,
    mail_tokens: HashSet<String>,
}


impl UsersTable {
    /// Adds Telegram user
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut users = read_users().unwrap();
    /// users.add_user(123456);
    /// ```
    pub fn add_user(&mut self, user_id: i64) {
        self.users.insert(user_id);
    }

    /// Deletes Telegram user
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut users = read_users().unwrap();
    /// users.del_user(123456);
    /// ```
    pub fn del_user(&mut self, user_id: i64) {
        self.users.remove(&user_id);
    }

    /// Adds IMAP token
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut users = read_users();
    /// users.add_token("tokenONE".to_string());
    /// ```
    pub fn add_token(&mut self, token: String) {
        self.mail_tokens.insert(token);
    }

    /// Deletes IMAP token
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut users = read_users();
    /// users.del_token("tokenONE".to_string());
    /// ```
    pub fn del_token(&mut self, token: String) {
        self.mail_tokens.remove(&token);
    }

    /// Checks whether the Telegram user is authorized
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut users = read_users().unwrap();
    /// users.add_user(123456);
    /// let accessed = users.check_user(123456); // true
    /// ```
    pub fn check_user(&self, user_id: i64) -> bool {
        self.users.contains(&user_id)
    }


    /// Checks whether the IMAP token is authorized
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut users = read_users().unwrap();
    /// users.add_token("tokenONE".to_string());
    /// let accessed = users.check_token("tokenONE".to_string()); // true
    /// ```
    pub fn check_token(&self, token: String) -> bool {
        self.mail_tokens.contains(&token)
    }

    /// Sets admin ID (Telegram)
    pub fn set_admin(&mut self, admin_id: i64) {
        self.admin = admin_id;
    }

    /// Converts UsersTable to Users with vectors instead of HashSets
    ///
    /// # Examples
    ///
    /// ```rust
    /// let users = read_users().unwrap();
    /// println!("{:?}", users); // You will see HashSets
    /// println!("{:?}", users.vectorize()); // You will see vectors, usable for TOML
    /// ```
    pub fn vectorize(&self) -> Users {
        let mut users = Users { users: vec![], admin: self.admin, mail_tokens: vec![] };
        for user in &self.users {
            users.users.push(*user);
        }

        for token in &self.mail_tokens {
            users.mail_tokens.push(token.to_string());
        }

        users
    }

    /// Returns admin ID
    pub fn get_admin(&self) -> i64 {
        self.admin
    }
}


fn hashify<T>(data: Vec<T>) -> HashSet<T> where T: std::hash::Hash + std::clone::Clone + std::cmp::Eq {
    HashSet::from_iter(data.iter().cloned())
}


/// Reads `users.toml` and returns Result with Users on Ok()
///
/// # Examples
///
/// ```rust
/// let users = read_users().unwrap();
/// ```
pub fn read_users() -> Result<UsersTable, String> {
    if !io_tools::exists("users.toml") {
        println!("No `users.toml` file, creating it...");
        write_config(&UsersTable {
            users: hashify(vec![]),
            admin: 0,
            mail_tokens: hashify(vec![]),
        }).unwrap();
    }
    let users_str = io_tools::read_str("users.toml");
    let users: Users = match toml::from_str(&users_str) {
        Ok(value) => value,
        Err(err) => {
            println!("Something goes wrong while reading the users: {}", err);
            return Err(format!("{:?}", err));
        }
    };

    let user_table = UsersTable {
        users: hashify(users.users),
        admin: users.admin,
        mail_tokens: hashify(users.mail_tokens),
    };
    Ok(user_table)
}


/// Writes Config to the `users.toml`, returns Result
///
/// # Examples
///
/// ```rust
/// let users = Users {
///     users: [45454. 911],
///     admin: 1000,
///     mail_tokens: ["tokenONE", "sfs66fsdf"]
/// };
/// write_config(users).unwrap();
/// ```
pub fn write_config(users: &UsersTable) -> Result<(), String> {
    let users_str = match toml::to_string(&users.vectorize()) {
        Ok(value) => value,
        Err(err) => {
            println!("Something went wrong while parsing the config: {}", err);
            panic!("{}", err);
        }
    };


    match io_tools::write_to_file("users.toml",
                                  users_str) {
        Ok(_) => return Ok(()),
        Err(err) => {
            println!("An error occured while writing to the config: {}", err);
            return Err(format!("{:?}", err));
        }
    };
}
