use super::{Field, FieldDefault, FieldType};

pub const FRONTLINE: [[Field; 13]; 1] = [
    [
        Field { name: "creditsAfterShellCosts", default: FieldDefault::Int(0), combined_string: "creditsAfterShellCosts<type 'int'>0<type 'NoneType'>skip", original_idx: 0, field_type: FieldType::AccountAll },
        Field { name: "unchargedShellCosts", default: FieldDefault::Int(0), combined_string: "unchargedShellCosts<type 'int'>0<type 'NoneType'>skip", original_idx: 1, field_type: FieldType::AccountAll },
        Field { name: "prevMetaLevel", default: FieldDefault::Tuple(&(FieldDefault::Int(1), 2)), combined_string: "prevMetaLevel<type 'tuple'>(1, 0)<type 'NoneType'>skip", original_idx: 2, field_type: FieldType::AccountAll },
        Field { name: "metaLevel", default: FieldDefault::Tuple(&(FieldDefault::Int(1), 2)), combined_string: "metaLevel<type 'tuple'>(1, 0)<type 'NoneType'>skip", original_idx: 3, field_type: FieldType::AccountAll },
        Field { name: "flXP", default: FieldDefault::Int(0), combined_string: "flXP<type 'int'>0<type 'NoneType'>skip", original_idx: 4, field_type: FieldType::AccountAll },
        Field { name: "originalFlXP", default: FieldDefault::Int(0), combined_string: "originalFlXP<type 'int'>0<type 'NoneType'>skip", original_idx: 5, field_type: FieldType::AccountAll },
        Field { name: "subtotalFlXP", default: FieldDefault::Int(0), combined_string: "subtotalFlXP<type 'int'>0<type 'NoneType'>skip", original_idx: 6, field_type: FieldType::AccountAll },
        Field { name: "boosterFlXP", default: FieldDefault::Int(0), combined_string: "boosterFlXP<type 'int'>0<type 'NoneType'>skip", original_idx: 7, field_type: FieldType::AccountAll },
        Field { name: "boosterFlXPFactor100", default: FieldDefault::Int(0), combined_string: "boosterFlXPFactor100<type 'int'>0<type 'NoneType'>any", original_idx: 8, field_type: FieldType::AccountAll },
    
        // Packed Value: DictPackers.ValueReplayPacker
        Field { name: "flXPReplay", default: FieldDefault::Str, combined_string: "flXPReplay<type 'str'><type 'instance'>skip", original_idx: 9, field_type: FieldType::AccountAll },
    
        Field { name: "basePointsDiff", default: FieldDefault::Int(0), combined_string: "basePointsDiff<type 'int'>0<type 'NoneType'>skip", original_idx: 10, field_type: FieldType::AccountAll },
        Field { name: "sumPoints", default: FieldDefault::Int(0), combined_string: "sumPoints<type 'int'>0<type 'NoneType'>skip", original_idx: 11, field_type: FieldType::AccountAll },
        Field { name: "hasBattlePass", default: FieldDefault::Bool(false), combined_string: "hasBattlePass<type 'bool'>False<type 'NoneType'>skip", original_idx: 12, field_type: FieldType::AccountAll },
    ]
];

