use std::io::Cursor;

use getset::Getters;
use macros::ToPacket;

use crate::{
    event::{battle_event::BattleEvent, PacketParser, ToPacket},
    packet_stream::Packet,
};

#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct Unknown {
    #[derivative(Debug = "ignore")]
    inner: Cursor<Vec<u8>>,
}


impl PacketParser for Unknown {
    fn parse(packet: &Packet) -> BattleEvent {
        let inner = packet.get_seekable_vec();

        BattleEvent::Unimplemented(Self { inner })
    }
}
