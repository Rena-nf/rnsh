use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use whoami::fallible;

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub username: String,
    pub hostname: String,
    pub home_dir: PathBuf,
}

#[doc(hidden)]
impl Users {
    pub fn new() -> Self {
        let hostname = match fallible::hostname() {
            Ok(x) => format!("@{}", x),
            Err(_) => String::from(""),
        };

        // Should default to root when home directory is missing
        // TODO: Test on windows
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

#[doc(hidden)]
// Not sure why i need this but i'm just following clippy
impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}
