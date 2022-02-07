use crate::battle_results::field_types::{FieldArenaType, ResultField, ResultFieldType};


pub const RANKED: [ResultField; 16] = [
    ResultField("updatedRankChange", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("accRank", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("vehRank", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("prevMaxRank", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("prevVehRank", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("shields", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("prevShields", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("rankedSeason", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("rankedSeasonNum", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("bonusBattleUsed", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("efficiencyBonusBattles", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("stepsBonusBattles", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("prevAccRank", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("basePointsDiff", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("sumPoints", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("hasBattlePass", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),

];

