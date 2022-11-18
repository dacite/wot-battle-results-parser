mod entity;
mod types;
pub mod utils;

pub use entity::{Entity, Method, Property};
pub use types::{OpaqueType, TypeAliasLookup, WotType};
pub type Result<T> = core::result::Result<T, std::io::Error>;

pub trait Size {
    /// Size in bytes of this structure
    fn get_size(&self) -> u64;
}

///////////////////////////////////////////////////////////////////////////////////////////////
// Implementations of the trait `Size`
//////////////////////////////////////////////////////////////////////////////////////////////
///
impl Size for WotType {
    fn get_size(&self) -> u64 {
        match self {
            WotType::OpaqueType(ty) => ty.get_size(),
            WotType::Array(_) => u16::MAX as u64,
            WotType::FixedDict { is_nullable, dict } => {
                if *is_nullable {
                    u16::MAX as u64
                } else {
                    dict.values().fold(0, |acc, ty| acc + ty.get_size())
                }
            }
        }
    }
}

impl Size for OpaqueType {
    fn get_size(&self) -> u64 {
        use OpaqueType::*;

        match self {
            U8 | I8 => 1,
            U16 | I16 => 2,
            F32 | U32 | I32 => 4,
            F64 | U64 | I64 | Vector2 => 8,
            Vector3 => 12,
            Vector4 => 16,
            String | Pickle | UserType => u16::MAX as u64,
            MailBox => 12, // this is 12 in bigworld docs
            Alias(ty) => ty.get_size(),
        }
    }
}

impl Size for Method {
    fn get_size(&self) -> u64 {
        let mut size = self.get_params().iter().fold(0, |acc, x| acc + x.get_size());

        if size >= u16::MAX as u64 {
            size = u16::MAX as u64 + self.get_variable_header_size().unwrap_or(1) as u64;
        } else {
            size += self.get_variable_header_size().unwrap_or(1) as u64;
        }

        size
    }
}

impl Size for Property {
    fn get_size(&self) -> u64 {
        self.ty.get_size()
    }
}
