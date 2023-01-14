use std::{fs, path::Path};

use log::info;
use wot_datfile_parser::{Battle, DatFileParser};

#[cfg(test)]
mod tests {
    use std::path::Path;

    use wot_datfile_parser::DatFileParser;

    #[test]
    fn datfile_parser_test() {
        env_logger::builder().is_test(true).try_init().unwrap();
        let parser = DatFileParser::new();

        let mut battles = super::parse_dir(Path::new("input_files/test"), &parser);
        battles.append(&mut super::parse_dir(Path::new("input_files/other"), &parser));
        battles.append(&mut super::parse_dir(
            Path::new("input_files/WOT_1_16_1_0"),
            &parser,
        ));
        battles.append(&mut super::parse_dir(
            Path::new("input_files/WOT_1_17_0_0"),
            &parser,
        ));

        battles
            .into_iter()
            .for_each(|battle| assert!(serde_json::to_string_pretty(&battle).is_ok()));
    }
}

/// Parse a directory of .dat files (only direct childs of the directory)
pub fn parse_dir(path: &Path, parser: &DatFileParser) -> Vec<Battle> {
    let file_paths = fs::read_dir(path).unwrap();

    let mut vec = Vec::new();

    for path_result in file_paths {
        match path_result {
            Ok(path) => {
                if path.path().is_file() {
                    vec.push(parse_datfile(path.path().as_path(), parser));
                }
            }
            Err(e) => {
                println!("Failed to process DirEntry: {}", e);
                continue;
            }
        }
    }

    vec
}

/// Parse a single .dat file
pub fn parse_datfile(path: &Path, parser: &DatFileParser) -> Battle {
    info!("Parsing {}", &path.to_string_lossy());
    let file = std::fs::read(path).unwrap();

    parser.parse(&file).unwrap()
}
