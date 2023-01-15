use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use walkdir::WalkDir;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse dat file", |b| b.iter(parse_files));
}

fn parse_files() {
    let all_entries = parse_dir("input_files");
    let entries: Vec<_> = all_entries.choose_multiple(&mut rand::thread_rng(), 5).collect();

    for entry in entries {
        let file = std::fs::read(entry.path()).expect("unable to read the file");

        let parser = wot_datfile_parser::DatFileParser::new();
        let result = parser.parse(&file).unwrap();

        let _s = serde_json::to_string_pretty(&result).unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn parse_dir(path: &str) -> Vec<walkdir::DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) if entry.path().is_file() => Some(entry),
            _ => None,
        })
        .collect()
}
