use std::io::{Cursor, SeekFrom, Seek};
use getset::Getters;
use byteorder::{LittleEndian, ReadBytesExt};
use macros::ToPacket;

use crate::{packet_stream::{Packet, METADATA_SIZE}, event::{PacketParser, ToPacket, TargetableEvent, battle_event::BattleEvent, EventPrinter, BattleInfo}};

use super::{PositionUpdateVariant, Unknown};

/// `(EventSource(u32), Unknown(u32), x(f32), z(f32), y(f32), ...)`
#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct ShotTook {
    took_by: u32,
    took_from: u32,

    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}

impl PacketParser for ShotTook {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();
        inner.seek(SeekFrom::Start(METADATA_SIZE)).unwrap();
        
        let took_by = inner.read_u32::<LittleEndian>().unwrap();
        inner.seek(SeekFrom::Current(8)).unwrap();

        let took_from = inner.read_u32::<LittleEndian>().unwrap();

        inner.set_position(0);
        
        BattleEvent::ShotTook(Self {
            took_by, took_from, inner
        })
    }
}

impl TargetableEvent for ShotTook {
    fn get_event_data(&self) -> &[u8] {
        &self.inner.get_ref()[METADATA_SIZE as usize..]
    }
}

impl EventPrinter for ShotTook {
    fn to_string(&self, battle_info: &BattleInfo) -> String {
        let took_by = battle_info.get_player(self.took_by);
        let took_from = battle_info.get_player(self.took_from);

        if let Some(by) = took_by {
            if let Some(from) = took_from {
                format!("{} took a shot from {} {:+?}", by.clone(), from.clone(), self.get_as_packet())
            } else {
                format!("Undecipherable Shot Received Event because 'from' cannot be identified {:+?}", self.get_as_packet())
            }
        } else {
            format!("Undecipherable Shot Received Event {:+?}", self.get_as_packet())
        }
    }
}