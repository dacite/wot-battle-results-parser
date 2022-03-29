use std::io::{Cursor, SeekFrom, Seek};
use getset::Getters;
use byteorder::{LittleEndian, ReadBytesExt};
use macros::ToPacket;

use crate::{packet_stream::{Packet, METADATA_SIZE}, event::{PacketParser, ToPacket, TargetableEvent, battle_event::BattleEvent, EventPrinter, BattleInfo}};


/// `(EventSource(u32), Unknown(u32), x(f32), z(f32), y(f32), ...)`
#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct DamageReceived {
    received_by: u32,
    received_from: u32,
    
    before: u16,
    after: u16,

    damage_type: u8,

    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}

impl PacketParser for DamageReceived {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();
        inner.seek(SeekFrom::Start(METADATA_SIZE)).unwrap();

        let received_by = inner.read_u32::<LittleEndian>().unwrap();
        inner.seek(SeekFrom::Current(8)).unwrap();

        let after = inner.read_u16::<LittleEndian>().unwrap();
        let before = inner.read_u16::<LittleEndian>().unwrap();
        let received_from = inner.read_u32::<LittleEndian>().unwrap();
        let damage_type = inner.read_u8().unwrap();

        inner.set_position(0);
        
        BattleEvent::DamageReceived(Self {
            received_by, received_from, before, after, damage_type, inner
        })
    }
}

impl TargetableEvent for DamageReceived {
    fn get_event_data(&self) -> &[u8] {
        &self.inner.get_ref()[METADATA_SIZE as usize..]
    }
}

impl EventPrinter for DamageReceived {
    fn to_string(&self, battle_info: &BattleInfo) -> String {
        let received_by = battle_info.get_player(self.received_by);
        let received_from = battle_info.get_player(self.received_from);
        let damage = self.before as u32 - self.after as u32;

        if let Some(by) = received_by {
            if let Some(from) = received_from {
                format!("{} took {} damage from {} and now has {} health {:+?}", by.clone(), damage, from.clone(), self.after, self.get_as_packet())
            } else {
                format!("Undecipherable Damage Received Event because 'from' cannot be identified {:+?}", self.get_as_packet())
            }
        } else {
            format!("Undecipherable Damage Received Event {:+?}", self.get_as_packet())
        }
    }
}