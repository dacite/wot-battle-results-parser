use macros::{EventPrinter, Version};
use serde::Deserialize;

use super::entity_method::Vector3;
use super::{event_stream::Context, BattleEvent, PacketParser};
use super::{EventPrinter, Version, VersionInfo};
use crate::packet_parser::{serde_packet, Packet};
use crate::Result;

#[derive(Debug, Clone, Deserialize, Version, EventPrinter)]
// Credit: https://github.com/Monstrofil/replays_unpack/blob/master/replay_unpack/core/packets/Position.py
pub struct Position {
    pub entity_id:      i32,
    pub vehicle_id:     i32,
    pub position:       Vector3,
    pub position_error: Vector3,
    pub yaw:            f32,
    pub pitch:          f32,
    pub roll:           f32,
    pub is_volatile:    u8,
}

impl PacketParser for Position {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent> {
        let position = serde_packet::from_slice(packet.get_payload(), context.get_version())?;

        Ok(BattleEvent::Position(position))
    }
}
