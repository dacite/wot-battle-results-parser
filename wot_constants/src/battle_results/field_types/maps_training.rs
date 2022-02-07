use crate::battle_results::field_types::{FieldArenaType, ResultField, ResultFieldType};


pub const MAPS_TRAINING: [ResultField; 3] = [
    ResultField("scenarioProgress", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("mt_progressImproved", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("mt_mapComplete", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
];

