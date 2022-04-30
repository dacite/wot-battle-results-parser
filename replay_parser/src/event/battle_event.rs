use enum_as_inner::EnumAsInner;
use enum_dispatch::enum_dispatch;

use super::{
    event_types::{
Unknown,
    },
    PacketParser,
};
use crate::packet_stream::{Packet};


#[derive(Debug, EnumAsInner, Clone)]
#[enum_dispatch(ToPacket, EventPrinter)]
#[non_exhaustive]
pub enum BattleEvent {
    Unimplemented(Unknown),
}

impl BattleEvent {
    pub fn new(packet: Packet) -> Self {
        match packet.get_type() {
            // 0x0A => PositionUpdate::parse(packet),
            // 0x23 => Chat::parse(packet),
            // 0x16 => ArenaStatusUpdate::parse(packet),
            // 0x08 => DiscreteEvent::parse(packet),
            _ => Unknown::parse(packet),
        }
    }
}


// struct DiscreteEvent;

// impl PacketParser for DiscreteEvent {
//     fn parse(packet: Packet) -> self::BattleEvent {
//         if let Some(subtype) = packet.get_subtype() {
//             match subtype {
//                 0x00 => ShotFired::parse(packet),
//                 0x02 => DamageReceived::parse(packet),
//                 0x08 => ShotTook::parse(packet),
//                 _ => Unknown::parse(packet),
//             }
//         } else {
//             Unknown::parse(packet)
//         }
//     }
// }
