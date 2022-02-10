use super::{Field, FieldDefault, FieldType};

pub const RANDOM_ARENA: [[Field; 3]; 1] = [
    [
        Field { name: "basePointsDiff", default: FieldDefault::Int(0), combined_string: "basePointsDiff<type 'int'>0<type 'NoneType'>skip", original_idx: 0, field_type: FieldType::AccountAll },
        Field { name: "sumPoints", default: FieldDefault::Int(0), combined_string: "sumPoints<type 'int'>0<type 'NoneType'>skip", original_idx: 1, field_type: FieldType::AccountAll },
        Field { name: "hasBattlePass", default: FieldDefault::Bool(false), combined_string: "hasBattlePass<type 'bool'>False<type 'NoneType'>skip", original_idx: 2, field_type: FieldType::AccountAll },
    ]
];