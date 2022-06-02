use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ArenaFieldsGetter;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of `AccountAll` that always occur in the battle results
pub struct AccountAll {
    avatar_damage_dealt: i32,
    avatar_kills:        i32,
    avatar_damaged:      i32,
    total_damaged:       i32,
    fairplay_violations: serde_json::Value,
    badges:              serde_json::Value,
    player_rank:         i32,

    /// Holds fields that only occur in certain gamemodes.
    /// Structs found below like `Random`, `Ranked` are some examples
    #[serde(flatten)]
    pub arena_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// This enum is only used so that serde can work its magic parsing `AccountAll`
/// from different gamemodes
pub enum AccountAllExtra {
    Random(Random),
    Ranked(Ranked),
    SteelHunter(SteelHunter),
    Frontline(Frontline),
    // Other(Other),
}

impl ArenaFieldsGetter for AccountAll {
    type EnumType = AccountAllExtra;

    fn get_arena_fields(&self) -> HashMap<String, serde_json::Value> {
        self.arena_fields.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountAll` that only occurs in Random Battles
pub struct Random {
    bp_chapter:                 i32,
    base_points_diff:           i32,
    bp_non_chapter_points_diff: i32,
    sum_points:                 i32,
    has_battle_pass:            bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountAll` that only occurs in Ranked Battles
pub struct Ranked {
    prev_acc_rank:              serde_json::Value,
    bp_chapter:                 i32,
    base_points_diff:           i32,
    bp_non_chapter_points_diff: i32,
    sum_points:                 i32,
    has_battle_pass:            bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountAll` that only occurs in Steel Hunter Gamemode
/// battles
pub struct SteelHunter {
    bp_chapter:                 i32,
    base_points_diff:           i32,
    bp_non_chapter_points_diff: i32,
    sum_points:                 i32,
    has_battle_pass:            bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountAll` that only occurs in Frontline battles
pub struct Frontline {
    credits_after_shell_costs: i32,
    uncharged_shell_costs: i32,
    prev_meta_level: serde_json::Value,
    meta_level: serde_json::Value,
    fl_xp: i32,
    original_fl_xp: i32,
    subtotal_fl_xp: i32,
    booster_fl_xp: i32,
    booster_fl_xp_factor100: i32,
    fl_xp_replay: String,
    bp_chapter: i32,
    base_points_diff: i32,
    bp_non_chapter_points_diff: i32,
    sum_points: i32,
    has_battle_pass: bool,
}
