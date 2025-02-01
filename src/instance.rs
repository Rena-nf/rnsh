use std::iter::Map;

use crate::{alias::Alias, args::Args, users::Users};

pub struct ShellInstance {
    alias: Alias,
    args: Args,
    user: Users,
}

impl ShellInstance {
    pub fn update_alias(new_alias: Map<&str, &str>) -> () {}
}
