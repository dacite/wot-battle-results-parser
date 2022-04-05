use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSelf {
    avatar_damage_dealt: i32,

    #[serde(rename = "accountDBID")]
    account_dbid: i64,

    avatar_kills:             i32,
    avatar_damaged:           i32,
    total_damaged:            i32,
    fairplay_violations:      serde_json::Value,
    badges:                   serde_json::Value,
    rank_change:              i32,
    avatar_ammo:              serde_json::Value,
    avatar_damage_event_list: serde_json::Value,
    team:                     i32,

    #[serde(rename = "clanDBID")]
    clan_dbid: i64,

    #[serde(rename = "fortClanDBIDs")]
    fort_clan_dbi_ds: serde_json::Value,

    winner_if_draw:               i32,
    is_premature_leave:           bool,
    watched_battle_to_the_end:    bool,
    vse_battle_results:           serde_json::Value,
    squad_bonus_info:             serde_json::Value,
    progressive_reward:           serde_json::Value,
    eligible_for_crystal_rewards: bool,
    active_rents:                 serde_json::Value,

    #[serde(rename = "recruitsIDs")]
    recruits_ids: serde_json::Value,

    #[serde(rename = "recruiterID")]
    recruiter_id: i64,

    #[serde(default)]
    referral_bonus_vehicles: serde_json::Value,

    #[serde(rename = "fareTeamXPPosition")]
    fare_team_xp_position: i32,

    quests_progress: serde_json::Value,

    // #[custom_parser = "parse_pm2_progress"]
    #[serde(rename = "PM2Progress")]
    pm2_progress: serde_json::Value,

    dog_tags:      serde_json::Value,
    event_credits: i32,

    #[serde(rename = "eventXP")]
    event_xp: i32,

    #[serde(rename = "eventFreeXP")]
    event_free_xp: i32,

    #[serde(rename = "eventTMenXP")]
    event_t_men_xp: i32,

    event_gold:       i32,
    event_crystal:    i32,
    event_event_coin: i32,
    event_bpcoin:     i32,
    credits:          i32,
    xp:               i32,

    #[serde(rename = "freeXP")]
    free_xp: i32,

    crystal:        i32,
    player_rank:    i32,
    gold_bank_gain: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountSelf` that only occurs in Random Battles
pub struct Random {
    bp_chapter:                 i32,
    base_points_diff:           i32,
    bp_non_chapter_points_diff: i32,
    sum_points:                 i32,
    has_battle_pass:            bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountSelf` that only occurs in Ranked Battles
pub struct Ranked {
    updated_rank_change:        i32,
    acc_rank:                   serde_json::Value,
    veh_rank:                   serde_json::Value,
    prev_max_rank:              serde_json::Value,
    prev_veh_rank:              serde_json::Value,
    shields:                    serde_json::Value,
    prev_shields:               serde_json::Value,
    ranked_season:              serde_json::Value,
    ranked_season_num:          i32,
    bonus_battle_used:          i32,
    efficiency_bonus_battles:   i32,
    steps_bonus_battles:        i32,
    prev_acc_rank:              serde_json::Value,
    bp_chapter:                 i32,
    base_points_diff:           i32,
    bp_non_chapter_points_diff: i32,
    sum_points:                 i32,
    has_battle_pass:            bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountSelf` that only occurs in Steel Hunter Gamemode
struct SteelHunter {
    max_achieved_br_title:      serde_json::Value,
    bp_chapter:                 i32,
    base_points_diff:           i32,
    bp_non_chapter_points_diff: i32,
    sum_points:                 i32,
    has_battle_pass:            bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountSelf` that only occurs in Frontline battles
struct Frontline {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `AccountSelf` that only occurs in Maps Training battles
struct MapsTraining {
    scenario_progress:    serde_json::Value,
    mt_progress_improved: i32,
    mt_map_complete:      i32,
}

impl AccountSelf {
    pub fn parse_pm2_progress(&mut self, _item: serde_pickle::Value) -> serde_json::Value {
        serde_json::Value::Null
    }

    pub fn get_account_dbid(&self) -> i64 {
        self.account_dbid
    }
}
