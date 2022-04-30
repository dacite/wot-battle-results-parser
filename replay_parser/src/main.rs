use std::{
    collections::{HashMap}, path::Path, fs::{self, DirEntry}, time::Instant,
};

// use image::{RgbImage, Rgb, RgbaImage, Rgba};
use time::Duration;
use anyhow::{Result, Context};
use wot_replay_parser::{
    self,
};

fn main() {
    let files = parse_dir(Path::new("/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples")).unwrap();
    for entry in files {
        let file = std::fs::read(entry.path()).unwrap();
        let _result = wot_replay_parser::parse(&file).unwrap();
    }

    let file = std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples/20220306_0941_france-F108_Panhard_EBR_105_45_north_america.wotreplay").unwrap(); // let file =
    let files = parse_dir(Path::new("/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples")).unwrap();
    
    let mut vec = Vec::new();
    for entry in files {
        let now = Instant::now();
        let file = std::fs::read(entry.path()).unwrap();
        println!("Reading took {}", now.elapsed().as_millis());

        let now = Instant::now();
        let result = wot_replay_parser::parse(&file).unwrap();
        if entry.path() == Path::new("/home/dacite/Projects/wot-battle-results-parser/replay_parser/examples/20220306_0941_france-F108_Panhard_EBR_105_45_north_america.wotreplay") {
            // println!("{}", serde_json::to_string(&result.0).unwrap());
        }
        println!("Parsing took {}", now.elapsed().as_millis());
        let now = Instant::now();
        let stream = wot_replay_parser::packet_stream::PacketStream::new(&result.1);
        println!("Creating stream took {}", now.elapsed().as_millis());

        let now = Instant::now();
        let results: Vec<_> = stream
        .into_iter()
        .filter(|x| x.get_type() == 0x00)
        // .filter(|x| true)
        .collect();
        println!("Filtering took {}", now.elapsed().as_millis());

        let now = Instant::now();
        for i in &results {

            println!(
                "{:+?}",
                i.clone(),
            );
            vec.append(&mut i.get_data().to_vec());
            let mut rest = 32 - (i.get_data().len() % 32);
            if rest == 32 {
                rest = 0;
            }
            println!("REST: {}", rest);
            println!("LEN: {}", i.get_data().len());
            vec.append(&mut vec!['*' as u8; rest]);
            // vec.append(&mut vec!['=' as u8; 32]);
        }
        println!("Printing took {}", now.elapsed().as_millis());
        // break;
    }

    
    std::fs::write("test.hex", vec);

}

// fn get_player_list(json: serde_json::Value) -> HashMap<u32, String> {
//     let mut player_list = HashMap::new();
//     let vehicles = json["vehicles"].as_object().unwrap();
//     for i in vehicles.into_iter() {
//         let avatar_id = i.0.parse::<u32>().unwrap();
//         let name = i.1["name"].as_str().unwrap();
//         let tank = i.1["vehicleType"].as_str().unwrap();
//         player_list.insert(
//             avatar_id,
//             [name.to_string(), ", ".to_string(), tank.to_string()].concat(),
//         );
//     }
//     player_list
// }

// fn get_battle_start_time(packet_stream: &[BattleEvent]) -> f32 {
//     for mut event in packet_stream.into_iter() {
//         if let BattleEvent::ArenaStatusUpdate(info) = event {
//             if info.status.clone() as i32 == 3 {
//                 return event.get_as_packet().get_time();
//             }
//         }
//     }
//     return -1.0;
// }

// fn get_replay_time(start_time: f64, current_time: f64) -> String {
//     let total_time = Duration::minutes(15);

//     let actual_time = total_time - Duration::seconds_f64(current_time - start_time);

//     format!("{}:{}", actual_time.whole_minutes(), actual_time.whole_seconds() % 60)
// }

/// Parse a directory of .dat files (only direct childs of the directory)
pub fn parse_dir(path: &Path) -> Result<Vec<DirEntry>> {
    let file_paths = fs::read_dir(path).with_context(|| format!("failed to read dir"))?;

    Ok(file_paths
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .collect())
}