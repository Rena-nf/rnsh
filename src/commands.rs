use std::{
    env,
    path::{self, Path},
    str::SplitWhitespace,
};

/// A built-in command to handle directory changes
/// As explained in [here](https://unix.stackexchange.com/questions/38808/why-is-cd-not-a-program/38819#38819)
/// Cd shouldn't be a program because it handles the shell state
// Cd is needed as a built-in command
// Because it handles the shell state
pub fn cd(args: SplitWhitespace) {
    let new_dir: &str = args.peekable().peek().map_or("/", |x| *x);
    let root: &Path = path::Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(root) {
        eprintln!("{}", e);
    }
}
