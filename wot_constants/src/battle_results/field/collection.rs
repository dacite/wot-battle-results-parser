use crate::ArenaBonusType;

use super::{field_list::FieldsList, FieldType};

pub struct Collection {
    pub account_all: FieldsList,
    pub account_self: FieldsList,
    pub vehicle_all: FieldsList,
    pub vehicle_self: FieldsList,
    pub common: FieldsList,
    pub server: FieldsList,
    pub player_info: FieldsList,
}

impl Collection {
    pub fn new(arena_type: ArenaBonusType) -> Self {
        Self {
            account_all: FieldsList::new(arena_type, FieldType::AccountAll),
            account_self: FieldsList::new(arena_type, FieldType::AccountSelf),
            vehicle_all: FieldsList::new(arena_type, FieldType::VehicleAll),
            vehicle_self: FieldsList::new(arena_type, FieldType::VehicleSelf),
            common: FieldsList::new(arena_type, FieldType::Common),
            server: FieldsList::new(arena_type, FieldType::Server),
            player_info: FieldsList::new(arena_type, FieldType::PlayerInfo),
        }
    }

    pub fn get_collection_from_type(&self, field_type: FieldType) -> FieldsList {
        match field_type {
            FieldType::Common => self.common.clone(),
            FieldType::PlayerInfo => self.player_info.clone(),
            FieldType::AccountAll => self.account_all.clone(),
            FieldType::AccountSelf => self.account_self.clone(),
            FieldType::VehicleAll => self.vehicle_all.clone(),
            FieldType::VehicleSelf => self.vehicle_self.clone(),
            FieldType::Server => self.server.clone(),
        }
    }
}
