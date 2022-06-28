mod details;
mod personal_vehicle;
mod player_info;
mod replay_common;
mod vehicle;

use std::collections::HashMap;

pub use details::Details;
pub use personal_vehicle::PersonalVehicle;
pub use player_info::PlayerInfo;
pub use replay_common::ReplayCommon;
use sqlx::{Pool, QueryBuilder, Sqlite};
pub use vehicle::Vehicle;
pub const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

#[derive(Default, Debug)]
pub struct Replay {
    replay_id:        u32,
    arena_unique_id:  Option<String>,
    common:           ReplayCommon,
    player_infos:     HashMap<i32, PlayerInfo>,
    personal_vehicle: HashMap<i32, PersonalVehicle>,
    vehicles:         HashMap<(i32, i32), Vehicle>,
}

impl Replay {
    pub fn new(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let parser = wot_replay_parser::ReplayParser::parse(file).unwrap();
        let json = parser.get_json();
        let replay_id = CRC32.checksum(path.as_bytes());

        let mut replay = Replay::default();
        replay.replay_id = replay_id;

        replay
            .parse_initial_info(json)
            .parse_arena_unique_id(json)
            .parse_player_performances(json)
            .update_player_info(json)
            .parse_personal_vehicles(json)
    }

    fn parse_initial_info(mut self, json: &[serde_json::Value]) -> Self {
        if json.len() <= 0 {
            // error case
        }

        let json = &json[0];
        let player_info_json = json.pointer("/vehicles").unwrap().as_object().unwrap();

        self.player_infos = player_info_json
            .into_iter()
            .map(|(avatar_id, info)| {
                let mut player_info: PlayerInfo = serde_json::from_value(info.clone()).unwrap();
                player_info.replay_id = self.replay_id;
                player_info.avatar_session_id = avatar_id.parse().unwrap();

                (player_info.avatar_session_id, player_info)
            })
            .collect();

        self.common = serde_json::from_value(json.clone()).unwrap();
        self.common.replay_id = self.replay_id;

        self
    }

    fn parse_arena_unique_id(mut self, json: &[serde_json::Value]) -> Self {
        if json.len() <= 1 {
            return self;
        }

        if let Some(arena_unique_id) = json[1].pointer("/0/arenaUniqueID") {
            self.arena_unique_id = Some(arena_unique_id.to_string());
        } else {
            // Eror case
        }

        self
    }

    fn parse_player_performances(mut self, json: &[serde_json::Value]) -> Self {
        if json.len() <= 1 {
            return self;
        }

        if let Some(vehicle_list) = json[1].pointer("/0/vehicles") {
            let vehicle_list = vehicle_list.as_object().unwrap();

            self.vehicles = vehicle_list
                .into_iter()
                .map(|(player_avatar_id, player_vehicles)| {
                    read_player_vehicles(
                        player_avatar_id.parse().unwrap(),
                        self.arena_unique_id.as_ref().unwrap().clone(),
                        player_vehicles.clone(),
                    )
                })
                .flatten()
                .collect();
        } else {
            // Error
        }

        self
    }

    fn update_player_info(mut self, json: &[serde_json::Value]) -> Self {
        if json.len() <= 1 {
            return self;
        }

        if self.vehicles.is_empty() {
            // Error case
        }

        // Update name (these maybe anonymized names which we need to switch to the real one)
        // Update account dbid
        // TODO: Update isAlive
        let json = &json[1];
        for ((avatar_id, _type_comp_descr), player_vehicle) in &self.vehicles {
            let player = self.player_infos.get_mut(&avatar_id).unwrap();

            // patches before anonymizer doesn't have the `realName` field.
            let path = format!("/0/players/{}/realName", player_vehicle.account_dbid);
            if let Some(name) = json.pointer(&path) {
                player.name = name.as_str().unwrap().to_string();
            }

            player.account_dbid = Some(player_vehicle.account_dbid);
        }

        self
    }

    fn parse_personal_vehicles(mut self, json: &[serde_json::Value]) -> Self {
        if json.len() <= 1 {
            return self;
        }

        if let Some(vehicle_list) = json[1].pointer("/0/personal") {
            let vehicle_list = vehicle_list.as_object().unwrap();
            self.personal_vehicle = vehicle_list
                .into_iter()
                .filter(|(type_comp_descr, _)| **type_comp_descr != "avatar")
                .map(|(type_comp_descr, personal_vehicle)| {
                    let key = type_comp_descr.parse().unwrap();
                    let mut value: PersonalVehicle =
                        serde_json::from_value(personal_vehicle.clone()).unwrap();
                    value.arena_unique_id = self.arena_unique_id.clone().unwrap();

                    (key, value)
                })
                .collect();
        } else {
            // Error
        }

        self
    }
}

/// A player may have multiple vehicles in a battle. We parse them and return an iterator of tuple containing
/// `((avatarID, typeCompDescr), Vehicle)`.
///
/// - `avatarID: i32` identifes the player in that battle
/// - `typeCompDescr: i32` identifies the vehicle
/// - `Vehicle` contains the info about the vehicle
fn read_player_vehicles(
    avatar_id: i32, arena_id: String, json: serde_json::Value,
) -> impl Iterator<Item = ((i32, i32), Vehicle)> {
    let vehicle_list = json.as_array().unwrap().clone();

    vehicle_list.into_iter().map(move |value| {
        let mut vehicle: Vehicle = serde_json::from_value(value.clone()).unwrap();
        vehicle.avatar_session_id = avatar_id;
        vehicle.arena_unique_id = arena_id.clone();

        ((avatar_id, vehicle.type_comp_descr), vehicle)
    })
}

async fn insert_replay(replay: Replay, db_conn: &Pool<Sqlite>) {
    let mut tx = db_conn.begin().await.unwrap();

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(ReplayCommon::insert_query_start());
    query_builder.push_values([replay.common], ReplayCommon::push_bindings);
    query_builder.build().execute(&mut tx).await.unwrap();

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(PlayerInfo::insert_query_start());
    query_builder.push_values(replay.player_infos.into_values(), PlayerInfo::push_bindings);
    query_builder.build().execute(&mut tx).await.unwrap();

    tx.commit().await.unwrap();
}
