use std::{fs, process::exit};

use crate::instance::ShellInstance;

pub fn open_config_toml(config_path: Option<String>) -> ShellInstance {
    let file_path = config_path.unwrap_or("rnsh.config.toml".to_owned());
    let contents = match fs::read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Failed to read from {}", &file_path);
            exit(1);
        }
    };

    let data: ShellInstance = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Failed to parse from {}", &file_path);
            exit(1);
        }
    };

    data
}
