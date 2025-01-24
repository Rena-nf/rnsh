use clap::Parser;

const ABOUT: &str = "A simple shell made in rust made in boredom";

#[derive(Parser, Debug)]
#[command(version, about, long_about = ABOUT)]
pub struct Args {
    #[arg(short, long, default_value = None)]
    pub config: Option<std::path::PathBuf>,
    #[arg(short, long, default_value = None)]
    pub enable_history: Option<bool>,
}
