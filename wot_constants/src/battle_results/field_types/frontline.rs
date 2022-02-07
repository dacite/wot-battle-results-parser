use crate::battle_results::field_types::{FieldArenaType, ResultField, ResultFieldType};


pub const FRONTLINE: [ResultField; 13] = [
    ResultField("creditsAfterShellCosts", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("unchargedShellCosts", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("prevMetaLevel", "<type 'tuple'>", "(1, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("metaLevel", "<type 'tuple'>", "(1, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("flXP", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("originalFlXP", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("subtotalFlXP", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("boosterFlXP", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("boosterFlXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::AccountAll),
    ResultField("flXPReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::AccountAll),
    ResultField("basePointsDiff", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("sumPoints", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("hasBattlePass", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
];

