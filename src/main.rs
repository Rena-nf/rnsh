use clap::Parser;
use rnsh::{args, start_loop, users::Users};

/// A Simple shell / command spawn
///
/// Currently it's missing a lot of feature that's generally supported in other browser
/// ##### There shouldn't be that many breaking changes but it's possible
///
/// to run it you can either do cargo run / cargo build and then do rnsh (-h to get help)
fn main() {
    // TODO : Implement args function
    #[allow(unused_variables)]
    let args: args::Args = args::Args::parse();
    let user: Users = Users::new();

    start_loop(user);
}
