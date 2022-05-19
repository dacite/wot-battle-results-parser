pub mod entity;
mod types;
pub mod utils;

pub use types::TypeAliasLookup;


pub trait Size {
    /// Size in bytes of this structure
    fn get_size(&self) -> u64;
}
