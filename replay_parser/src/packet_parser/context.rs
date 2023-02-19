use std::collections::HashMap;

use crate::{entity_defs::EntityType, utils::validate_version};
// This file contains information regarding method calls for different replay versions. This is
// generated during the build process
include!(concat!(env!("OUT_DIR"), "/method_map_codegen.rs"));

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

    pub fn find_entity_type(&self, entity_id: i32) -> Option<EntityType> {
        self.entities.get(&entity_id).copied()
    }

    pub fn add_entity(&mut self, entity_id: i32, entity_type: EntityType) {
        self.entities.insert(entity_id, entity_type);
    }

    pub fn find_method(&self, entity_id: i32, method_id: i32) -> Option<&str> {
        let entity_name = if let Some(name) = self.entities.get(&entity_id) {
            name.to_string()
        } else {
            EntityType::Vehicle.to_string()
        };

        let version_str = crate::utils::version_as_string(self.version);

        find_method(&entity_name, &version_str, method_id)
    }

    pub fn find_player(&self, id: i32) -> Option<String> {
        self.players.get(&id).map(Into::into)
    }
}


pub fn find_method(entity_name: &str, version_str: &str, method_id: i32) -> Option<&'static str> {
    let key = format!("{entity_name} {version_str} {method_id}");
    METHOD_MAP.get(&key).copied()
}
