use std::path::Path;

use rayon::prelude::*;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::*, Registry};
use walkdir::WalkDir;
use wot_replay_parser::{BattleEvent, ReplayError, ReplayParser};

pub fn main() -> Result<(), ReplayError> {
    let formatting_layer = BunyanFormattingLayer::new("wot_replay_parser".into(), std::io::stderr);
    let subscriber = Registry::default().with(JsonStorageLayer).with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();


    // We use WalkDir because replays are in nested folders
    let replay_entries: Vec<_> = WalkDir::new("/home/dacite/Projects/wot-battle-results-parser/replays")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                Some(entry)
            } else {
                None
            }
        })
        .collect();

    // We use rayon to use multiple cores when parsing replays
    replay_entries
        .par_iter()
        .for_each(|path| parse_replay(path.path()));

    Ok(())
}

fn parse_replay<T: AsRef<Path> + std::fmt::Debug>(path: T) {
    let parser = ReplayParser::parse_file(&path).unwrap();
    let context = wot_replay_parser::Context::new(parser.parse_replay_version().unwrap());

    // Here we use packet stream to get each packet and then convert into the event
    // However we can use parser.event_stream() directly as well
    for packet in parser.packet_stream() {
        let packet = packet.unwrap();

        if let Ok(event) = BattleEvent::parse(&packet, &context) {
            println!("{:?}", event)
        }
    }
}
