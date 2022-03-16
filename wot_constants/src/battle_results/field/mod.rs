mod collection;
mod field_list;
mod checksum;

use std::collections::{BTreeMap};

use unpickler::PickleValue;

use crate::ArenaBonusType;
pub use checksum::{ChecksumInfo, ChecksumManager};
pub use collection::Collection;

use super::{RANDOM_ARENA, RANKED, FRONTLINE, BATTLE_ROYALE, MAPS_TRAINING};

#[derive(Clone)]
pub struct Field {
    pub name: &'static str,
    pub default: FieldDefault,
    pub combined_string: &'static str,
    pub version: usize,
    pub max_version: usize,
    pub field_type: FieldType,
}


/// Type of a Field. For ex: if FieldType is VehicleAll, its a field that is present
/// for every player in that particular battle. In this case name of that Field could 
/// be damageDealt.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
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
#[derive(Clone)]
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

impl FieldDefault {
    pub fn to_pickle_value(&self) -> PickleValue {
        match self {
            FieldDefault::None => PickleValue::None,
            FieldDefault::Int(x) => PickleValue::I64(*x),
            FieldDefault::Bool(x) => PickleValue::Bool(*x),
            FieldDefault::Float(x) => PickleValue::F64(*x),
            FieldDefault::Dict => PickleValue::Dict(BTreeMap::new()),
            FieldDefault::Str => PickleValue::String(String::from("")),
            FieldDefault::List => PickleValue::List(Vec::new()),
            FieldDefault::Set => PickleValue::List(Vec::new()),
            FieldDefault::Tuple(x) => {
                let mut pickle_value = Vec::new();

                for _ in 0..(x.1) {
                    pickle_value.push(x.0.clone().to_pickle_value());
                }
                PickleValue::Tuple(pickle_value)
            },
        }
    }
}

impl ArenaBonusType {
    pub fn get_collection(&self) -> Option<&[Field]> {
        match self {
            ArenaBonusType::EpicRandom => Some(RANDOM_ARENA),
            ArenaBonusType::EpicRandomTraining => Some(RANDOM_ARENA),
            ArenaBonusType::Mapbox => Some(RANDOM_ARENA),
            ArenaBonusType::Ranked => Some(RANKED),
            ArenaBonusType::EpicBattle => Some(FRONTLINE),
            ArenaBonusType::EpicBattleTraining => Some(FRONTLINE),
            ArenaBonusType::BattleRoyaleTrnSolo => Some(BATTLE_ROYALE),
            ArenaBonusType::BattleRoyaleTrnSquad => Some(BATTLE_ROYALE),
            ArenaBonusType::BattleRoyaleSolo => Some(BATTLE_ROYALE),
            ArenaBonusType::BattleRoyaleSquad => Some(BATTLE_ROYALE),
            ArenaBonusType::MapsTraining => Some(MAPS_TRAINING),
            _ => None
        }
    }
}

