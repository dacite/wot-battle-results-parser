use std::io::{Cursor, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use getset::Getters;
use macros::ToPacket;

use super::{PositionUpdateVariant, Unknown};
use crate::{
    event::{battle_event::BattleEvent, EventPrinter, PacketParser, TargetableEvent, ToPacket},
    packet_stream::{Packet, METADATA_SIZE},
};

/// `(EventSource(u32), Unknown(u32), x(f32), z(f32), y(f32), ...)`
#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct ShotFired {
    from: u32,

    #[derivative(Debug = "ignore")]
    inner: Cursor<Vec<u8>>,
}

impl PacketParser for ShotFired {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();
        inner.seek(SeekFrom::Start(METADATA_SIZE)).unwrap();

        let from = inner.read_u32::<LittleEndian>().unwrap();

        inner.set_position(0);

        BattleEvent::ShotFired(Self { from, inner })
    }
}

impl TargetableEvent for ShotFired {
    fn get_event_data(&self) -> &[u8] {
        &self.inner.get_ref()[METADATA_SIZE as usize..]
    }
}

impl EventPrinter for ShotFired {
    fn to_string(&self, battle_info: &crate::event::BattleInfo) -> String {
        let player = battle_info.get_player(self.from).unwrap().clone();
        format!("Shot fired by {} {:+?}", player, self.get_as_packet())
    }
}
