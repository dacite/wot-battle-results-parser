use std::collections::HashMap;

use crate::{ArenaBonusType};

use super::FieldType;

#[derive(Clone, Debug)]

/// A checksum corresponds to particular list of identifiers with a specific `FieldType` and `ArenaBonusType` and `version`.
/// 
/// For ex: `FieldType::PlayerInfo` and `ArenaBonusType::Random` and `version = (1, 0)`
/// 
/// In the above example it tells us that the `checksum` belongs to a list of values that describes a particular player's
/// player info in random battles and the exact version (which tells us which values may/may not be present) is `(1, 0)`
pub struct ChecksumInfo {
    pub field_type: FieldType,
    pub arena_type: ArenaBonusType,
    pub checksum: i32,
    pub version: (usize, usize),
}

pub struct ChecksumManager {
    pub checksums: HashMap<FieldType, Vec<ChecksumInfo>>
}

impl ChecksumManager {
    pub fn insert(&mut self, field_type: FieldType, mut additional: Vec<ChecksumInfo>) {
        if let Some(current_list) = self.checksums.get_mut(&field_type) {
            current_list.append(&mut additional);
        } else {
            self.checksums.insert(field_type, additional);
        }
    }
    
    pub fn get(&self, field_type: FieldType, checksum: i32) -> Option<ChecksumInfo>{
        return if let Some(checksum_list) = self.checksums.get(&field_type) {
            for checksum_info in checksum_list {
                if checksum_info.checksum == checksum {
                    Some(checksum_info.clone());
                }
            }
            None
        } else {
            None
        }
    }
}

