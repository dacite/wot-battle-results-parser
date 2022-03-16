use crate::{ArenaBonusType, battle_results::ALL_TYPES};
use super::{ChecksumInfo, FieldType, Field};

pub const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

#[derive(Clone)]
pub struct FieldList {
    arena_type: ArenaBonusType,
    field_type: FieldType,
    general: Vec<Field>,

    max_version: usize,
}

impl FieldList {
    pub fn new(arena_type: ArenaBonusType, field_type: FieldType) -> FieldList {
        let mut fields = Vec::new();
        let mut max_version = 0;
        for field in ALL_TYPES {
            if FieldList::should_insert(field, field_type) {
                if field.version > max_version {
                    max_version = field.version;
                }
                fields.push(field.clone());
            }
        }

        if let Some(arena_specific_fields) = arena_type.get_collection() {
            for field in arena_specific_fields {
                if FieldList::should_insert(field, field_type) {
                    if field.version > max_version {
                        max_version = field.version;
                    }
                    fields.push(field.clone());
                }
            }
        }

        FieldList {
            arena_type,
            field_type,
            general: fields,
            max_version,
        }
    }

    pub fn get_fields(&self, version: usize) -> Vec<Field> {
        self.general.clone().into_iter().filter(|x| x.version <= version).collect()
    }

    pub fn get_checksum(&self, max_version: usize) -> ChecksumInfo {
        let mut combined_string = String::from("");

        self.general.iter().for_each(|field| {
            if field.version <= max_version && (field.max_version >= max_version || field.max_version == 0) {
                combined_string.push_str(field.combined_string)
            }
        });

        ChecksumInfo { checksum: CRC32.checksum(combined_string.as_bytes()) as i32, version: max_version, field_type: self.field_type, arena_type: self.arena_type }
    }

    // TODO: MAKE MORE EFFICIENT!
    pub fn generate_all_checksums(&mut self) -> Vec<ChecksumInfo> {
        let mut checksums = Vec::new();
        for i in 0..(self.max_version + 1) {
            checksums.push(self.get_checksum(i));
        }
        
        checksums
    }

    fn should_insert(field: &Field, field_type: FieldType) -> bool {
        (field.field_type == field_type) || 
        (field.field_type == FieldType::AccountAll && field_type == FieldType::AccountSelf) ||
        (field.field_type == FieldType::VehicleAll && field_type == FieldType::VehicleSelf)
    }
}