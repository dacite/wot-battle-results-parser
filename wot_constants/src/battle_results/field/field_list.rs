use crate::{ArenaBonusType, battle_results::ALL_TYPES};
use super::{ChecksumInfo, FieldType, Field};

pub const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

#[derive(Clone)]
pub struct FieldsVec {
    arena_type: ArenaBonusType,
    field_type: FieldType,

    general: Vec<Field>,
    arena_specific: Vec<Field>,
    
    version: (usize, usize)
}

impl FieldsVec {
    pub fn new(arena_type: ArenaBonusType, field_type: FieldType) -> FieldsVec {
        
        let (general_fields, general_version) = FieldsVec::generate(ALL_TYPES, field_type);

        let mut arena_specific_fields = Vec::new();
        let mut arena_specific_version: usize = 0;

        if let Some(fields) = arena_type.get_collection() {
            let result = FieldsVec::generate(fields, field_type);
            arena_specific_fields = result.0;
            arena_specific_version = result.1;
        }

        return FieldsVec {
            arena_type,
            field_type,
            general: general_fields,
            arena_specific: arena_specific_fields,
            version: (general_version, arena_specific_version),
        };
    }

    pub fn get_fields(&self) -> Vec<Field> {
        return [self.general.clone(), self.arena_specific.clone()].concat()
    }

    pub fn get_checksum(&self) -> ChecksumInfo {
        let mut combined_string = String::from("");

        self.general.iter().for_each(|field| combined_string.push_str(field.combined_string));
        self.arena_specific.iter().for_each(|field| combined_string.push_str(field.combined_string));

        println!("Combined: {}", &combined_string);

        ChecksumInfo { checksum: CRC32.checksum(combined_string.as_bytes()) as i32, version: self.version, field_type: self.field_type, arena_type: self.arena_type }
    }

    pub fn generate_all_checksums(&mut self) -> Vec<ChecksumInfo> {
        let oldest_version = (0, 0);
        let mut all_checksums = Vec::new();
        for i in (oldest_version.0)..(self.version.0 + 1) {
            for j in (oldest_version.1)..(self.version.1 + 1) {
                self.restore((i, j));
                all_checksums.push(self.get_checksum());
            }
        }

        all_checksums
    }

    pub fn restore(&mut self, restore_version: (usize, usize)) {
        let current_version = self.version;

        // Upgrade/Downgrade General Fields
        if current_version.0 < restore_version.0 {
            Self::upgrade1(ALL_TYPES, &mut self.general, current_version.0, restore_version.0)
        } else if current_version.0 > restore_version.0 {
            Self::downgrade1(ALL_TYPES, &mut self.general, current_version.0, restore_version.0)
        }

        //Upgrade/Downgrade Arena Specific Fields
        if let Some(additional) = self.arena_type.get_collection() {
            if current_version.1 < restore_version.1 {
                Self::upgrade1(additional, &mut self.arena_specific, current_version.0, restore_version.0)
            } else if current_version.1 > restore_version.1 {
                Self::downgrade1(additional, &mut self.arena_specific, current_version.0, restore_version.0)
            }
        }

        self.version = restore_version;
    }

    fn upgrade1(src: &[&[Field]], target: &mut Vec<Field>, current_version: usize, upgrade_version: usize) {
        if current_version >= upgrade_version {
            panic!("incorrect use of upgrade");
        }

        for i in (current_version + 1)..(upgrade_version + 1) {
            src[i].iter().for_each(|field| target.insert(field.original_idx, field.clone()));
        }

    }

    fn downgrade1(src: &[&[Field]], target: &mut Vec<Field>, current_version: usize, downgrade_version: usize) {
        if current_version <= downgrade_version {
            panic!("incorrect use of upgrade");
        }

        for i in ((downgrade_version + 1)..(current_version + 1)).rev() {
            src[i].iter().for_each(|field| {target.remove(field.original_idx);});
        }

    }
    fn upgrade(&mut self, dest: &mut FieldsVec, version: (usize, usize)) {
        // Check if version is smaller than max possible version
        // Check if version is smaller than current version
        if version.0 < ALL_TYPES.len()  || version.0 < self.version.0 {
            panic!("Incorrect use of upgrade");
        }

        // Upgrade general fields
        for i in (self.version.0 + 1)..(version.0 + 1) {
            ALL_TYPES[i].iter().for_each(|field|self.general.insert(field.original_idx, field.clone()));
        }
        
        // Upgrade arena specific fields
        if let Some(fields) = self.arena_type.get_collection() {
            if fields.len() >= (version.1 + 1) || self.version.1 <= (version.1) {
                panic!("Incorrect use of upgrade");
            }
            for i in (self.version.1 + 1)..(version.1 + 1) {
                fields[i].iter().for_each(|field|self.arena_specific.insert(field.original_idx, field.clone()));
            }
        }

    }

    fn downgrade(&mut self, dest: &mut FieldsVec, version: (usize, usize)) {
        // Check if version is smaller than max possible version
        // Check if version is smaller than current version
        if version.0 < ALL_TYPES.len() || self.version.0 > (version.0) {
            panic!("Incorrect use of downgrade");
        }

        // Downgrade general fields
        for i in (((version.0 + 1)..self.version.0 + 1)).rev() {
            ALL_TYPES[i].iter().for_each(|field| {self.general.remove(field.original_idx);});
        }
        
        // Downgrade arena specific fields
        if let Some(fields) = self.arena_type.get_collection() {
            if fields.len() >= (version.1 + 1) || self.version.1 <= (version.1) {
                panic!("Incorrect use of upgrade");
            }
            for i in (((version.1 + 1)..self.version.1 + 1)).rev() {
                fields[i].iter().for_each(|field| {self.arena_specific.remove(field.original_idx);});
            }
        }
    }

    fn generate(fields: &[&[Field]], field_type: FieldType) -> (Vec<Field>, usize){
        let mut fields_vec = Vec::new();
        let mut version: i64 = -1;

        for fields in fields {
            for field in fields.iter() {
                if FieldsVec::should_insert(field, field_type) {
                    if version <= 0 {
                        fields_vec.push(field.clone());
                    } else {
                        fields_vec.insert(field.original_idx, field.clone());
                    }
                }
                
            }
            version += 1;
        }

        return (fields_vec, usize::try_from(version).expect("unexpected error: usize should be atleast zero here"));
    }

    fn should_insert(field: &Field, field_type: FieldType) -> bool {
        if field.field_type == field_type {
            return true;
        } else if field.field_type == FieldType::AccountAll && field_type == FieldType::AccountSelf {
            return true;
        } else if field.field_type == FieldType::VehicleAll && field_type == FieldType::VehicleSelf {
            return true;
        } else {
            return  false;
        }
    }
}