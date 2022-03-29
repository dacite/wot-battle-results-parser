pub mod battle_event;
pub mod event_stream;
pub mod event_types;
use std::{io::{SeekFrom, Cursor, Seek}, collections::HashMap};

use byteorder::{LittleEndian, ReadBytesExt};
use enum_dispatch::enum_dispatch;
use time::Duration;

use crate::packet_stream::Packet;
pub use event_types::*;
pub use battle_event::BattleEvent;

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
pub trait ToPacket {
    fn get_all_data(&self) -> &[u8];
    fn get_as_packet(&self) -> Packet {
        Packet::new(self.get_all_data())
    }
}

pub trait PacketParser {
    fn parse(packet: Packet) -> battle_event::BattleEvent;
}

#[enum_dispatch]
pub trait EventPrinter: ToPacket {
    fn to_string(&self, battle_info: &BattleInfo) -> String;

    fn print(&self, battle_info: &BattleInfo) {
        let time = get_replay_time(battle_info.get_start_time() as f64, self.get_as_packet().get_time() as f64);

        println!("{} {}", time, self.to_string(battle_info))
    }
}

pub struct BattleInfo {
    pub players: HashMap<u32, String>,
    pub start_time: f32,
}

impl BattleInfo {
    pub fn get_player(&self, avatar_id: u32) -> Option<&String> {
        self.players.get(&avatar_id).clone()
    }

    pub fn get_start_time(&self) -> f32 {
        self.start_time
    }
}

fn get_replay_time(start_time: f64, current_time: f64) -> String {
    let total_time = Duration::minutes(15);

    let actual_time = total_time - Duration::seconds_f64(current_time - start_time);

    format!("{}:{}", actual_time.whole_minutes(), actual_time.whole_seconds() % 60)
}

