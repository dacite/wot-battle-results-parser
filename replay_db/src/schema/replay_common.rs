use macros::SqlInsert;
use serde::Deserialize;
use serde::Serialize;

/// General information about a replay
#[derive(Default, Debug, Serialize, Deserialize, SqlInsert)]
#[serde(rename_all = "camelCase")]
pub struct ReplayCommon {
    #[serde(rename = "replayID", default)]
    pub replay_id:               u32,
    pub battle_type:             i32,
    #[serde(rename = "arenaUniqueID", default)]
    pub arena_unique_id:         String,
    pub client_version_from_exe: String,
    pub client_version_from_xml: String,
    pub date_time:               String,
    #[serde(rename = "gameplayID")]
    pub gameplay_id:             String,
    pub map_display_name:        String,
    pub map_name:                String,
    #[serde(rename = "playerID")]
    pub player_id:               i64,
    pub player_name:             String,
    pub player_vehicle:          String,
    pub region_code:             String,
    pub server_name:             String,
    #[serde(skip)]
    pub is_complete:             bool,
}


pub fn push_bindings<'args, 'qb, DB>(
    mut b: sqlx::query_builder::Separated<'qb, 'args, DB, &'static str>, item: ReplayCommon,
) where
    DB: sqlx::Database,
{
}
