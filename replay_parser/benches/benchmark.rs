use std::path::Path;

use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use wot_replay_parser::event::BattleEvent;
use wot_replay_parser::{parse, parse_json};


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Replay: Parse JSON Only", |b| b.iter(parse_json_only));
    c.bench_function("Replay: Parse in Entirety", |b| b.iter(parse_entire_replay));
}

fn parse_json_only() {
    let files = utils::parse_dir(Path::new(
        "/home/dacite/Projects/wot-battle-results-parser/replay_parser/input_files",
    ))
    .unwrap();
    for entry in files {
        let file = std::fs::read(entry.path()).unwrap();
        let _result = parse_json(&file).unwrap();
    }
}

fn parse_entire_replay() {
    let files = utils::parse_dir(Path::new(
        "/home/dacite/Projects/wot-battle-results-parser/replay_parser/input_files",
    ))
    .unwrap();
    for entry in files {
        let file = std::fs::read(entry.path()).unwrap();
        let _result = parse(&file).unwrap();
    }
}

fn parse_events(bin_stream: &[u8]) {
    let stream = wot_replay_parser::packet_stream::PacketStream::new(bin_stream);
    let packets: Vec<_> = stream.into_iter().collect();

    let _events: Vec<_> = packets.iter().map(|packet| BattleEvent::new(packet)).collect();
}

pub fn criterion_benchmark_events(c: &mut Criterion) {
    let file = std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/input_files/20220312_2330_uk-GB98_T95_FV4201_Chieftain_59_asia_great_wall.wotreplay").unwrap();
    let result = parse(&file).unwrap();

    let mut group = c.benchmark_group("Event parsing overhead");
    group.sample_size(5000);
    group.bench_with_input(BenchmarkId::new("input_example", "ok"), &result.1, |b, s| {
        b.iter(|| parse_events(s));
    });
    group.finish();
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(event_parsing, criterion_benchmark_events);
criterion_main!(event_parsing);
