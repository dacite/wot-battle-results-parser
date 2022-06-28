use std::{collections::BTreeMap, path::Path};

use anyhow::Result;
use wot_replay_parser::packet_parser::{Packet, PacketStream};

macro_rules! write_packet {
    ($packet_type: expr, $vec: expr, $packet_stream: expr) => {
        for packet in $packet_stream {
            let packet = packet.unwrap();
            if packet.get_type() == $packet_type {
                $vec.append(&mut packet.get_inner().to_vec());
                $vec.append(&mut vec!['*' as u8; 32 - packet.get_inner().len() % 32]);
            }
        }
    };
}

pub fn main() -> Result<()> {
    let replay_files = utils::parse_dir(Path::new("replay_parser/input_files"))?;

    let mut vec = Vec::new();
    for replay_file in replay_files {
        println!("\n{:?}", replay_file.file_name());
        let replay_file = std::fs::read(replay_file.path())?;
        let (_json, binary) = wot_replay_parser::parse(&replay_file)?;

        let packet_stream = PacketStream::new(&binary);
        write_packet!(0x08, vec, packet_stream.clone());

        on_packet_stream(packet_stream.clone());

        let _packets = packet_stream.clone().fold(BTreeMap::new(), |mut acc, packet| {
            let mut value = 1;

            if let Some(current) = acc.get(&packet.as_ref().unwrap().get_type()) {
                value = *current;
            }

            acc.insert(packet.unwrap().get_type(), value);
            acc
        });

        // println!("{:?}", &packets);
    }
    std::fs::write("across_replays", vec)?;

    Ok(())
}

fn on_packet_stream(packet_stream: PacketStream) {
    for packet in packet_stream {
        on_packet(&packet.unwrap());
    }
}
fn on_packet(packet: &Packet) {
    if packet.get_type() == 0x16 {
        print_packet_data(packet);
    }
}

fn print_packet_data(packet: &Packet) {
    let p: String = packet
        .get_inner()
        .into_iter()
        .map(|byte| format!("{:02X?}", byte))
        .collect();

    println!("{p}")
}
