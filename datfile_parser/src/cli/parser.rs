use std::{fs, path::Path};

use anyhow::{anyhow, Context, Result};
use log::info;
use standard_format::Battle;
use wot_datfile_parser::DatFileParser;

/// High level function that parses directly from the wot cache folder
pub fn parse_from_wot_data_folder(
    wot_cache_folder: Option<String>, parser: &DatFileParser, out_dir: &str,
) -> Result<()> {
    let wot_data_folder_path = if let Some(path) = wot_cache_folder {
        // User specified the folder for the dat files
        Path::new(&path).to_path_buf()
    } else {
        // Programs find the dat files automaticaly
        let path = directories::BaseDirs::new().unwrap();
        path.data_dir()
            .join(Path::new("Wargaming.net/WorldOfTanks/battle_results"))
    };


    let wot_data_folder =
        fs::read_dir(&wot_data_folder_path).with_context(|| "Cannot read wot data folder path")?;

    for player_folder_result in wot_data_folder {
        if let Ok(player_folder) = player_folder_result {
            let player_folder_path = player_folder.path();
            let decoded_name = base_32_decode(&player_folder_path)?;

            let output_path = format!("{}/{}", out_dir, &decoded_name);
            fs::create_dir_all(&output_path)?;

            let parsed_battles = parse_dir(player_folder_path.as_path(), parser)?;
            parsed_battles.into_iter().for_each(|battle| match battle {
                Ok(battle) => {
                    let output_file_name = format!("{}.json", &battle.arena_unique_id);
                    write_battle(battle, &format!("{}/{}", &output_path, &output_file_name));
                }
                Err(e) => {
                    println!("Parsing dat file result in errors: {}", e);
                }
            });
        } else {
            println!("Cannot read player folder");
        }
    }

    Ok(())
}

/// Parse a directory of .dat files (only direct childs of the directory)
pub fn parse_dir(path: &Path, parser: &DatFileParser) -> Result<Vec<Result<Battle>>> {
    let file_paths = fs::read_dir(path).with_context(|| "failed to read dir")?;

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

    Ok(vec)
}

/// Parse a single .dat file
pub fn parse_datfile(path: &Path, parser: &DatFileParser) -> Result<Battle> {
    info!("Parsing {}", &path.to_string_lossy());
    let file =
        std::fs::read(path).with_context(|| format!("Cannot read in file at {}", path.to_string_lossy()))?;

    parser.parse(&file)
}

/// .dat files are organized under a folder where its name is encoded in base32.
fn base_32_decode(input: &Path) -> Result<String> {
    let base_32_name = input
        .file_name()
        .unwrap()
        .to_str()
        .ok_or_else(|| anyhow!("base 32 decode error for {}", input.to_string_lossy()))?;
    let actual_name_buffer =
        base32::decode(base32::Alphabet::RFC4648 { padding: false }, base_32_name).unwrap();

    Ok(String::from_utf8(actual_name_buffer)?
        .split(';')
        .next()
        .unwrap()
        .to_string())
}

/// Convert battle to json and write to file
pub fn write_battle(battle: Battle, path: &str) {
    match serde_json::to_vec_pretty(&battle) {
        // JSON conversion successful
        Ok(buf) => {
            let _ = fs::write(path, buf).map_err(|e| println!("Failed to write file: {}", e));
        }
        Err(e) => println!("Converting battle to JSON failed: {}", e),
    }
}
