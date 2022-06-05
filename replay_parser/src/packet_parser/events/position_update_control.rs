use std::io::{Cursor, SeekFrom, Seek};
use getset::Getters;
use byteorder::{LittleEndian, ReadBytesExt};
use macros::ToPacket;

use crate::{packet_stream::{Packet, METADATA_SIZE}, event::{PacketParser, ToPacket, TargetableEvent, battle_event::BattleEvent, EventPrinter, BattleInfo}};

/// `(EventSource(u32), Unknown(u32), x(f32), z(f32), y(f32), ...)`
#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct PositionUpdateVariant {
    one: u32,
    two: u32,
    three: u32,

    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}

impl PacketParser for PositionUpdateVariant {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();
        inner.seek(SeekFrom::Start(METADATA_SIZE + 8)).unwrap();

        let one = inner.read_u32::<LittleEndian>().unwrap();
        let two = inner.read_u32::<LittleEndian>().unwrap();
        let three = inner.read_u32::<LittleEndian>().unwrap();

        inner.set_position(0);
        
        BattleEvent::PositionUpdateControl(Self {
            one, two, three, inner
        })
    }
}

impl TargetableEvent for PositionUpdateVariant {
    fn get_event_data(&self) -> &[u8] {
        &self.inner.get_ref()[METADATA_SIZE as usize..]
    }
}

impl EventPrinter for PositionUpdateVariant {
    fn to_string(&self, _: &BattleInfo) -> String {
        format!("PUVariant")
    }

}