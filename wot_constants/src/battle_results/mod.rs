mod battle_result_fields;
mod field;

use std::collections::HashMap;

use battle_result_fields::{ALL_TYPES, BATTLE_ROYALE, FRONTLINE, MAPS_TRAINING, RANDOM_ARENA, RANKED};
pub use field::{ChecksumManager, Collection, Field, FieldType};

use crate::ArenaBonusType;

pub struct BattleResultsManager {
    fields_collection: HashMap<ArenaBonusType, Collection>,
    checksum_manager: ChecksumManager,
}
impl Default for BattleResultsManager {
    fn default() -> Self {
        Self::new()
    }
}
impl BattleResultsManager {
    pub fn new() -> Self {
        let mut fields_collection = HashMap::new();

        let mut checksum_manager = ChecksumManager::new();
        fields_collection.insert(ArenaBonusType::EpicRandom,Collection::new(ArenaBonusType::EpicRandom),);
        fields_collection.insert(ArenaBonusType::Ranked,Collection::new(ArenaBonusType::Ranked),);
        fields_collection.insert(ArenaBonusType::BattleRoyaleSolo,Collection::new(ArenaBonusType::BattleRoyaleSolo),);
        fields_collection.insert(ArenaBonusType::MapsTraining,Collection::new(ArenaBonusType::MapsTraining),);
        fields_collection.insert(ArenaBonusType::EpicBattle,Collection::new(ArenaBonusType::EpicBattle),);
        fields_collection.insert(ArenaBonusType::Unknown,Collection::new(ArenaBonusType::Unknown),);
        
        fields_collection.iter_mut().for_each(|item| {
            checksum_manager.insert_list(item.1.account_all.generate_all_checksums());
            checksum_manager.insert_list(item.1.account_self.generate_all_checksums());
            checksum_manager.insert_list(item.1.vehicle_all.generate_all_checksums());
            checksum_manager.insert_list(item.1.vehicle_self.generate_all_checksums());
            checksum_manager.insert_list(item.1.server.generate_all_checksums());
            checksum_manager.insert_list(item.1.player_info.generate_all_checksums());
            checksum_manager.insert_list(item.1.common.generate_all_checksums());
        });

        Self {
            fields_collection,
            checksum_manager,
        }
    }

    pub fn get_iden_list(&self, field_type: FieldType, checksum: i32) -> Option<Vec<Field>> {
        if let Some(checksum_info) = self.checksum_manager.get(checksum, field_type) {
            let big_collection = self
                .fields_collection
                .get(&checksum_info.arena_type)
                .unwrap();
            let collection = big_collection.get_collection_from_type(field_type);

            return Some(collection.get_fields(checksum_info.version));
        }
        None
    }
}
