use nom::number::complete::le_u32;

use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct Chat {
    pub msg: String,
}

impl PacketParser for Chat {
    fn parse(packet: &Packet, _: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (remaining, msg_length) = le_u32(data)?;
        let msg_buffer = &remaining[..msg_length as usize];
        let msg = String::from_utf8_lossy(msg_buffer);

        Ok(BattleEvent::Chat(Chat { msg: msg.into() }))
    }
}
