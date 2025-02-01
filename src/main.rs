use clap::Parser;
use rnsh::{args, start_loop, users::Users};

fn main() {
    #[allow(unused_variables)]
    let args: args::Args = args::Args::parse();
    let user: Users = Users::new();

    start_loop(user);
}
