use clap::Parser;
use serde::Deserialize;

const ABOUT: &str = "A simple shell made in rust made in boredom";

/// To print help and handle some config option
#[derive(Parser, Debug, Deserialize)]
#[command(version, about, long_about = ABOUT)]
pub struct Args {
    #[arg(short, long, default_value = None)]
    pub config: Option<String>,
    #[arg(short, long, default_value = None)]
    pub enable_history: Option<bool>,
}

impl Args {}
