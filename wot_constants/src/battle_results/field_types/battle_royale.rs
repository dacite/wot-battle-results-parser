use crate::battle_results::field_types::{ResultField, ResultFieldType};


pub const BATTLE_ROYALE: [ResultField; 8] = [
    ResultField("maxAchievedBRTitle", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("brPosInBattle", "<type 'int'>", "255", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("battleXPTotal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("modulesDescriptors", "<type 'list'>", "[]", "<type 'NoneType'>", "extend", ResultFieldType::Server),
    ResultField("achivedLevel", "<type 'int'>", "1", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("basePointsDiff", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("sumPoints", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("hasBattlePass", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
];

