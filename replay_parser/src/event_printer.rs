use std::{
    fs::{create_dir_all, DirEntry, File},
    io::Write,
};

use anyhow::Result;
use wot_replay_parser::events::{BattleEvent, EventPrinter};

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
    create_dir_all("results_known")?;
    create_dir_all("results_json")?;
    create_dir_all("results_bin")?;
    let mut write_file = File::options().write(true).create(true).open(format!(
        "results/{}.txt",
        dir_entry.file_name().into_string().unwrap()
    ))?;

    let mut write_file_known = File::options().write(true).create(true).open(format!(
        "results_known/{}.txt",
        dir_entry.file_name().into_string().unwrap()
    ))?;

    let write_file_json = File::options().write(true).create(true).open(format!(
        "results_json/{}.json",
        dir_entry.file_name().into_string().unwrap()
    ))?;

    let mut write_file_bin = File::options().write(true).create(true).open(format!(
        "results_bin/{}.hex",
        dir_entry.file_name().into_string().unwrap()
    ))?;

    let parser = wot_replay_parser::ReplayParser::parse(read_file)?;
    let battle_context = parser.battle_context();
    serde_json::to_writer_pretty(write_file_json, parser.get_json())?;
    let mut vec = Vec::new();
    for packet in parser.packet_stream().unwrap() {
        let packet = packet.unwrap();
        vec.append(&mut packet.get_inner().to_vec());
        vec.append(&mut vec!['*' as u8; 32 - packet.get_inner().len() % 32]);
    }
    write_file_bin.write_all(&vec).unwrap();
    for event in parser.event_stream()? {
        match event {
            Ok(event) => {
                if let BattleEvent::EntityMethod(method) = event.event() {
                    if method.is_unknown() {
                        writeln!(
                            write_file,
                            "{} {}",
                            wot_replay_parser::get_replay_time(
                                battle_context.get_start_time() as f64,
                                event.packet().get_time() as f64
                            ),
                            method.to_debug_string(&battle_context)
                        )?;
                        writeln!(write_file, "{:+4?}\n", event.packet())?;
                    } else {
                        writeln!(
                            write_file_known,
                            "{} {}",
                            wot_replay_parser::get_replay_time(
                                battle_context.get_start_time() as f64,
                                event.packet().get_time() as f64
                            ),
                            method.to_debug_string(&battle_context)
                        )?;
                        writeln!(write_file_known, "{:+4?}\n", event.packet())?;
                    }
                } else {
                    write!(
                        write_file,
                        "{} ",
                        wot_replay_parser::get_replay_time(
                            battle_context.get_start_time() as f64,
                            event.packet().get_time() as f64
                        ),
                    )?;
                    writeln!(write_file, "{:+4?}\n", event.packet())?;
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
