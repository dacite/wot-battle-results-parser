mod avatar;
pub use avatar::{AvatarProperties, AVATAR_PROPS};

mod entity_types;
pub use entity_types::{find_entity_type, EntityType, ENTITY_TYPE_MAP};

mod vehicle;
use serde::{Deserialize, Serialize};
pub use vehicle::{VehicleProperties, VEHICLE_PROPS};

use crate::PacketError;


#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum EntityProperty {
    AvatarProperties(AvatarProperties),
    VehicleProperties(VehicleProperties),
    Unimplemented,
}

pub trait VariantDeserializer {
    fn deserialize_variant<'de, D>(discim: &'static str, d: D) -> core::result::Result<Self, D::Error>
    where
        Self: Sized,
        D: serde::de::Deserializer<'de>;
}

pub trait PropertyParser {
    type Discrim;
    fn parse(input: &[u8], version: [u16; 4], discim: Self::Discrim) -> Result<Self, PacketError>
    where
        Self: Sized;
}


pub const VERSIONS: &'static [[u16; 4]] = &[
    [0, 9, 12, 0],
    [0, 9, 13, 0],
    [0, 9, 14, 0],
    [0, 9, 15, 0],
    [0, 9, 16, 0],
    [0, 9, 17, 0],
    [0, 9, 18, 0],
    [0, 9, 19, 0],
    [0, 9, 20, 0],
    [0, 9, 21, 0],
    [0, 9, 22, 0],
    [0, 9, 23, 0],
    [1, 0, 0, 0],
    [1, 0, 1, 0],
    [1, 0, 02, 0],
    [1, 1, 0, 0],
    [1, 2, 0, 0],
    [1, 3, 0, 0],
    [1, 4, 0, 0],
    [1, 4, 1, 0],
    [1, 5, 0, 0],
    [1, 5, 1, 0],
    [1, 6, 0, 0],
    [1, 6, 1, 0],
    [1, 7, 0, 0],
    [1, 7, 1, 0],
    [1, 8, 0, 0],
    [1, 9, 0, 0],
    [1, 10, 0, 0],
    [1, 10, 1, 0],
    [1, 11, 0, 0],
    [1, 11, 1, 0],
    [1, 12, 0, 0],
    [1, 12, 1, 0],
    [1, 13, 0, 0],
    [1, 14, 0, 0],
    [1, 14, 1, 0],
    [1, 15, 0, 0],
    [1, 16, 0, 0],
    [1, 16, 1, 0],
    [1, 17, 0, 0],
    [1, 17, 1, 0],
    [1, 18, 0, 0],
    [1, 18, 1, 0],
    [1, 19, 0, 0],
    [1, 19, 1, 0],
];
