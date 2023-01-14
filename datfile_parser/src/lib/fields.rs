use std::collections::HashMap;

use wot_types::ArenaBonusType;

use crate::battle_results::{get_collection, Field, FieldType, ALL_TYPES, MAX_VERSION};

pub const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

#[derive(Debug)]
pub struct ChecksumInfo {
    pub fields_list_index: usize,
    pub version:           usize,
    pub arena_type:        ArenaBonusType,
}

// Manages the different types of field list
pub struct FieldCollection {
    fields_list: Vec<Vec<&'static Field>>,
    checksums:   HashMap<i64, ChecksumInfo>,
}

pub fn gen_collection() -> FieldCollection {
    use wot_types::ArenaBonusType::*;
    let arena_types = [
        EpicRandom,
        Ranked,
        EpicBattle,
        BattleRoyaleSolo,
        MapsTraining,
        Rts1x1,
        Unknown,
    ];

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

    pub fn get_fields_list(&self, checksum: i64) -> Option<(&Vec<&'static Field>, usize)> {
        let checksum_info = self.checksums.get(&checksum)?;
        let fields_list = &self.fields_list[checksum_info.fields_list_index];

        Some((fields_list, checksum_info.version))
    }

    pub fn add_fields_list(&mut self, fields: Vec<&'static Field>, arena_type: ArenaBonusType) {
        let fields_list_index = self.fields_list.len();
        for version in 0..MAX_VERSION {
            let checksum = get_list_checksum(&fields, version);
            let checksum_info = ChecksumInfo {
                fields_list_index,
                version,
                arena_type,
            };

            self.checksums.insert(checksum.into(), checksum_info);
        }
        self.fields_list.push(fields);
    }
}

pub fn generate_fields_list(arena_type: ArenaBonusType) -> Vec<Vec<&'static Field>> {
    use FieldType::*;
    let field_types = [
        Common,
        PlayerInfo,
        AccountAll,
        AccountSelf,
        VehicleAll,
        VehicleSelf,
    ];

    let mut fields_list = Vec::new();
    field_types.into_iter().for_each(|field_type| {
        let mut fields = filter_list_for_type(field_type, ALL_TYPES);

        if let Some(arena_specific_fields) = get_collection(arena_type) {
            let mut arena_field_list = filter_list_for_type(field_type, arena_specific_fields);

            fields.append(&mut arena_field_list);
        }

        fields_list.push(fields);
    });

    fields_list
}


fn get_list_checksum(field_list: &[&'static Field], version: usize) -> i32 {
    let list_string = field_list
        .iter()
        .filter_map(|field| {
            if matches_version(version, field) {
                Some(field.combined_string)
            } else {
                None
            }
        })
        .collect::<String>();

    // Here some data is lost due to conversion to i32 but it is intended
    CRC32.checksum(list_string.as_bytes()) as i32
}


fn filter_list_for_type(field_type: FieldType, field_list: &'static [Field]) -> Vec<&'static Field> {
    field_list
        .iter()
        .filter(|field| matches_type(field_type, field))
        .collect()
}

/// Check if a field is part of a specific vesion.
/// If field has a max version of 0, it does not have a max version.
/// If field version is higher than `MAX_VERSION` it's a programmer error.
pub fn matches_version(version: usize, field: &Field) -> bool {
    if field.version >= MAX_VERSION {
        panic!("field version is higher than MAX_VERSION. MAX_VERSION may not have been updated")
    }

    (field.version <= version) && (field.max_version > version || field.max_version == 0)
}

/// Check if a given field's type matches the type to match.
/// We have a function for this because of the special cases `AccountAll` and
/// `VehicleAll`. This is because `AccountAll` (for ex.) fields are also `AccountSelf` fields.
fn matches_type(type_to_match: FieldType, field: &Field) -> bool {
    (field.field_type == type_to_match)
        || (field.field_type == FieldType::AccountAll && type_to_match == FieldType::AccountSelf)
        || (field.field_type == FieldType::VehicleAll && type_to_match == FieldType::VehicleSelf)
}
