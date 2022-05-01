pub mod battle_event;
pub mod event_types;

use anyhow::Result;
pub use battle_event::BattleEvent;
use byteorder::{LittleEndian, ReadBytesExt};
use enum_dispatch::enum_dispatch;
pub use event_types::*;

use crate::packet_stream::Packet;

pub trait TargetableEvent {
    fn get_event_data(&self) -> &[u8];

    fn get_event_source(&self) -> u32 {
        let mut payload = self.get_event_data();
        if payload.len() >= 4 {
            payload.read_u32::<LittleEndian>().unwrap()
        } else {
            0
        }
    }
}


#[enum_dispatch]
pub trait PacketParser<'a> {
    fn parse(packet: &'a Packet) -> Result<BattleEvent<'a>>;
}
