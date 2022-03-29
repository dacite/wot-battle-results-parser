// use super::{EventData};

use std::{io::{Cursor, Seek, SeekFrom}, rc::Rc};

use byteorder::{ReadBytesExt, LittleEndian};
use enum_as_inner::EnumAsInner;
use enum_dispatch::enum_dispatch;

use crate::packet_stream::{Packet, METADATA_SIZE};

use super::{TargetableEvent, event_types::{PositionUpdate, Unknown, Chat, PositionUpdateVariant, ArenaStatusUpdate, ShotFired, DamageReceived, ShotTook}, PacketParser, ToPacket};


pub enum BattleEventId {
    Generic,
    PositionUpdate = 10,
}

#[derive(Debug, EnumAsInner, Clone)]
#[enum_dispatch(ToPacket, EventPrinter)]
#[non_exhaustive]
pub enum BattleEvent {
    PositionUpdate(PositionUpdate),
    Chat(Chat),
    Unimplemented(Unknown),
    PositionUpdateControl(PositionUpdateVariant),
    ArenaStatusUpdate(ArenaStatusUpdate),
    ShotFired(ShotFired),
    DamageReceived(DamageReceived),
    ShotTook(ShotTook)
}

impl BattleEvent {
    pub fn new(packet: Packet) -> Self {
        match packet.get_type() {
            0x0A => PositionUpdate::parse(packet),
            0x23 => Chat::parse(packet),
            0x16 => ArenaStatusUpdate::parse(packet),
            0x08 => DiscreteEvent::parse(packet),
            _ => Unknown::parse(packet),
        }
    }
}





// impl ToPacket for BattleEvent {
//     fn get_all_data(&self) -> &[u8] {
//         match self {
//             BattleEvent::PositionUpdate(x) => x.get_all_data(),
//             BattleEvent::Unimplemented(x) => x.get_all_data(),
//             BattleEvent::Chat(x) => x.get_all_data(),
//             BattleEvent::PositionUpdateControl(x) => x.get_all_data(),
//             BattleEvent::ArenaStatusUpdate(x) =>  x.get_all_data(),
//             BattleEvent::Eight01(x) =>  x.get_all_data(),
//             BattleEvent::ShotFired(x) => x.get_all_data(),
            
//         }
//     }
// }

impl TargetableEvent for Rc<dyn ToPacket> {
    fn get_event_data(&self) -> &[u8] {
        &self.get_all_data()[METADATA_SIZE as usize..]
    }
}

struct DiscreteEvent;

impl PacketParser for DiscreteEvent {
    fn parse(packet: Packet) -> self::BattleEvent {
        if let Some(subtype) = packet.get_subtype() {
            match subtype {
                0x00 => ShotFired::parse(packet),
                0x02 => DamageReceived::parse(packet),
                0x08 => ShotTook::parse(packet),
                _ => Unknown::parse(packet)
            }
        } else {
            Unknown::parse(packet)
        }
    }
}








