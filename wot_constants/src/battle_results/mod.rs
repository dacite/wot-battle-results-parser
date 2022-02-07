mod field_types;

use std::collections::HashMap;

pub use field_types::FieldArenaType;
pub use field_types::ResultFieldType;
pub use field_types::ResultField;
pub use field_types::FieldCollection;


struct BattleResultsHandler {
    current: FieldArenaType,
    checksums: HashMap<FieldArenaType, (ResultFieldType, i64)>
}

impl BattleResultsHandler {
    // fn new() -> Self {
    //
    // }
}