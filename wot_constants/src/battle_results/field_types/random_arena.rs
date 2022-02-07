use crate::battle_results::field_types::{FieldArenaType, ResultField, ResultFieldType};


pub const RANDOM_ARENA: [ResultField; 3] = [
    ResultField("basePointsDiff", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("sumPoints", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("hasBattlePass", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
];