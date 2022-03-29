use std::io::{Cursor, SeekFrom, Seek};
use getset::Getters;
use byteorder::{LittleEndian, ReadBytesExt};
use macros::ToPacket;

use crate::{packet_stream::{Packet, METADATA_SIZE}, event::{PacketParser, ToPacket, TargetableEvent, battle_event::BattleEvent, EventPrinter, BattleInfo}};

use anyhow::anyhow;

/// `(StatusFlag(u32))`
#[derive(derivative::Derivative, ToPacket, Getters, Clone)]
#[derivative(Debug)]
pub struct ArenaStatusUpdate {
    pub status: ArenaStatus,

    #[derivative(Debug="ignore")]
    inner: Cursor<Vec<u8>>,
}

impl PacketParser for ArenaStatusUpdate {
    fn parse(packet: Packet) -> BattleEvent {
        let mut inner = packet.get_seekable_vec();
        inner.seek(SeekFrom::Start(METADATA_SIZE)).unwrap();

        let val = inner.read_u32::<LittleEndian>().unwrap();
        let status = ArenaStatus::try_from(val).unwrap();

        inner.set_position(0);
        
        BattleEvent::ArenaStatusUpdate(Self {
            status, inner
        })
    }
}

impl EventPrinter for ArenaStatusUpdate {
    fn to_string(&self, _: &BattleInfo) -> String {
        match self.status {
            ArenaStatus::Waiting => format!("Waiting for players"),
            ArenaStatus::Countdown => format!("Countdown starts now!"),
            ArenaStatus::Rollout => format!("Rollout!"),
        }
    }
}


#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub enum ArenaStatus {
    Waiting = 1,
    Countdown = 2,
    Rollout = 3,
}

impl TryFrom<u32> for ArenaStatus {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ArenaStatus::Waiting),
            2 => Ok(ArenaStatus::Countdown),
            3 => Ok(ArenaStatus::Rollout),
            _ => Err(anyhow!("unknown arena status flag"))
        }
    }
}