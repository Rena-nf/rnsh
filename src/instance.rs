use crate::{alias::Alias, users::Users};
use serde::Deserialize;

// Good luck to future me
#[derive(Deserialize)]
pub struct ShellInstance {
    pub alias: Option<Alias>,
    pub user: Option<Users>,
}

#[doc(hidden)]
impl ShellInstance {}
