use serde::{Serialize, Deserialize};

use macros::FieldAccess;
use crate::FieldAccess;
use crate::WotValue;
#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountSelf {
    // Common
    avatar_damage_dealt: i32,
    account_dbid: u64,
    avatar_kills: i32,
    avatar_damaged: i32,
    total_damaged: i32,
    fairplay_violations: serde_json::Value,
    badges: serde_json::Value,
    rank_change: i32,
    avatar_ammo: serde_json::Value,
    avatar_damage_event_list: serde_json::Value,
    team: i32,
    clan_dbid: u64,
    fort_clan_dbi_ds: serde_json::Value,
    winner_if_draw: i32,
    is_premature_leave: bool,
    watched_battle_to_the_end: bool,
    vse_battle_results: serde_json::Value,
    squad_bonus_info: serde_json::Value,
    progressive_reward: serde_json::Value,
    eligible_for_crystal_rewards: bool,
    active_rents: serde_json::Value,
    recruits_i_ds: serde_json::Value,
    recruiter_id: u64,
    referral_bonus_vehicles: serde_json::Value,
    fare_team_xp_position: i32,
    quests_progress: serde_json::Value,

    #[custom_parser = "parse_pm2_progress"]
    pm2_progress: serde_json::Value,

    dog_tags: serde_json::Value,
    event_credits: i32,
    event_xp: i32,
    event_free_xp: i32,
    event_t_men_xp: i32,
    event_gold: i32,
    event_crystal: i32,
    event_event_coin: i32,
    event_bpcoin: i32,
    credits: i32,
    xp: i32,
    free_xp: i32,
    crystal: i32,
    player_rank: i32,
    gold_bank_gain: i32,
    replay_url: String,

    // Steel Hunter?
    max_achieved_br_title: serde_json::Value,
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
    scenario_progress: serde_json::Value,
    mt_progress_improved: i32,
    mt_map_complete: i32,

    // Ranked Battles
    updated_rank_change: i32,
    acc_rank: serde_json::Value,
    veh_rank: serde_json::Value,
    prev_max_rank: serde_json::Value,
    prev_veh_rank: serde_json::Value,
    shields: serde_json::Value,
    prev_shields: serde_json::Value,
    ranked_season: serde_json::Value,
    ranked_season_num: i32,
    bonus_battle_used: i32,
    efficiency_bonus_battles: i32,
    steps_bonus_battles: i32,
    prev_acc_rank: serde_json::Value,
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,
}

impl AccountSelf {
    pub fn parse_pm2_progress(&mut self, _item: serde_pickle::Value) -> serde_json::Value {
        serde_json::Value::Null
    }
}
