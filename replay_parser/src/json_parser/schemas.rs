use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
// subset of fields available on the first json in replays
pub struct ReplayCommon {
    battle_type:             i32,
    client_version_from_xml: String,
    date_time:               String,

    #[serde(rename = "gameplayID")]
    gameplay_id: String,

    map_display_name: String,
    map_name:         String,
    player_name:      String,
    player_vehicle:   String,
    // regionCode: String,
    // serverName: String,
}
