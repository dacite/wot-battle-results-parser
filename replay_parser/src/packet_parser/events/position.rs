use std::io::{Cursor, SeekFrom, Seek};
use getset::Getters;
use byteorder::{LittleEndian, ReadBytesExt};
use macros::ToPacket;

use crate::{packet_stream::{Packet, METADATA_SIZE}, event::{PacketParser, ToPacket, TargetableEvent, battle_event::BattleEvent, EventPrinter, BattleInfo}};

use super::PositionUpdateVariant;

/// `(EventSource(u32), Unknown(u32), x(f32), z(f32), y(f32), ...)`
#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct PositionUpdate {
    x: f32,
    y: f32,
    z: f32,

    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}

impl PacketParser for PositionUpdate {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();

        // Check for variant
        // TODO: Better logic for handling variants
        inner.seek(SeekFrom::Start(METADATA_SIZE + 4)).unwrap();
        let test = inner.read_u32::<LittleEndian>().unwrap();
        if test != 0 {
            return PositionUpdateVariant::parse(packet);
        }

        let x = inner.read_f32::<LittleEndian>().unwrap();
        let z = inner.read_f32::<LittleEndian>().unwrap();
        let y = inner.read_f32::<LittleEndian>().unwrap();

        inner.set_position(0);
        
        BattleEvent::PositionUpdate(Self {
            x, y, z, inner
        })
    }
}

impl TargetableEvent for PositionUpdate {
    fn get_event_data(&self) -> &[u8] {
        &self.inner.get_ref()[METADATA_SIZE as usize..]
    }
}

impl EventPrinter for PositionUpdate {
    fn to_string(&self, battle_info: &BattleInfo) -> String {
        if let Some(player) = battle_info.get_player(self.get_event_source()) {
            format!("Position Update [{}]: x: {}, y: {}, z: {}", player.clone(), self.x, self.y, self.z)
        } else {
            format!("Position Update [{}]: x: {}, y: {}, z: {}", self.get_event_source(), self.x, self.y, self.z)
        }
    }
}