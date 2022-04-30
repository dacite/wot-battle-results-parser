use std::io::{Cursor};


use macros::ToPacket;
use getset::Getters;

use crate::{packet_stream::{Packet}, event::{PacketParser, ToPacket, battle_event::BattleEvent}};

#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct Unknown {
    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}


impl PacketParser for Unknown {
    fn parse(packet: &Packet) -> BattleEvent {
        let inner = packet.get_seekable_vec(); 

        BattleEvent::Unimplemented(Self { inner })
    }
}

