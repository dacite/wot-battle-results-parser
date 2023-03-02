use std::collections::HashMap;

use crate::{entity_defs::EntityType, utils::validate_version, PacketError};


#[derive(Default, Debug)]
pub struct Context {
    entities: HashMap<i32, EntityType>,
    players:  HashMap<i32, String>,
    version:  [u16; 4],
}

impl Context {
    pub fn new(version: [u16; 4], players: HashMap<i32, String>) -> Self {
        let validated_version = validate_version(version);
        Context {
            entities: HashMap::new(),
            players,
            version: validated_version,
        }
    }

    /// This may not be same as the replay version. This version returns a version that is closest to
    /// the actual replay version that we have .def files for
    pub fn get_version(&self) -> [u16; 4] {
        self.version
    }

    pub fn find_entity_type(&self, entity_id: i32) -> Result<EntityType, PacketError> {
        self.entities
            .get(&entity_id)
            .copied()
            .ok_or_else(|| PacketError::NotFoundError {
                err: format!("entity with id: {entity_id} not found for current replay context"),
            })
    }

    pub fn add_entity(&mut self, entity_id: i32, entity_type: EntityType) {
        self.entities.insert(entity_id, entity_type);
    }

    pub fn find_player(&self, id: i32) -> Option<String> {
        self.players.get(&id).map(Into::into)
    }
}
