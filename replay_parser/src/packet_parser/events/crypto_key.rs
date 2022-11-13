use nom::number::complete::le_u32;

use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
/// Looks a lot like a blowfish key but no idea what it is
pub struct CryptoKey {
    pub key: String,
}

impl PacketParser for CryptoKey {
    fn parse(packet: &Packet, _: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (remaining, size) = le_u32(data)?;

        if remaining.len() != size as usize {
            return Err(PacketError::UnconsumedInput);
        }

        let key = String::from_utf8(remaining.to_vec())?;

        Ok(BattleEvent::CryptoKey(CryptoKey { key }))
    }
}
