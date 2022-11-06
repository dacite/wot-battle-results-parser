use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::*, Registry};
use wot_replay_parser::wot_types::ArenaBonusType;
use wot_replay_parser::{ReplayError, ReplayParser};

pub fn main() -> Result<(), ReplayError> {
    let formatting_layer = BunyanFormattingLayer::new("wot_replay_parser".into(), std::io::stderr);
    let subscriber = Registry::default().with(JsonStorageLayer).with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let paths = utils::parse_dir("/home/dacite/Projects/wot-battle-results-parser/replays").unwrap();
    for path in paths {
        parse_replay(path.path().as_os_str().to_str().unwrap())
    }
    Ok(())
}

fn parse_replay(path: &str) {
    let parser = ReplayParser::parse_file(path).unwrap();
    if let ArenaBonusType::Regular = parser.battle_type() {
        for event in parser.event_stream().unwrap() {
            match event {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        }
    }
}
