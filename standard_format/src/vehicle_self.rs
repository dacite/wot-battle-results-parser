use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of `VehicleSelf` that always occur in the battle results
pub struct VehicleSelf {
    health:     i32,
    max_health: i32,
    credits:    i32,
    xp:         i32,

    #[serde(rename = "xp/attack")]
    xp_attack: i32,

    #[serde(rename = "xp/assist")]
    xp_assist: i32,

    #[serde(rename = "xp/other")]
    xp_other: i32,

    xp_penalty:          i32,
    achievement_credits: i32,

    #[serde(rename = "achievementXP")]
    achievement_xp: i32,

    #[serde(rename = "achievementFreeXP")]
    achievement_free_xp: i32,

    shots: i32,
    direct_hits: i32,
    direct_enemy_hits: i32,
    direct_team_hits: i32,
    explosion_hits: i32,
    piercings: i32,
    piercing_enemy_hits: i32,
    damage_dealt: i32,
    sniper_damage_dealt: i32,
    equipment_damage_dealt: i32,
    damage_assisted_radio: i32,
    damage_assisted_track: i32,
    damage_assisted_stun: i32,
    damage_assisted_smoke: i32,
    damage_assisted_inspire: i32,
    stun_num: i32,
    stun_duration: f32,
    damage_received: i32,
    damage_received_from_invisibles: i32,
    damage_blocked_by_armor: i32,
    direct_hits_received: i32,
    no_damage_direct_hits_received: i32,
    explosion_hits_received: i32,
    piercings_received: i32,
    tdamage_dealt: i32,
    tdestroyed_modules: serde_json::Value,
    tkills: i32,
    is_team_killer: bool,
    capture_points: i32,
    capturing_base: serde_json::Value,
    dropped_capture_points: i32,
    mileage: i32,
    life_time: i32,

    #[serde(rename = "killerID")]
    killer_id: i32,

    achievements:                serde_json::Value,
    in_battle_achievements:      serde_json::Value,
    potential_damage_received:   i32,
    rollouts_count:              i32,
    death_count:                 i32,
    flag_actions:                serde_json::Value,
    solo_flag_capture:           i32,
    flag_capture:                i32,
    win_points:                  i32,
    resource_absorbed:           i32,
    stop_respawn:                bool,
    num_recovered:               i32,
    vehicle_num_captured:        i32,
    destructibles_num_destroyed: i32,
    destructibles_damage_dealt:  i32,
    destructibles_hits:          i32,
    destructible_deaths:         serde_json::Value,
    num_defended:                i32,

    #[serde(rename = "accountDBID")]
    account_dbid: u64,

    type_comp_descr: i32,
    index:           i32,
    death_reason:    i32,
    team:            i32,
    kills:           i32,
    spotted:         i32,
    damaged:         i32,
    damaged_hp:      i32,
    stunned:         i32,

    repair: i32,

    #[serde(rename = "freeXP")]
    free_xp: i32,

    // #[custom_parser = "parse_bytes"]
    details: serde_json::Value,

    credits_penalty: i32,
    credits_contribution_in: i32,
    credits_contribution_out: i32,
    original_credits_to_draw: i32,
    credits_to_draw: i32,
    damage_before_team_was_damaged: i32,
    kills_before_team_was_damaged: i32,
    percent_from_total_team_damage: f32,
    win_alone_against_vehicle_count: i32,
    percent_from_second_best_damage: f32,
    killed_and_damaged_by_all_squadmates: i32,
    damaged_while_moving: i32,
    damaged_while_enemy_moving: i32,
    committed_suicide: bool,
    crystal: i32,
    event_coin: i32,
    bpcoin: i32,
    piggy_bank: i32,
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
    original_credits: i32,

    // #[custom_parser = "parse_bytes"]
    credits_replay: serde_json::Value,

    #[serde(rename = "originalXP")]
    original_xp: i32,

    // #[custom_parser = "parse_bytes"]
    xp_replay: serde_json::Value,

    #[serde(rename = "originalFreeXP")]
    original_free_xp: i32,

    // #[custom_parser = "parse_bytes"]
    #[serde(rename = "freeXPReplay")]
    free_xp_replay: serde_json::Value,

    #[serde(rename = "originalTMenXP")]
    original_t_men_xp: i32,

    // #[custom_parser = "parse_bytes"]
    #[serde(rename = "tmenXPReplay")]
    tmen_xp_replay: serde_json::Value,

    #[serde(rename = "tmenXP")]
    tmen_xp: i32,

    original_gold: i32,

    // #[custom_parser = "parse_bytes"]
    gold_replay: serde_json::Value,

    gold:             i32,
    original_crystal: i32,

    // #[custom_parser = "parse_bytes"]
    crystal_replay: serde_json::Value,

    original_event_coin: i32,
    original_bpcoin:     i32,

    // #[custom_parser = "parse_bytes"]
    event_coin_replay: serde_json::Value,

    // #[custom_parser = "parse_bytes"]
    bpcoin_replay: serde_json::Value,

    #[serde(rename = "factualXP")]
    factual_xp:      i32,
    #[serde(rename = "factualFreeXP")]
    factual_free_xp: i32,

    factual_credits:  i32,
    subtotal_credits: i32,

    #[serde(rename = "subtotalXP")]
    subtotal_xp: i32,

    #[serde(rename = "subtotalFreeXP")]
    subtotal_free_xp: i32,

    #[serde(rename = "subtotalTMenXP")]
    subtotal_t_men_xp: i32,

    subtotal_gold:       i32,
    subtotal_crystal:    i32,
    subtotal_event_coin: i32,
    subtotal_bpcoin:     i32,
    event_credits_list:  serde_json::Value,

    #[serde(rename = "eventXPList")]
    event_xp_list: serde_json::Value,

    #[serde(rename = "eventFreeXPList")]
    event_free_xp_list: serde_json::Value,

    #[serde(rename = "eventTMenXPList")]
    event_t_men_xp_list: serde_json::Value,

    event_gold_list:               serde_json::Value,
    event_crystal_list:            serde_json::Value,
    event_event_coin_list:         serde_json::Value,
    event_bpcoin_list:             serde_json::Value,
    event_credits_factor1000_list: serde_json::Value,

    #[serde(default)]
    event_credits_factor100_list: serde_json::Value,

    #[serde(rename = "eventXPFactor100List")]
    event_xp_factor100_list: serde_json::Value,

    #[serde(rename = "eventFreeXPFactor100List")]
    event_free_xp_factor100_list: serde_json::Value,

    #[serde(rename = "eventTMenXPFactor100List")]
    event_t_men_xp_factor100_list: serde_json::Value,

    event_gold_factor100_list: serde_json::Value,

    #[serde(rename = "originalXPPenalty")]
    original_xp_penalty: i32,

    original_credits_penalty:          i32,
    original_credits_contribution_in:  i32,
    original_credits_contribution_out: i32,

    #[serde(rename = "premiumVehicleXP")]
    premium_vehicle_xp: i32,

    #[serde(rename = "premiumVehicleXPFactor100")]
    premium_vehicle_xp_factor100: i32,

    #[serde(rename = "squadXP")]
    squad_xp: i32,

    #[serde(rename = "squadXPFactor100")]
    squad_xp_factor100: i32,

    #[serde(rename = "referral20XP")]
    referral20_xp: i32,

    #[serde(rename = "referral20XPFactor100")]
    referral20_xp_factor100: i32,

    referral20_credits:           i32,
    referral20_credits_factor100: i32,

    #[serde(rename = "premiumXPFactor100")]
    premium_xp_factor100: i32,

    #[serde(rename = "premiumPlusXPFactor100")]
    premium_plus_xp_factor100: i32,

    #[serde(rename = "appliedPremiumXPFactor100")]
    applied_premium_xp_factor100: i32,

    #[serde(rename = "premiumTmenXPFactor100")]
    premium_tmen_xp_factor100: i32,

    #[serde(rename = "premiumPlusTmenXPFactor100")]
    premium_plus_tmen_xp_factor100: i32,

    #[serde(rename = "appliedPremiumTmenXPFactor100")]
    applied_premium_tmen_xp_factor100: i32,

    premium_credits_factor100:         i32,
    premium_plus_credits_factor100:    i32,
    applied_premium_credits_factor100: i32,
    prem_squad_credits_factor100:      i32,
    original_prem_squad_credits:       i32,
    prem_squad_credits:                i32,

    #[serde(rename = "dailyXPFactor10")]
    daily_xp_factor10: i32,

    #[serde(rename = "additionalXPFactor10")]
    additional_xp_factor10: i32,

    #[serde(rename = "igrXPFactor10")]
    igr_xp_factor10: i32,

    aogas_factor10: i32,

    #[serde(rename = "refSystemXPFactor10")]
    ref_system_xp_factor10: i32,

    fairplay_factor10: i32,
    order_credits:     i32,

    #[serde(rename = "orderXP")]
    order_xp: i32,

    #[serde(rename = "orderFreeXP")]
    order_free_xp: i32,

    #[serde(rename = "orderTMenXP")]
    order_t_men_xp: i32,

    order_credits_factor100: i32,

    #[serde(rename = "orderXPFactor100")]
    order_xp_factor100: i32,

    #[serde(rename = "orderFreeXPFactor100")]
    order_free_xp_factor100: i32,

    #[serde(rename = "orderTMenXPFactor100")]
    order_t_men_xp_factor100: i32,

    booster_credits: i32,

    #[serde(rename = "boosterXP")]
    booster_xp: i32,

    #[serde(rename = "boosterFreeXP")]
    booster_free_xp: i32,

    #[serde(rename = "boosterTMenXP")]
    booster_t_men_xp: i32,

    booster_credits_factor100: i32,

    #[serde(rename = "boosterXPFactor100")]
    booster_xp_factor100: i32,

    #[serde(rename = "boosterFreeXPFactor100")]
    booster_free_xp_factor100: i32,

    #[serde(rename = "boosterTMenXPFactor100")]
    booster_t_men_xp_factor100: i32,

    #[serde(rename = "playerRankXP")]
    player_rank_xp: i32,

    #[serde(rename = "playerRankXPFactor100")]
    player_rank_xp_factor100: serde_json::Value,

    is_premium: serde_json::Value,
    prem_mask:  serde_json::Value,

    #[serde(rename = "xpByTmen")]
    xp_by_tmen: serde_json::Value,

    auto_repair_cost:         serde_json::Value,
    auto_load_cost:           serde_json::Value,
    auto_equip_cost:          serde_json::Value,
    auto_equip_boosters_cost: serde_json::Value,
    prev_mark_of_mastery:     i32,
    mark_of_mastery:          serde_json::Value,
    dossier_pop_ups:          serde_json::Value,
    dossier_log_records:      serde_json::Value,
    veh_type_lock_time:       i32,

    #[serde(rename = "serviceProviderID")]
    service_provider_id: i32,
    marks_on_gun:        i32,
    moving_avg_damage:   i32,
    damage_rating:       i32,
    battle_num:          i32,

    // #[custom_parser = "parse_quests_progress"]
    quests_progress: serde_json::Value,

    // #[custom_parser = "parse_c11n_progress"]
    c11n_progress: serde_json::Value,

    original_credits_to_draw_squad:          i32,
    original_credits_penalty_squad:          i32,
    original_credits_contribution_in_squad:  i32,
    original_credits_contribution_out_squad: serde_json::Value,

    #[serde(default)]
    setups_indexes: serde_json::Value,

    #[serde(flatten)]
    extra_fields: VehicleSelfExtra,
}

impl VehicleSelf {
    pub fn parse_bytes(&mut self, _item: serde_pickle::Value) -> serde_json::Value {
        serde_json::Value::Null
    }

    pub fn parse_c11n_progress(&mut self, _item: serde_pickle::Value) -> serde_json::Value {
        serde_json::Value::Null
    }

    //TODO: This custom parser was not needed before. find out why its needed now
    pub fn parse_quests_progress(&mut self, _item: serde_pickle::Value) -> serde_json::Value {
        if let Ok(value) = serde_pickle::from_value(_item) {
            value
        } else {
            serde_json::Value::Null
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `VehicleSelf` that only occurs in Random Battles.
/// We have this empty struct so that serde can match a variant of
/// `VehicleSelf` enum when there are no extra fields. This is the case for
/// all gamemodes except steel hunter.
struct Random {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `VehicleSelf` that only occurs in Steel Hunter Gamemode
struct SteelHunter {
    br_pos_in_battle: i32,
    achived_level:    i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// This enum is only used so that serde can work its magic parsing
/// `VehicleSelf` from different gamemodes
enum VehicleSelfExtra {
    SteelHunter(SteelHunter),
    Random(Random),
}
