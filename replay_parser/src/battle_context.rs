use std::collections::HashMap;

use byteorder::{ReadBytesExt, LE};
use time::Duration;

use crate::PacketStream;

/// Store information about a battle so that it can be used for debugging.
pub struct BattleContext {
    players:    HashMap<i32, String>,
    start_time: f32,
}

impl BattleContext {
    pub fn from(json: &[serde_json::Value], binary_stream: &[u8]) -> Self {
        let players = get_player_list(&json[0]);

        let packet_stream = PacketStream::new(binary_stream);
        let start_time = get_battle_start_time(packet_stream);

        BattleContext { players, start_time }
    }

    pub fn entity_id_to_player(&self, id: i32) -> Option<String> {
        self.players.get(&id).map(|name| name.to_owned())
    }

    pub fn get_start_time(&self) -> f32 {
        self.start_time
    }
}

fn get_player_list(json: &serde_json::Value) -> HashMap<i32, String> {
    let mut player_list = HashMap::new();
    let vehicles = json["vehicles"].as_object().unwrap();
    for i in vehicles.into_iter() {
        let avatar_id = i.0.parse::<i32>().unwrap();
        let name = i.1["name"].as_str().unwrap();
        let tank = i.1["vehicleType"].as_str().unwrap();

        player_list.insert(avatar_id, format!("{}, {}", name, tank));
    }
    player_list
}

fn get_battle_start_time(packet_stream: PacketStream) -> f32 {
    for packet in packet_stream {
        let packet = packet.unwrap();
        if packet.get_type() == 0x16 && packet.get_payload().read_u32::<LE>().unwrap() == 3 {
            return packet.get_time();
        }
    }

    -1.0
}

pub fn get_replay_time(start_time: f64, current_time: f64) -> String {
    let total_time = Duration::minutes(15);

    let actual_time = total_time - Duration::seconds_f64(current_time - start_time);

    format!(
        "{}:{}",
        actual_time.whole_minutes(),
        actual_time.whole_seconds() % 60
    )
}
