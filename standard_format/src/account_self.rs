use serde::{Serialize, Deserialize};

use crate::WotValue;
use macros::FieldAccess;
use crate::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountSelf {
    // Common
    avatar_damage_dealt: WotValue,
    avatar_kills: WotValue,
    avatar_damaged: WotValue,
    total_damaged: WotValue,
    fairplay_violations: WotValue,
    badges: WotValue,
    rank_change: WotValue,
    avatar_ammo: WotValue,
    avatar_damage_event_list: WotValue,
    account_dbid: WotValue,
    team: WotValue,
    clan_dbid: WotValue,
    fort_clan_dbi_ds: WotValue,
    winner_if_draw: WotValue,
    is_premature_leave: WotValue,
    watched_battle_to_the_end: WotValue,
    vse_battle_results: WotValue,
    squad_bonus_info: WotValue,
    progressive_reward: WotValue,
    eligible_for_crystal_rewards: WotValue,
    active_rents: WotValue,
    recruits_i_ds: WotValue,
    recruiter_id: WotValue,
    referral_bonus_vehicles: WotValue,
    fare_team_xp_position: WotValue,
    quests_progress: WotValue,
    pm2_progress: WotValue,
    dog_tags: WotValue,
    event_credits: WotValue,
    event_xp: WotValue,
    event_free_xp: WotValue,
    event_t_men_xp: WotValue,
    event_gold: WotValue,
    event_crystal: WotValue,
    event_event_coin: WotValue,
    event_bpcoin: WotValue,
    credits: WotValue,
    xp: WotValue,
    free_xp: WotValue,
    crystal: WotValue,
    player_rank: WotValue,
    gold_bank_gain: WotValue,

    // Steel Hunter?
    max_achieved_br_title: WotValue,
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
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
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,

    // Random Battles
    bp_chapter: WotValue,
    base_points_diff: WotValue,
    bp_non_chapter_points_diff: WotValue,
    sum_points: WotValue,
    has_battle_pass: WotValue,

    // Maps Training (Recon Mode?)
    scenario_progress: WotValue,
    mt_progress_improved: WotValue,
    mt_map_complete: WotValue,

    // Ranked Battles
    updated_rank_change: WotValue,
    acc_rank: WotValue,
    veh_rank: WotValue,
    prev_max_rank: WotValue,
    prev_veh_rank: WotValue,
    shields: WotValue,
    prev_shields: WotValue,
    ranked_season: WotValue,
    ranked_season_num: WotValue,
    bonus_battle_used: WotValue,
    efficiency_bonus_battles: WotValue,
    steps_bonus_battles: WotValue,
    prev_acc_rank: WotValue,
    // bp_chapter: WotValue,
    // base_points_diff: WotValue,
    // bp_non_chapter_points_diff: WotValue,
    // sum_points: WotValue,
    // has_battle_pass: WotValue,
}

impl AccountSelf {

}
