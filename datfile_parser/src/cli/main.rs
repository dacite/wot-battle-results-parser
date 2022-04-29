mod parser;


use std::{fs, path::Path};

use clap::{ArgGroup, Parser};
use parser::{parse_datfile, parse_from_wot_data_folder, write_battle};
use wot_datfile_parser::DatFileParser;

/// This is a parser for the `.dat` files generated by WoT client after a battle
/// when you view battle results. These files are usually found at
/// `C:\Users\$USER_NAME\AppData\Roaming\Wargaming.net\WorldOfTanks\
/// battle_results`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("input_format").args(&["file", "cache-folder"])))]
struct Cli {
    /// Use this option for parsing a single .dat file (Not recommended)
    #[clap(short, long)]
    file: Option<String>,

    /// World of Tanks Cache folder
    ///
    /// Example : `C:\Users\$USER_NAME\AppData\Roaming\Wargaming.net\
    /// WorldOfTanks\battle_results`
    ///
    /// Only use this option if the automatic setting is not working.
    /// This can be known by checking if the error says something like `Cannot
    /// read wot data folder path`
    #[clap(short, long)]
    cache_folder: Option<String>,

    /// Output directory for parsed results.
    /// If not specified this will be the same directory as the executable
    #[clap(short, long, default_value_t = String::from("parsed_battle_results"))]
    out_dir: String,
}

fn main() {
    env_logger::init();
    let parser = DatFileParser::new();
    let cli = Cli::parse();

    let Cli {
        file,
        cache_folder,
        out_dir,
    } = cli;

    match (file, cache_folder) {
        // User specifies to parse a single dat file
        (Some(single_file_path), _) => {
            let path_object = Path::new(&single_file_path);
            let battle = parse_datfile(path_object, &parser).unwrap();

            fs::create_dir_all(&out_dir).unwrap();
            let output_file_name = format!("{}.json", &battle.arena_unique_id);

            std::fs::create_dir_all(&out_dir).unwrap();
            let output_path = format!("{}/{}", &out_dir, &output_file_name);
            write_battle(battle, &output_path);
        }
        (_, cache_folder) => {
            parse_from_wot_data_folder(cache_folder, &parser, &out_dir).unwrap();
        }
    }
}
