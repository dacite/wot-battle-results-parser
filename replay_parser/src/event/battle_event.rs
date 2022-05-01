use anyhow::Result;
use enum_as_inner::EnumAsInner;
use enum_dispatch::enum_dispatch;

use super::{GameVersion, PacketParser, Unknown};
use crate::packet_stream::{Packet, PacketMetadata};


#[derive(Debug, EnumAsInner, Clone)]
#[enum_dispatch(PacketMetadata)]
#[non_exhaustive]
pub enum BattleEvent<'pkt> {
    Unimplemented(Unknown<'pkt>),
    GameVersion(GameVersion<'pkt>),
}

impl<'pkt> BattleEvent<'pkt> {
    pub fn new(packet: &'pkt Packet) -> Result<Self> {
        match packet.get_type() {
            0x18 => GameVersion::parse(packet),
            _ => Unknown::parse(packet),
        }
    }
}
