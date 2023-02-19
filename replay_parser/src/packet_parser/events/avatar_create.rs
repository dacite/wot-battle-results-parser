use serde::de;

use crate::entity_defs::EntityType;
use crate::packet_parser::prelude::*;
use crate::wot_types::WotValue;
#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct AvatarCreate {
    pub entity_id:   i32,
    pub entity_type: u16,

    #[version([0, 9, 14, 0])]
    _padding: Option<u32>, // not exactly sure what this is

    _size:    u32,
    pub name: String,

    #[version([1, 7, 0, 0])]
    session_id: Option<String>,

    pub arena_unique_id:  i64,
    pub arena_type_id:    i32,
    pub arena_bonus_type: u8,
    pub arena_gui_type:   u8,

    #[serde(deserialize_with = "deserialize_pickle")]
    pub arena_extra_data: WotValue, // Pickle

    pub weather_preset_id:  u8,
    pub denunciations_left: i16, // Complaints left
}


impl PacketParser for AvatarCreate {
    fn parse_mut(packet: &Packet, context: &mut Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (_, avatar_create) = from_slice_unchecked::<AvatarCreate>(data, context.get_version())?;

        context.add_entity(avatar_create.entity_id, EntityType::Avatar); // TODO: Use entity type map here

        Ok(BattleEvent::AvatarCreate(avatar_create))
    }
}


fn deserialize_pickle<'de, D>(deserializer: D) -> core::result::Result<WotValue, D::Error>
where
    D: de::Deserializer<'de>,
{
    let vec: &[u8] = de::Deserialize::deserialize(deserializer)?;
    let pickle =
        serde_pickle::value_from_slice(vec, serde_pickle::DeOptions::new().replace_unresolved_globals())
            .unwrap();

    let wot_value = serde_pickle::from_value(pickle).map_err(|e| serde::de::Error::custom(e.to_string()))?;
    Ok(wot_value)
}
