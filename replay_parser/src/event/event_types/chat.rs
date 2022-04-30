use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use getset::Getters;
use macros::ToPacket;

use crate::{
    event::{battle_event::BattleEvent, BattleInfo, EventPrinter, PacketParser, TargetableEvent, ToPacket},
    packet_stream::{Packet, METADATA_SIZE},
};

#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct Chat {
    msg:   String,
    #[derivative(Debug = "ignore")]
    inner: Cursor<Vec<u8>>,
}


impl PacketParser for Chat {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();

        inner.seek(SeekFrom::Start(METADATA_SIZE)).unwrap();
        let msg_length = inner.read_u32::<LittleEndian>().unwrap() as usize;

        let mut msg_buffer: Vec<u8> = vec![0; msg_length];

        inner.read(&mut msg_buffer).unwrap();
        let msg = String::from_utf8(msg_buffer).unwrap();

        inner.set_position(0);

        BattleEvent::Chat(Self { msg, inner })
    }
}

impl EventPrinter for Chat {
    fn to_string(&self, _: &BattleInfo) -> String {
        format!("Chat: {}", self.msg.clone())
    }
}
