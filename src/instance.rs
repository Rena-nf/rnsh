use crate::{alias::Alias, args::Args, users::Users};
use serde::Deserialize;

// Good luck to future me
#[derive(Deserialize)]
pub struct ShellInstance {
    pub alias: Option<Alias>,
    pub args: Option<Args>,
    pub user: Option<Users>,
}

#[doc(hidden)]
impl ShellInstance {}
