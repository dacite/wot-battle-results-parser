use crate::ArenaBonusType;

use super::{field_list::FieldsVec, FieldType};

pub struct Collection {
    pub account_all: FieldsVec,
    pub account_self: FieldsVec,
    pub vehicle_all: FieldsVec,
    pub vehicle_self: FieldsVec,
    pub common: FieldsVec,
    pub server: FieldsVec,
    pub player_info: FieldsVec
}

impl Collection {
    pub fn new(arena_type: ArenaBonusType) -> Self {
        Self {
            account_all: FieldsVec::new(arena_type, FieldType::AccountAll),
            account_self: FieldsVec::new(arena_type, FieldType::AccountSelf),
            vehicle_all: FieldsVec::new(arena_type, FieldType::VehicleAll),
            vehicle_self: FieldsVec::new(arena_type, FieldType::VehicleSelf),
            common: FieldsVec::new(arena_type, FieldType::Common),
            server: FieldsVec::new(arena_type, FieldType::Server),
            player_info: FieldsVec::new(arena_type, FieldType::PlayerInfo)
        }
    }

    pub fn get_collection_from_type(&self, field_type: FieldType) -> FieldsVec {
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