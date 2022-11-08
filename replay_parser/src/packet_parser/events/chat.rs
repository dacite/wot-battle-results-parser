use nom::number::complete::le_u32;

use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct Chat {
    msg: String,
}

impl PacketParser for Chat {
    fn parse(packet: &Packet, _context: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();
        let (remaining, msg_length) = le_u32(data)?;

        let msg_buffer = &remaining[..msg_length as usize];

        let msg = std::str::from_utf8(msg_buffer).unwrap();

        Ok(BattleEvent::Chat(Chat { msg: msg.to_string() }))
    }
}
