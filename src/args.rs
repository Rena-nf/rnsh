use std::path::PathBuf;

use clap::{command, value_parser, Arg};

pub fn get_args() {
    let config = Arg::new("config")
        .short('c')
        .long("config")
        .help("Load config file")
        .long_help("Load config file, default to \"rnsh.config.toml\"")
        .value_parser(value_parser!(PathBuf));

    let matches = command!().arg(config).get_matches();

    if let Some(c) = matches.get_one::<String>("config") {
        println!("Value for config is : {}", c);
    }
}
