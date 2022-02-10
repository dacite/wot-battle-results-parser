use super::{Field, FieldDefault, FieldType};

pub const BATTLE_ROYALE: [[Field; 8]; 1] = [
    [
        Field { name: "maxAchievedBRTitle", default: FieldDefault::Tuple(&(FieldDefault::Int(0), 2)), combined_string: "maxAchievedBRTitle<type 'tuple'>(0, 0)<type 'NoneType'>skip", original_idx: 0, field_type: FieldType::AccountSelf },
        Field { name: "brPosInBattle", default: FieldDefault::Int(255), combined_string: "brPosInBattle<type 'int'>255<type 'NoneType'>skip", original_idx: 1, field_type: FieldType::VehicleSelf },
        Field { name: "battleXPTotal", default: FieldDefault::Int(0), combined_string: "battleXPTotal<type 'int'>0<type 'NoneType'>sum", original_idx: 2, field_type: FieldType::Server },
        Field { name: "modulesDescriptors", default: FieldDefault::List, combined_string: "modulesDescriptors<type 'list'>[]<type 'NoneType'>extend", original_idx: 3, field_type: FieldType::Server },
        Field { name: "achivedLevel", default: FieldDefault::Int(1), combined_string: "achivedLevel<type 'int'>1<type 'NoneType'>skip", original_idx: 4, field_type: FieldType::VehicleAll },
        Field { name: "basePointsDiff", default: FieldDefault::Int(0), combined_string: "basePointsDiff<type 'int'>0<type 'NoneType'>skip", original_idx: 5, field_type: FieldType::AccountAll },
        Field { name: "sumPoints", default: FieldDefault::Int(0), combined_string: "sumPoints<type 'int'>0<type 'NoneType'>skip", original_idx: 6, field_type: FieldType::AccountAll },
        Field { name: "hasBattlePass", default: FieldDefault::Bool(false), combined_string: "hasBattlePass<type 'bool'>False<type 'NoneType'>skip", original_idx: 7, field_type: FieldType::AccountAll },    
    ]
];

