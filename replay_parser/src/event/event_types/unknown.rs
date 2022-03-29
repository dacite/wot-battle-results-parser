use std::io::{Cursor, SeekFrom, Seek};

use byteorder::{LittleEndian, ReadBytesExt};
use macros::ToPacket;
use getset::Getters;

use crate::{packet_stream::{Packet, METADATA_SIZE}, event::{PacketParser, ToPacket, TargetableEvent, battle_event::BattleEvent, EventPrinter, BattleInfo}};

#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct Unknown {
    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}


impl PacketParser for Unknown {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec(); 

        BattleEvent::Unimplemented(Self { inner })
    }
}

impl EventPrinter for Unknown {
    fn to_string(&self, _: &BattleInfo) -> String {
        format!("Unknown event: {:+?}", &self.get_as_packet())
    }
}