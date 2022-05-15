use std::{
    fs::{create_dir_all, DirEntry, File},
    io::Write,
};

use anyhow::Result;
use wot_replay_parser::events::{AsPacket, BattleEvent, EventPrinter};

pub fn main() -> Result<()> {
    let files =
        utils::parse_dir("/home/dacite/Projects/wot-battle-results-parser/replay_parser/input_files")?;

    for file in files {
        analyze(file)?;
    }

    Ok(())
}

fn analyze(dir_entry: DirEntry) -> wot_replay_parser::Result<()> {
    println!("{:?}", dir_entry.file_name());
    let read_file = File::open(dir_entry.path())?;

    create_dir_all("results")?;
    create_dir_all("results_json")?;

    let mut write_file = File::options().write(true).create(true).open(format!(
        "results/{}.txt",
        dir_entry.file_name().into_string().unwrap()
    ))?;

    let write_file_json = File::options().write(true).create(true).open(format!(
        "results_json/{}.json",
        dir_entry.file_name().into_string().unwrap()
    ))?;

    let parser = wot_replay_parser::ReplayParser::parse(read_file)?;
    let battle_context = parser.battle_context();
    serde_json::to_writer_pretty(write_file_json, parser.get_json())?;
    for event in parser.event_stream()? {
        match event {
            Ok(event) => {
                if let BattleEvent::EntityMethod(method) = &event {
                    if method.is_unknown() {
                        writeln!(
                            write_file,
                            "{} {}",
                            wot_replay_parser::get_replay_time(
                                battle_context.get_start_time() as f64,
                                event.as_packet().get_time() as f64
                            ),
                            method.to_debug_string(&battle_context)
                        )?;
                        writeln!(write_file, "{:+4?}\n", event.as_packet())?;
                    }
                }
                // if !event.is_unknown() {
                //     writeln!(write_file, "{:?}", &event)?;
                // }
            }
            Err(err) => eprintln!("{:?}", err),
        }
    }

    Ok(())
}
