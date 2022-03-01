use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use field_access_derive::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountAll {
    // Common
    avatar_damage_dealt: WotValue,
    avatar_kills: WotValue,
    avatar_damaged: WotValue,
    total_damaged: WotValue,
    fairplay_violations: WotValue,
    badges: WotValue,
    player_rank: WotValue,

    // Steel Hunter?
    // base_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,

    // Frontlines
    credits_after_shell_costs: WotValue,
    uncharged_shell_costs: WotValue,
    prev_meta_level: WotValue,
    meta_level: WotValue,
    fl_xp: WotValue,
    original_fl_xp: WotValue,
    subtotal_fl_xp: WotValue,
    booster_fl_xp: WotValue,
    booster_fl_xp_factor100: WotValue,
    fl_xp_replay: WotValue,
    // base_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,

    // Random Battles
    base_points_diff: WotValue,
    sum_points: WotValue,
    has_battle_pass: WotValue,

    // Maps Training (Recon Mode?)

    // Ranked Battles
    prev_acc_rank: WotValue,
    // base_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,
}


