use std::path::PathBuf;

use rayon::prelude::*;
use wot_replay_parser::{ReplayError, ReplayParser};

// This is a real world use case of the library. I used it to sort replays I downloaded from wotreplays.eu
// into folders based on version and map
pub fn main() -> Result<(), ReplayError> {
    let paths = utils::parse_dir("/home/dacite/Projects/wot-battle-results-parser/replays").unwrap();

    // Use rayon to multithread this workload
    paths
        .par_iter()
        .for_each(|path| copy_replay(path.path()));

    Ok(())
}

fn copy_replay(path: PathBuf) {
    println!("{}", path.display());
    let parser = ReplayParser::parse_file(&path).unwrap();
    let json = parser.replay_json_start().unwrap();

    let region = json.pointer("/regionCode").unwrap().as_str().unwrap();
    let recorder_vehicle = json.pointer("/playerVehicle").unwrap().as_str().unwrap();
    let map = json.pointer("/mapName").unwrap().as_str().unwrap();
    let arena_unique_id = parser.parse_arena_unique_id().unwrap();

    let version = parser.parse_replay_version().unwrap();
    let version = utils::version_as_string(version);

    let dir_location = format!("test/replays/{}/{}", version, map);
    let replay_name = format!("{}_{}_{}", region, arena_unique_id, recorder_vehicle);

    let copy_path = format!("{}/{}", dir_location, replay_name);
    println!("{}", &copy_path);
    std::fs::create_dir_all(&dir_location).unwrap();
    std::fs::copy(path, copy_path).unwrap();
}
