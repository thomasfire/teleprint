extern crate toml;

use std::collections::HashSet;
use std::iter::FromIterator;

use io_tools;

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub users: Vec<i64>,
    pub admin: i64,
}

pub struct UsersTable {
    users: HashSet<i64>,
    admin: i64,
}


impl UsersTable {
    pub fn add_user(&mut self, user_id: i64) {
        self.users.insert(user_id);
    }

    pub fn del_user(&mut self, user_id: i64) {
        self.users.remove(&user_id);
    }

    pub fn check_user(&self, user_id: i64) -> bool {
        self.users.contains(&user_id)
    }

    pub fn set_admin(&mut self, admin_id: i64) {
        self.admin = admin_id;
    }

    pub fn vectorize(&self) -> Users {
        let mut users = Users { users: vec![], admin: self.admin };
        for user in &self.users {
            users.users.push(*user);
        }

        users
    }

    pub fn get_admin(&self) -> i64 {
        self.admin
    }
}

impl Clone for UsersTable {
    fn clone(&self) -> UsersTable {
        UsersTable { users: self.users.clone(), admin: self.admin.clone() }
    }
}

fn hashify(data: Vec<i64>) -> HashSet<i64> {
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
        write_config(&UsersTable { users: hashify(vec![]), admin: 0 }).unwrap();
    }
    let users_str = io_tools::read_str("users.toml");
    let users: Users = match toml::from_str(&users_str) {
        Ok(value) => value,
        Err(err) => {
            println!("Something goes wrong while reading the users: {}", err);
            return Err(format!("{:?}", err));
        }
    };

    let user_table = UsersTable { users: hashify(users.users), admin: users.admin };
    Ok(user_table)
}


/// Writes Config to the `users.toml`, returns Result
///
/// # Examples
///
/// ```rust
/// let users = Users {
///     users: [45454. 911],
///     admin: 1000
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
