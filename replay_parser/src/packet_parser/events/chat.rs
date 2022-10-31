use macros::{EventPrinter, Version};
use nom::number::complete::le_u32;
use serde::{Deserialize, Serialize};

use super::battle_event::Version;
use super::{event_stream::Context, BattleEvent, EventPrinter, PacketParser};
// todo: how can this be inside macro (VersionInfo)
use crate::events::VersionInfo;
use crate::packet_parser::Packet;
use crate::Result;

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct Chat {
    msg: String,
}


impl PacketParser for Chat {
    fn parse(packet: &Packet, _context: &Context) -> Result<BattleEvent> {
        let data = packet.get_payload();
        let (remaining, msg_length) = le_u32(data)?;

        let msg_buffer = &remaining[..msg_length as usize];

        let msg = std::str::from_utf8(msg_buffer).unwrap();

        Ok(BattleEvent::Chat(Chat { msg: msg.to_string() }))
    }
}
