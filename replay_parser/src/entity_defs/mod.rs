mod avatar;
mod entity_types;

pub use avatar::{AvatarProperties, AVATAR_PROPS};
pub use entity_types::{find_entity_type, EntityType, ENTITY_TYPE_MAP};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum EntityProperty {
    AvatarProperties(AvatarProperties),
    Unimplemented,
}

pub trait VariantDeserializer {
    fn deserialize_variant<'de, D>(discim: &'static str, d: D) -> core::result::Result<Self, D::Error>
    where
        Self: Sized,
        D: serde::de::Deserializer<'de>;
}
