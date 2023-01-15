use std::path::Path;

use walkdir::WalkDir;
use wot_datfile_parser::{Battle, DatFileParser};

#[cfg(test)]
mod tests {
    use wot_datfile_parser::DatFileParser;

    use super::*;

    #[test]
    fn datfile_parser_test() {
        let parser = DatFileParser::new();

        let datfiles = parse_dir("input_files");

        datfiles.into_iter().for_each(|datfile| {
            let battle = parse_datfile(datfile.path(), &parser);
            assert!(serde_json::to_string_pretty(&battle).is_ok())
        });
    }
}

pub fn parse_dir(path: &str) -> Vec<walkdir::DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) if entry.path().is_file() => Some(entry),
            _ => None,
        })
        .collect()
}

/// Parse a single .dat file
pub fn parse_datfile(path: &Path, parser: &DatFileParser) -> Battle {
    println!("Parsing {}", path.display());
    let file = std::fs::read(path).unwrap();

    parser.parse(&file).unwrap()
}
