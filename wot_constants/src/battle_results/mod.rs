mod battle_result_fields;

use std::collections::{BTreeMap};

use unpickler::PickleValue;

use crate::ArenaBonusType;

pub use battle_result_fields::{RANDOM_ARENA, RANKED, FRONTLINE, BATTLE_ROYALE, MAPS_TRAINING, RTS, ALL_TYPES, };

#[derive(Clone)]
/// A data structure that holds information about a field found in battle results.
pub struct Field {
    /// Name of the battle result field. Ex: damageDealt
    pub name: &'static str,
    
    /// Default value of the battle result
    pub default: FieldDefault,

    /// A value needed to generate the checksum. This value comes from WoT's python code
    pub combined_string: &'static str,

    /// A relative number that tells us when this field was introduced
    pub version: usize,

    /// A relative number that tells us when the field was removed
    pub max_version: usize,

    /// The context where this field occurs. Ex: damageDealt is found for the player themselves(VehicleSelf)
    /// and all the other players in battle(VehicleAll)
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
            ArenaBonusType::Rts => Some(RTS),
            ArenaBonusType::Rts1x1 => Some(RTS),
            ArenaBonusType::RtsBootcamp => Some(RTS),
            _ => None
        }
    }
}

