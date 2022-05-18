pub mod entity;
mod types;
use std::collections::HashMap;
use wot_replay_parser::Result;
pub use types::load_type_aliases;
pub use types::TypeAlias;

pub struct DefinitionParser {
    game_version: [u16; 4],
    type_aliases: HashMap<String, TypeAlias>
}

impl DefinitionParser {
    pub fn new(game_version: [u16; 4]) -> Result<Self> {
        let type_aliases = load_type_aliases(game_version)?;

        Ok(DefinitionParser { game_version, type_aliases })
    }
}


