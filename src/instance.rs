use crate::{alias::Alias, args::Args, users::Users};

// Good luck to future me
pub struct ShellInstance {
    pub alias: Alias,
    pub args: Args,
    pub user: Users,
}

#[doc(hidden)]
impl ShellInstance {}
