use getset::Getters;

use super::{AsPacket, BattleEvent, EventPrinter, PacketParser};
use crate::packet_parser::Packet;
use crate::{BattleContext, Result};

#[derive(derivative::Derivative, Getters, Clone)]
#[derivative(Debug)]
/// Represents an event that we don't know how to parse yet.
pub struct Unknown<'pkt> {
    #[derivative(Debug = "ignore")]
    inner: Packet<'pkt>,
}

impl<'pkt> PacketParser<'pkt> for Unknown<'pkt> {
    fn parse(packet: Packet<'pkt>, _version: [u16; 4]) -> Result<BattleEvent<'pkt>> {
        Ok(BattleEvent::Unimplemented(Self { inner: packet }))
    }
}

impl AsPacket for Unknown<'_> {
    fn as_packet(&self) -> &Packet {
        &self.inner
    }
}

impl EventPrinter for Unknown<'_> {
    fn to_debug_string(&self, _context: &BattleContext) -> String
    where
        Self: std::fmt::Debug + AsPacket,
    {
        String::from("Unknown")
    }
}
