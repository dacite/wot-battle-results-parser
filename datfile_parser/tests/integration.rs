use std::{path::Path, fs};

use log::info;
use standard_format::Battle;
use wot_datfile_parser::DatFileParser;
use anyhow::{Result, Context};

#[cfg(test)]
mod tests {
    use std::path::Path;

    use wot_datfile_parser::DatFileParser;

    #[test]
    fn datfile_parser_test() {
        env_logger::builder().is_test(true).try_init().unwrap();
        let parser = DatFileParser::new();
        let mut battles = super::parse_dir(Path::new("input_files/test"), &parser).unwrap();
        battles.append(&mut super::parse_dir(Path::new("input_files/other"), &parser).unwrap());

        battles.into_iter().for_each(|battle| match battle {
            Ok(battle) => {
                assert!(serde_json::to_string_pretty(&battle).is_ok());
            }
            Err(e) => {
                println!("Parsing dat file result in errors: {}", e);
                panic!("Test failed");
            }
        });
    }
}

/// Parse a directory of .dat files (only direct childs of the directory)
pub fn parse_dir(path: &Path, parser: &DatFileParser) -> Result<Vec<Result<Battle>>> {
    let file_paths = fs::read_dir(path).with_context(|| format!("failed to read dir"))?;

    let mut vec = Vec::new();

    for path_result in file_paths {
        match path_result {
            Ok(path) => {
                if path.path().is_file() {
                    vec.push(parse_datfile(path.path().as_path(), parser));
                }
            }
            Err(e) => {
                println!("Failed to process DirEntry: {}", e.to_string());
                continue;
            }
        }
    }

    Ok(vec)
}

/// Parse a single .dat file
pub fn parse_datfile(path: &Path, parser: &DatFileParser) -> Result<Battle> {
    info!("Parsing {}", &path.to_string_lossy());
    let file = std::fs::read(path).with_context(|| format!("Cannot read in file at {}", path.to_string_lossy()))?;

    parser.parse(&file)
}