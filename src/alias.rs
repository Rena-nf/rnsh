use serde::Deserialize;
use toml::Table;

#[derive(Deserialize)]
pub struct Alias {
    pub aliases: Table,
}

impl Alias {
    pub fn update_alias(&mut self, new_alias: Alias) {}
}
