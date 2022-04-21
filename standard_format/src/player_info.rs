use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of `PlayerInfo` that always occur in the battle results
pub struct PlayerInfo {
    name:      String,
    real_name: String,

    #[serde(rename = "clanDBID")]
    clan_dbid: i64,

    clan_abbrev: String,

    #[serde(rename = "prebattleID")]
    prebattle_id: i32,

    team:     i32,
    igr_type: i32,
}
