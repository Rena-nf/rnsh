use std::{
    env,
    path::{self, Path},
    str::SplitWhitespace,
    sync::Arc,
};

// Cd is needed as a built-in command
// Because it handles the shell state
pub fn cd(args: SplitWhitespace) {
    let new_dir: &str = args.peekable().peek().map_or("/", |x| *x);
    let root: &Path = path::Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(root) {
        eprintln!("{}", e);
    }
}

pub fn alias(alias_list: &Arc<Vec<&str>>, args: &mut SplitWhitespace) {
    if args.any(|x| x == "-p") {
        let _ = alias_list.iter().map(|&x| println!("{}", x));
    }
}
