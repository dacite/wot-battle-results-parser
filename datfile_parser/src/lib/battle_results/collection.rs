use wot_constants::ArenaBonusType;
use wot_constants::battle_results::{FieldType};

use super::field_list::FieldList;

pub struct Collection {
    pub account_all: FieldList,
    pub account_self: FieldList,
    pub vehicle_all: FieldList,
    pub vehicle_self: FieldList,
    pub common: FieldList,
    pub server: FieldList,
    pub player_info: FieldList,
}

impl Collection {
    pub fn new(arena_type: ArenaBonusType) -> Self {
        Self {
            account_all: FieldList::new(arena_type, FieldType::AccountAll),
            account_self: FieldList::new(arena_type, FieldType::AccountSelf),
            vehicle_all: FieldList::new(arena_type, FieldType::VehicleAll),
            vehicle_self: FieldList::new(arena_type, FieldType::VehicleSelf),
            common: FieldList::new(arena_type, FieldType::Common),
            server: FieldList::new(arena_type, FieldType::Server),
            player_info: FieldList::new(arena_type, FieldType::PlayerInfo),
        }
    }

    pub fn get_collection_from_type(&self, field_type: FieldType) -> FieldList {
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
