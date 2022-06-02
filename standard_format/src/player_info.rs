use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ArenaFieldsGetter;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfoExtra;

impl ArenaFieldsGetter for PlayerInfo {
    type EnumType = PlayerInfoExtra;

    fn get_arena_fields(&self) -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }

    // Always ok because there is no arena fields to validate here
    fn validate_arena_fields(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
