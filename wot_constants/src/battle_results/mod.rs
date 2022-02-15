mod field;
mod battle_result_fields;

use std::collections::HashMap;

use battle_result_fields::{ALL_TYPES, BATTLE_ROYALE, FRONTLINE, RANDOM_ARENA, RANKED, MAPS_TRAINING};
pub use field::{ChecksumManager, Collection, FieldType, Field};

use crate::ArenaBonusType;


pub struct BattleResultsManager {
    fields_collection: HashMap<ArenaBonusType, Collection>,
    checksum_manager: ChecksumManager
}

impl BattleResultsManager {
    pub fn new() -> Self {
        let mut fields_collection = HashMap::new();
        let mut checksums = HashMap::new();

        fields_collection.insert(ArenaBonusType::EpicRandom, Collection::new(ArenaBonusType::EpicRandom));
        fields_collection.insert(ArenaBonusType::Ranked, Collection::new(ArenaBonusType::Ranked));
        fields_collection.insert(ArenaBonusType::BattleRoyaleSolo, Collection::new(ArenaBonusType::BattleRoyaleSolo));
        fields_collection.insert(ArenaBonusType::MapsTraining, Collection::new(ArenaBonusType::MapsTraining));
        fields_collection.insert(ArenaBonusType::EpicBattle, Collection::new(ArenaBonusType::EpicBattle));
        fields_collection.iter_mut().for_each(|item| {
            checksums.insert(FieldType::AccountAll, item.1.account_all.generate_all_checksums());
            checksums.insert(FieldType::AccountSelf, item.1.account_self.generate_all_checksums());
            checksums.insert(FieldType::VehicleAll, item.1.vehicle_all.generate_all_checksums());
            checksums.insert(FieldType::VehicleSelf, item.1.vehicle_self.generate_all_checksums());
            checksums.insert(FieldType::Server, item.1.server.generate_all_checksums());
            checksums.insert(FieldType::Common, item.1.common.generate_all_checksums());
        });

        Self {
            fields_collection,
            checksum_manager: ChecksumManager {checksums},
        }
    }

    pub fn get_iden_list(&self, field_type: FieldType, checksum: i32) -> Option<Vec<Field>> {
        if let Some(checksum_info) = self.checksum_manager.get(field_type, checksum) {
            let big_collection = self.fields_collection.get(&checksum_info.arena_type).unwrap();
            let mut collection = big_collection.get_collection_from_type(field_type);

            collection.restore(checksum_info.version);
            
            return Some(collection.get_fields());
        }
        return None;
    }
}
