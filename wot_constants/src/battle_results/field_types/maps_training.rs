use super::{Field, FieldDefault, FieldType};

pub const MAPS_TRAINING: [[Field; 3]; 1] = [
    [
        Field { name: "scenarioProgress", default: FieldDefault::Dict, combined_string: "scenarioProgress<type 'dict'>{}<type 'NoneType'>skip", original_idx: 0, field_type: FieldType::AccountSelf },
        Field { name: "mt_progressImproved", default: FieldDefault::Int(0), combined_string: "mt_progressImproved<type 'int'>0<type 'NoneType'>skip", original_idx: 1, field_type: FieldType::AccountSelf },
        Field { name: "mt_mapComplete", default: FieldDefault::Int(0), combined_string: "mt_mapComplete<type 'int'>0<type 'NoneType'>skip", original_idx: 2, field_type: FieldType::AccountSelf },    
    ]
];

