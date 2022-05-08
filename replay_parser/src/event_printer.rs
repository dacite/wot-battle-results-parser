use std::path::Path;

use anyhow::Result;
use wot_replay_parser::{
    events::{AsPacket, EventPrinter},
    get_replay_time,
    packet_parser::{self, events::BattleEvent},
    BattleContext,
};

pub fn main() -> Result<()> {
    let replay_path =
        Path::new("replay_parser/input_files/20220421_0117_china-Ch44_114_SP2_45_north_america.wotreplay");

    let replay_file = std::fs::read(replay_path)?;
    let (json, binary) = wot_replay_parser::parse(&replay_file)?;

    let packet_stream = wot_replay_parser::packet_parser::PacketStream::new(&binary);
    let battle_context = BattleContext::from(&json, &binary);
    for packet in packet_stream {
        let event = packet_parser::parse(&packet).unwrap();

        if let BattleEvent::Unimplemented(_) = event {
            continue;
        }

        if let BattleEvent::EntityMethod(entity_method) = &event {
            if entity_method.is_unknown() {
                continue;
            }
        }

        println!(
            "{} {}",
            get_replay_time(
                battle_context.get_start_time() as f64,
                event.as_packet().get_time() as f64
            ),
            event.to_debug_string(&battle_context)
        );
        println!("{:+?}", packet);
    }


    Ok(())
}
