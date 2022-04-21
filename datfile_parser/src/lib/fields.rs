use std::collections::HashMap;

use wot_constants::{
    battle_results::{Field, FieldType, ALL_TYPES},
    ArenaBonusType,
};

pub const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

pub const MAX_VERSION: usize = 4;

#[derive(Debug)]
pub struct ChecksumInfo {
    pub fields_list_index: usize,
    pub version:           usize,
    pub arena_type:        ArenaBonusType,
}

// Manages the different types of field list
pub struct FieldCollection {
    fields_list: Vec<Vec<Field>>,
    checksums:   HashMap<i32, ChecksumInfo>,
}

pub fn gen_collection() -> FieldCollection {
    use ArenaBonusType::*;
    let arena_types = [EpicRandom, Ranked, EpicBattle, BattleRoyaleSolo, MapsTraining, Unknown];
    let mut fields_collection = FieldCollection::new();

    arena_types.into_iter().for_each(|arena_type| {
        let arena_fields = generate_fields_list(arena_type);

        arena_fields.into_iter().for_each(|field_list| {
            fields_collection.add_fields_list(field_list, arena_type);
        });
    });
    fields_collection
}

impl FieldCollection {
    fn new() -> Self {
        FieldCollection {
            fields_list: Vec::new(),
            checksums:   HashMap::new(),
        }
    }

    pub fn get_fields_list(&self, checksum: i32) -> Option<(&Vec<Field>, usize)> {
        let checksum_info = self.checksums.get(&checksum)?;
        let fields_list = &self.fields_list[checksum_info.fields_list_index];

        Some((fields_list, checksum_info.version))
    }

    pub fn add_fields_list(&mut self, fields: Vec<Field>, arena_type: ArenaBonusType) {
        let fields_list_index = self.fields_list.len();
        for version in 0..MAX_VERSION {
            let checksum = get_list_checksum(&fields, version);
            let checksum_info = ChecksumInfo {
                fields_list_index,
                version,
                arena_type,
            };

            self.checksums.insert(checksum, checksum_info);
        }
        self.fields_list.push(fields);
    }
}

pub fn generate_fields_list(arena_type: ArenaBonusType) -> Vec<Vec<Field>> {
    use FieldType::*;
    let field_types = [Common, PlayerInfo, AccountAll, AccountSelf, VehicleAll, VehicleSelf];

    let mut fields_list = Vec::new();
    field_types.into_iter().for_each(|field_type| {
        let mut fields = filter_list_for_type(field_type, ALL_TYPES);

        if let Some(arena_specific_fields) = arena_type.get_collection() {
            let mut arena_field_list = filter_list_for_type(field_type, arena_specific_fields);

            fields.append(&mut arena_field_list);
        }

        fields_list.push(fields);
    });

    fields_list
}

fn get_list_checksum(field_list: &[Field], version: usize) -> i32 {
    let mut list_string = String::new();

    field_list.iter().for_each(|field| {
        if matches_version(version, field) {
            list_string.push_str(field.combined_string);
        }
    });

    CRC32.checksum(list_string.as_bytes()) as i32
}

fn filter_list_for_type(field_type: FieldType, field_list: &[Field]) -> Vec<Field> {
    field_list
        .iter()
        .filter(|field| matches_type(field_type, field))
        .cloned()
        .collect()
}

/// Check if a field is part of a specific vesion.
/// If field has a max version of 0, it does not have a max version
pub fn matches_version(version: usize, field: &Field) -> bool {
    (field.version <= version) && (field.max_version > version || field.max_version == 0)
}

/// Check if a given field's type matches the type to match
/// We have a function for this because of the special cases `AccountAll` and
/// `VehicleAll`
fn matches_type(type_to_match: FieldType, field: &Field) -> bool {
    (field.field_type == type_to_match)
        || (field.field_type == FieldType::AccountAll && type_to_match == FieldType::AccountSelf)
        || (field.field_type == FieldType::VehicleAll && type_to_match == FieldType::VehicleSelf)
}
