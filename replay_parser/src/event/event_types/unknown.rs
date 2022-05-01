use anyhow::Result;
use getset::Getters;
use macros::PacketMetadata;

use crate::{
    event::{battle_event::BattleEvent, PacketParser},
    packet_stream::{Packet, PacketMetadata},
};

#[derive(derivative::Derivative, PacketMetadata, Getters, Clone)]
#[derivative(Debug)]
pub struct Unknown<'pkt> {
    #[derivative(Debug = "ignore")]
    inner: &'pkt [u8],
}


impl<'pkt> PacketParser<'pkt> for Unknown<'pkt> {
    fn parse(packet_data: &'pkt Packet) -> Result<BattleEvent<'pkt>> {
        let inner = packet_data.get_inner();

        Ok(BattleEvent::Unimplemented(Self { inner }))
    }
}
