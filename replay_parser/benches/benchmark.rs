use std::{
    fs::{self, DirEntry},
    path::Path,
};

use anyhow::{Context, Result, ensure};
use criterion::{criterion_group, criterion_main, Criterion};
use wot_replay_parser::{parse, parse_json};


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Replay: Parse JSON Only", |b| b.iter(parse_json_only));
    c.bench_function("Replay: Parse in Entirety", |b| b.iter(parse_entire_replay));
}

fn parse_json_only() {
    let files = parse_dir(Path::new(
        "/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples",
    ))
    .unwrap();
    for entry in files {
        let file = std::fs::read(entry.path()).unwrap();
        let _result = parse_json(&file).unwrap();
    }
}

fn parse_entire_replay() {
    let files = parse_dir(Path::new(
        "/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples",
    ))
    .unwrap();
    for entry in files {
        let file = std::fs::read(entry.path()).unwrap();
        let _result = parse(&file).unwrap();
    }
}

fn load_packet_id_and_time() {
    let files = parse_dir(Path::new(
        "/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples",
    ))
    .unwrap();

    let mut vec = Vec::new();
    for entry in files {
        let file = std::fs::read(Path::new("/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples/20220306_0941_france-F108_Panhard_EBR_105_45_north_america.wotreplay")).unwrap();
        let result = parse(&file).unwrap();

        let stream = wot_replay_parser::packet_stream::PacketStream::new(&result.1);

        for packet in stream {
            vec.push((packet.get_type(), packet.get_time()));
        }
        break;
    }
}
use nom::{bytes::complete::take, multi::many_till, combinator::eof};
use nom::multi::length_data;
use nom::number::complete::le_u32;
use nom::IResult;
pub fn length_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, length) = le_u32(input)?;
    take(length + 8)(input)
}

fn load_packet_id_and_time_nom() {
    let files = parse_dir(Path::new(
        "/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples",
    ))
    .unwrap();

    // let mut vec = Vec::new();
    for entry in files {
        let file = std::fs::read(Path::new("/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples/20220306_0941_france-F108_Panhard_EBR_105_45_north_america.wotreplay")).unwrap();
        let result = parse(&file).unwrap();

        let packets = many_till(length_value, eof)(result.1.as_slice()).unwrap();

        if packets.0.len() == 0 {

        } else {
            println!("FAILED");
        }
        break;
    }
}

/// Parse a directory of .dat files (only direct childs of the directory)
pub fn parse_dir(path: &Path) -> Result<Vec<DirEntry>> {
    let file_paths = fs::read_dir(path).with_context(|| format!("failed to read dir"))?;

    Ok(file_paths
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .collect())
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
