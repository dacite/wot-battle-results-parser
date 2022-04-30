use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
