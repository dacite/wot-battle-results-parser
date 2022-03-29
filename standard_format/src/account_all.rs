use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use macros::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountAll {
    // Common
    avatar_damage_dealt: i32,
    avatar_kills: i32,
    avatar_damaged: i32,
    total_damaged: i32,
    fairplay_violations: serde_json::Value,
    badges: serde_json::Value,
    player_rank: i32,

    // Steel Hunter?
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,

    // Frontlines
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
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,

    // Random Battles
    bp_chapter: i32,
    base_points_diff: i32,
    bp_non_chapter_points_diff: i32,
    sum_points: i32,
    has_battle_pass: bool,

    // Maps Training (Recon Mode?)

    // Ranked Battles
    prev_acc_rank: serde_json::Value,
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,
}


