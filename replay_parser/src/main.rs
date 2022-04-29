// use std::{io::Write, collections::{HashSet, HashMap}, rc::Rc,};
// // use image::{RgbImage, Rgb, RgbaImage, Rgba};

// use time::Duration;
// use wot_replay_parser::{
//     self,
//     event::{ToPacket, TargetableEvent, battle_event::BattleEvent, BattleInfo,
// EventPrinter}, };
// fn main() {
//     let file =
// std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/
// examples/20220306_0941_france-F108_Panhard_EBR_105_45_north_america.
// wotreplay").unwrap();     // let file =
// std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/
// examples/20220302_2332_germany-G42_Maus_45_north_america.wotreplay").
// unwrap();     // let file =
// std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/
// examples/example.wotreplay").unwrap();     // let file =
// std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/
// examples/20160603_2346_germany-G120_M41_90_GrandFinal_11_murovanka.wotreplay"
// ).unwrap();     let result = wot_replay_parser::parse(&file).unwrap();
//     std::fs::write("result.json",
// result.0[1].to_string().as_bytes()).unwrap();

//     // let timer = Instant::now();
//     let stream =
// wot_replay_parser::packet_stream::PacketStream::new(&result.1);
//     let events =
// wot_replay_parser::event::event_stream::EventStream::new(stream);

//     let player_list = get_player_list(result.0[0].clone());
//     let events_vec: Vec<BattleEvent> = events.into_iter().collect();
//     let start_time = get_battle_start_time(&events_vec);

//     let battle_info = BattleInfo {
//         players: player_list,
//         start_time,
//     };

//     let results: Vec<BattleEvent> = events_vec.into_iter()
//         // .filter(|x| x.get_as_packet().get_type() == 0x08)
//         .filter(|x| true)
//         .collect();

//     let mut tick = 0.0;
//     for i in results {
//         if let BattleEvent::ShotFired(e) = i.clone() {
//             i.print(&battle_info);
//         }
//         // println!("{} {:+?}", get_replay_time(start_time as f64,
// i.get_as_packet().get_time() as f64), i.clone().get_as_packet(),);     }

//     // let duration = timer.elapsed();

//     // println!("Time elapsed: {}", duration.as_millis());
// }

// fn get_player_list(json: serde_json::Value) -> HashMap<u32, String> {
//     let mut player_list = HashMap::new();
//     let vehicles = json["vehicles"].as_object().unwrap();
//     for i in vehicles.into_iter() {
//         let avatar_id = i.0.parse::<u32>().unwrap();
//         let name = i.1["name"].as_str().unwrap();
//         let tank = i.1["vehicleType"].as_str().unwrap();
//         player_list.insert(avatar_id, [name.to_string(), ", ".to_string(),
// tank.to_string()].concat());     }
//     player_list
// }

// fn get_battle_start_time (packet_stream: &[BattleEvent]) -> f32 {
//     for mut event in packet_stream.into_iter() {
//             if let BattleEvent::ArenaStatusUpdate(info) = event {
//                 if info.status.clone() as i32 == 3{
//                     return  event.get_as_packet().get_time();
//                 }
//             }
//     }
//     return -1.0;
// }

// fn get_replay_time(start_time: f64, current_time: f64) -> String {
//     let total_time = Duration::minutes(15);

//     let actual_time = total_time - Duration::seconds_f64(current_time -
// start_time);

//     format!("{}:{}", actual_time.whole_minutes(), actual_time.whole_seconds()
// % 60) }

pub fn main() {
    println!("Not done");
}
