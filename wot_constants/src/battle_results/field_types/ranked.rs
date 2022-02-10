use super::{Field, FieldDefault, FieldType};

pub const RANKED: [[Field; 16]; 1] = [
    [
        Field { name: "updatedRankChange", default: FieldDefault::Int(0), combined_string: "updatedRankChange<type 'int'>0<type 'NoneType'>skip", original_idx: 0, field_type: FieldType::AccountSelf },
        Field { name: "accRank", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "accRank<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 1, field_type: FieldType::AccountSelf },
        Field { name: "vehRank", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "vehRank<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 2, field_type: FieldType::AccountSelf },
        Field { name: "prevMaxRank", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "prevMaxRank<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 3, field_type: FieldType::AccountSelf },
        Field { name: "prevVehRank", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "prevVehRank<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 4, field_type: FieldType::AccountSelf },
        Field { name: "shields", default: FieldDefault::Dict, combined_string: "shields<type 'dict'>{}<type 'NoneType'>skip", original_idx: 5, field_type: FieldType::AccountSelf },
        Field { name: "prevShields", default: FieldDefault::Dict, combined_string: "prevShields<type 'dict'>{}<type 'NoneType'>skip", original_idx: 6, field_type: FieldType::AccountSelf },
        Field { name: "rankedSeason", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "rankedSeason<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 7, field_type: FieldType::AccountSelf },
        Field { name: "rankedSeasonNum", default: FieldDefault::Int(0), combined_string: "rankedSeasonNum<type 'int'>0<type 'NoneType'>skip", original_idx: 8, field_type: FieldType::AccountSelf },
        Field { name: "bonusBattleUsed", default: FieldDefault::Int(0), combined_string: "bonusBattleUsed<type 'int'>0<type 'NoneType'>skip", original_idx: 9, field_type: FieldType::AccountSelf },
        Field { name: "efficiencyBonusBattles", default: FieldDefault::Int(0), combined_string: "efficiencyBonusBattles<type 'int'>0<type 'NoneType'>skip", original_idx: 10, field_type: FieldType::AccountSelf },
        Field { name: "stepsBonusBattles", default: FieldDefault::Int(0), combined_string: "stepsBonusBattles<type 'int'>0<type 'NoneType'>skip", original_idx: 11, field_type: FieldType::AccountSelf },
        Field { name: "prevAccRank", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "prevAccRank<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 12, field_type: FieldType::AccountAll },
        Field { name: "basePointsDiff", default: FieldDefault::Int(0), combined_string: "basePointsDiff<type 'int'>0<type 'NoneType'>skip", original_idx: 13, field_type: FieldType::AccountAll },
        Field { name: "sumPoints", default: FieldDefault::Int(0), combined_string: "sumPoints<type 'int'>0<type 'NoneType'>skip", original_idx: 14, field_type: FieldType::AccountAll },
        Field { name: "hasBattlePass", default: FieldDefault::Bool(false), combined_string: "hasBattlePass<type 'bool'>False<type 'NoneType'>skip", original_idx: 15, field_type: FieldType::AccountAll },    
    ]
];

