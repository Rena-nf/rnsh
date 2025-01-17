use std::{env, path, str::SplitWhitespace, sync::Arc};

// Cd is needed as a built-in command
// Because it handles the shell state
pub fn cd(args: SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = path::Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(root) {
        eprintln!("{}", e);
    }
}

pub fn alias(alias_list: &Arc<Vec<&str>>, args: &mut SplitWhitespace) {
    if args.find(|&x| x == "-p").is_some() {
        let _ = alias_list.iter().map(|&x| println!("{}", x));
    }

    let aliases = Arc::clone(alias_list);
}
