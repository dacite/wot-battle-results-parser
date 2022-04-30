use std::io::{Cursor, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use super::battle_event::BattleEvent;
use crate::packet_stream::PacketStream;

pub struct EventStream<'a> {
    inner: PacketStream<'a>,
}

impl<'a> EventStream<'a> {
    pub fn new(inner: PacketStream<'a>) -> Self {
        Self {
            inner: inner.into_iter(),
        }
    }
}

impl<'a> Iterator for EventStream<'a> {
    type Item = BattleEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(packet) = self.inner.next() {
            return Some(BattleEvent::new(packet));
        }

        None
    }
}
