use std::path::PathBuf;

use serde::Serialize;
use whoami::fallible;

#[derive(Serialize)]
pub struct Users {
    pub username: String,
    pub hostname: String,
    pub home_dir: PathBuf,
}

impl Users {
    pub fn new() -> Self {
        let hostname = match fallible::hostname() {
            Ok(x) => format!("@{}", x),
            Err(_) => String::from(""),
        };

        let home = match homedir::my_home() {
            Ok(x) => x.unwrap(),
            Err(_) => {
                if cfg!(windows) {
                    PathBuf::from("C:\\")
                } else {
                    PathBuf::from("/")
                }
            }
        };

        Users {
            username: whoami::username(),
            hostname,
            home_dir: home,
        }
    }
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}
