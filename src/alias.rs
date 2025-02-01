use serde::Deserialize;
use toml::Table;

#[derive(Deserialize)]
pub struct Alias {
    pub aliases: Table,
}

#[doc(hidden)]
impl Alias {}
