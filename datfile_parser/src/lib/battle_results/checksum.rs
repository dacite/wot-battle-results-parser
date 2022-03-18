use std::collections::HashMap;

use wot_constants::battle_results::FieldType;



/// A checksum corresponds to particular list of identifiers with a specific `FieldType` and `ArenaBonusType` and `version`.
///
/// For ex: `FieldType::PlayerInfo` and `ArenaBonusType::Random` and `version = 1 `
///
/// In the above example it tells us that the `checksum` belongs to a list of values that describes a particular player's
/// player info in random battles and the exact version (which tells us which identifiers may/may not be present) is `1`
#[derive(Clone, Debug)]
pub struct ChecksumInfo {
    pub field_type: FieldType,
    pub arena_type: wot_constants::ArenaBonusType,
    pub checksum: i32,
    pub version: usize,
}

pub struct ChecksumManager {
    pub checksums: HashMap<i32, ChecksumInfo>,
}
impl Default for ChecksumManager {
    fn default() -> Self {
        Self::new()
    }
}
impl ChecksumManager {
    pub fn new() -> Self {
        ChecksumManager {
            checksums: HashMap::new(),
        }
    }
    pub fn insert(&mut self, checksum_info: ChecksumInfo) {
        self.checksums.insert(checksum_info.checksum, checksum_info);
    }

    pub fn insert_list(&mut self, checksum_info: Vec<ChecksumInfo>) {
        for x in checksum_info {
            self.insert(x)
        }
    }
    pub fn get(&self, checksum: i32, field_type: FieldType) -> Option<ChecksumInfo> {
        return if let Some(checksum_info) = self.checksums.get(&checksum) {
            if field_type == checksum_info.field_type {
                Some(checksum_info.clone())
            } else {
                None
            }
        } else {
            None
        };
    }
}
