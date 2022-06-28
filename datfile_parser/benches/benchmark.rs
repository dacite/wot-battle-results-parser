use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse dat file", |b| b.iter(parse_files));
}

fn parse_files() {
    let all_entries: Vec<_> = utils::parse_dir(Path::new("input_files")).unwrap();
    let entries: Vec<_> = all_entries.choose_multiple(&mut rand::thread_rng(), 5).collect();

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let file = std::fs::read(path).expect("unable to read the file");

        let parser = wot_datfile_parser::DatFileParser::new();
        let result = parser.parse(&file).unwrap();

        let _s = serde_json::to_string_pretty(&result).unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
