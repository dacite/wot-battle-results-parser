use macros::{EventPrinter, Version};
use nom::number::complete::{le_i32, le_u16, le_u32};
use serde::{de, Deserialize, Serialize};
use standard_format::WotValue;

use super::{battle_event::Version, event_stream::UpdateContext};
use super::{event_stream::Context, BattleEvent, EventPrinter, PacketParser};
use crate::packet_parser::Packet;
use crate::Result;
// todo: how can this be inside macro (VersionInfo)
use crate::{events::VersionInfo, packet_parser::serde_packet};

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct AvatarCreate {
    #[serde(skip)]
    entity_id: i32,

    name: String,

    #[version([1, 7, 0, 0])]
    session_id: Option<String>,

    arena_unique_id:  u64,
    arena_type_id:    i32,
    arena_bonus_type: u8,
    arena_gui_type:   u8,

    #[serde(deserialize_with = "deserialize_pickle")]
    arena_extra_data: WotValue, // Pickle

    weather_preset_id:  u8,
    denunciations_left: i16, // Complaints left
}

impl PacketParser for AvatarCreate {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent> {
        let data = packet.get_payload();
        let (remaining, entity_id) = le_i32(data)?;
        let (remaining, _entity_type) = le_u16(remaining)?;
        let (remaining, _unknown) = le_i32(remaining)?;
        let (remaining, size) = le_u32(remaining)?;

        assert!(remaining.len() == size as usize);

        let mut avatar_create: AvatarCreate =
            serde_packet::from_slice_unchecked(remaining, context.get_version())?;

        avatar_create.entity_id = entity_id;

        println!("{}", serde_json::to_string_pretty(&avatar_create).unwrap());

        Ok(BattleEvent::AvatarCreate(avatar_create))
    }
}

impl UpdateContext for AvatarCreate {
    fn update_context(&self, context: &mut Context) {
        context.add_entity(self.entity_id, "Avatar");
    }
}

fn deserialize_pickle<'de, D>(deserializer: D) -> core::result::Result<WotValue, D::Error>
where
    D: de::Deserializer<'de>,
{
    let vec: &[u8] = de::Deserialize::deserialize(deserializer)?;
    let pickle =
        serde_pickle::value_from_slice(&vec, serde_pickle::DeOptions::new().replace_unresolved_globals())
            .unwrap();

    let wot_value = serde_pickle::from_value(pickle).map_err(|e| serde::de::Error::custom(e.to_string()))?;
    Ok(wot_value)
}
