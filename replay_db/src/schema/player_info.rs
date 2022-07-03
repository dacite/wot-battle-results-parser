use macros::SqlInsert;
use serde::Deserialize;
use serde::Serialize;

/// General information about a player available at the start of the game. This is a bit different from
/// `PlayerInfo` used in other contexts such as the datfile parser because this `PlayerInfo` contains some
/// information about the initial tank as well
#[derive(Debug, Serialize, Deserialize, SqlInsert)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    #[serde(rename = "replayID", default)]
    pub replay_id: u32, // Primary key

    #[serde(rename = "avatarSessionID", skip)]
    pub avatar_session_id: i32, // Primary key

    #[serde(rename = "accountDBID")]
    pub account_dbid: Option<i64>,

    /// After anonymizer patch, the `name` may contain the fake name.  
    pub name: String,

    // pub badges: Vec<Vec<i32>>,
    pub clan_abbrev:                  String,
    pub custom_role_slot_type_id:     Option<i32>,
    pub fake_name:                    Option<String>,
    pub forbid_in_battle_invitations: bool,
    pub igr_type:                     i32,
    #[serde(deserialize_with="utils::bool_to_int")]
    pub is_alive:                     i32,
    pub is_god_mode_active:           Option<i32>,
    #[serde(deserialize_with="utils::bool_to_int")]
    pub is_team_killer:               i32,

    /// `max_health` maybe differrent from the actual max (perhaps if the player switches to improved
    /// hardening equipment)
    pub max_health: Option<i32>,

    // pub overridden_badge: i32,
    // pub ranked: Vec<i32>,
    pub team: i32,

    /// `vehicle_type` is simply the tank player had selected when joining the battle
    pub vehicle_type: String,
    pub wtr:          Option<i32>,
}
