mod field_arena_types;
mod all_types;
mod ranked;
mod frontline;
mod random_arena;
mod maps_training;
mod battle_royale;

use std::collections::HashMap;
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::ArenaBonusType;
use crate::battle_results::field_types::all_types::ALL_TYPES;
use crate::battle_results::field_types::battle_royale::BATTLE_ROYALE;
use crate::battle_results::field_types::frontline::FRONTLINE;
use crate::battle_results::field_types::maps_training::MAPS_TRAINING;
use crate::battle_results::field_types::random_arena::RANDOM_ARENA;
use crate::battle_results::field_types::ranked::RANKED;

pub const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Clone)]
pub enum FieldArenaType {
    All,
    BattleRoyale,
    Frontline,
    MapsTraining,
    RandomArena,
    Ranked,
}


pub struct FieldCollection {
    account_all: Vec<ResultField>,
    account_self: Vec<ResultField>,
    vehicle_all: Vec<ResultField>,
    vehicle_self: Vec<ResultField>,
    common: Vec<ResultField>,
    player_info: Vec<ResultField>,
    server: Vec<ResultField>,

    checksums: HashMap<i32, ResultFieldType>
}



impl Default for FieldCollection {
    fn default() -> Self {
        Self {
            account_all: Vec::new(),
            account_self: Vec::new(),
            vehicle_all: Vec::new(),
            vehicle_self: Vec::new(),
            common: Vec::new(),
            player_info: Vec::new(),
            server: Vec::new(),
            checksums: HashMap::new(),
        }
    }
}

impl ArenaBonusType {
    pub fn get_collection(&self) -> Option<&[ResultField]> {
        match self {
            ArenaBonusType::EpicRandom => Some(RANDOM_ARENA.as_slice()),
            ArenaBonusType::EpicRandomTraining => Some(RANDOM_ARENA.as_slice()),
            ArenaBonusType::Mapbox => Some(RANDOM_ARENA.as_slice()),
            ArenaBonusType::Ranked => Some(RANKED.as_slice()),
            ArenaBonusType::EpicBattle => Some(FRONTLINE.as_slice()),
            ArenaBonusType::EpicBattleTraining => Some(FRONTLINE.as_slice()),
            ArenaBonusType::BattleRoyaleTrnSolo => Some(BATTLE_ROYALE.as_slice()),
            ArenaBonusType::BattleRoyaleTrnSquad => Some(BATTLE_ROYALE.as_slice()),
            ArenaBonusType::BattleRoyaleSolo => Some(BATTLE_ROYALE.as_slice()),
            ArenaBonusType::BattleRoyaleSquad => Some(BATTLE_ROYALE.as_slice()),
            ArenaBonusType::MapsTraining => Some(MAPS_TRAINING.as_slice()),
            _ => None
        }
    }
}

impl FieldCollection {
    pub fn new(arena_type: ArenaBonusType) -> Self {
        let mut new_field_collection = FieldCollection::default();
        new_field_collection.append(ALL_TYPES.as_slice());

        if let Some(additional_collection) = arena_type.get_collection() {
            new_field_collection.append(additional_collection)
        }

        new_field_collection.generate_checksums();

        new_field_collection
    }

    pub fn append(&mut self, field_array: &[ResultField]) {
        for field in field_array {
            match field.5 {
                ResultFieldType::PlayerInfo => self.player_info.push(field.clone()),
                ResultFieldType::Common => self.common.push(field.clone()),
                ResultFieldType::AccountAll => {
                    self.account_all.push(field.clone());
                    self.account_self.push(field.clone());
                },
                ResultFieldType::AccountSelf => self.account_self.push(field.clone()),
                ResultFieldType::VehicleAll => {
                    self.vehicle_all.push(field.clone());
                    self.vehicle_self.push(field.clone());
                },
                ResultFieldType::VehicleSelf => self.vehicle_self.push(field.clone()),
                ResultFieldType::Server => self.server.push(field.clone())
            }
        }
    }

    pub fn get_child_from_type(&self, child_type: &ResultFieldType) -> Vec<ResultField> {
        match child_type {
            ResultFieldType::Common => self.common.clone(),
            ResultFieldType::PlayerInfo => self.player_info.clone(),
            ResultFieldType::AccountAll => self.account_all.clone(),
            ResultFieldType::AccountSelf => self.account_self.clone(),
            ResultFieldType::VehicleAll => self.vehicle_all.clone(),
            ResultFieldType::VehicleSelf => self.vehicle_self.clone(),
            ResultFieldType::Server => self.server.clone(),
        }
    }

    pub fn get_child_from_checksum(&self, checksum: i32) -> Option<Vec<ResultField>> {
        return if let Some(result) = self.checksums.get(&checksum) {
            Some(self.get_child_from_type(result))
        } else {
            None
        }
    }


    fn generate_checksums(&mut self) {
        self.checksums.insert(calculate_checksum(self.common.clone()), ResultFieldType::Common);
        self.checksums.insert(calculate_checksum(self.player_info.clone()), ResultFieldType::PlayerInfo);
        self.checksums.insert(calculate_checksum(self.account_self.clone()), ResultFieldType::AccountSelf);
        self.checksums.insert(calculate_checksum(self.account_all.clone()), ResultFieldType::AccountAll);
        self.checksums.insert(calculate_checksum(self.vehicle_self.clone()), ResultFieldType::VehicleSelf);
        self.checksums.insert(calculate_checksum(self.vehicle_all.clone()), ResultFieldType::VehicleAll);
        self.checksums.insert(calculate_checksum(self.server.clone()), ResultFieldType::Server);
    }
}

fn calculate_checksum(data: Vec<ResultField>) -> i32 {
    let mut combined_string = String::from("");
    data.into_iter().for_each(|field| combined_string.push_str(&field.get_combined_string()));

    CRC32.checksum(combined_string.as_bytes()) as i32
}


#[derive(Clone)]
pub struct ResultField(&'static str, &'static str, &'static str, &'static str, &'static str, ResultFieldType);

impl ResultField {
    pub fn get_combined_string(&self) -> String {
        [self.0, self.1, self.2, self.3, self.4].concat()
    }

    pub fn get_name(&self) -> &str {
        self.0
    }
}

#[derive(Clone)]
pub enum ResultFieldType {
    Common,
    PlayerInfo,
    AccountAll,
    AccountSelf,
    VehicleAll,
    VehicleSelf,
    Server,
}

