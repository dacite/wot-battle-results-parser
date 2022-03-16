use criterion::{criterion_group, criterion_main, Criterion};


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse dat file", |b| b.iter(parse_files));
}

fn parse_files() {
    let entries = std::fs::read_dir("examples").unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }

        let file = std::fs::read(path).expect("unable to read the file");

        let parser = wot_datfile_parser::DatFileParser::new();
        let result = parser.parse(&file).unwrap();

        let _s= serde_json::to_string_pretty(&result).unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);