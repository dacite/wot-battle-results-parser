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

pub struct Field {
    name: &'static str,
    default: FieldDefault,
    combined_string: &'static str,
    original_idx: u32,
    field_type: FieldType,
}

/// Type of a Field. For ex: if FieldType is VehicleAll, its a field that is present
/// for every player in that particular battle. In this case name of that Field could 
/// be damageDealt.
pub enum FieldType {
    Common,
    PlayerInfo,
    AccountAll,
    AccountSelf,
    VehicleAll,
    VehicleSelf,
    Server,
}

/// A Representation for the default value for a certain field.
/// `Dict`, `Str`, `List` variants create empty instances of HashMap, String, Vec respectively.
/// `Tuple` variant can be seen as the tuple.0 value repeated tuple.1 times. For ex: Tuple(&(Int(0), 3))
/// is a tuple like (0, 0, 0) 
pub enum FieldDefault {
    None,
    Int(i64),
    Bool(bool),
    Float(f64),
    Dict,
    Str,
    List,
    Set,
    Tuple(&'static(FieldDefault, u32)),
}

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

