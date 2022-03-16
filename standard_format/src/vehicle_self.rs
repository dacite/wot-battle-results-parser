use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use macros::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct VehicleSelf {
    // Common
    health: WotValue,
    max_health: WotValue,
    credits: WotValue,
    xp: WotValue,
    xp_attack: WotValue,
    xp_assist: WotValue,
    xp_other: WotValue,
    xp_penalty: WotValue,
    achievement_credits: WotValue,
    achievement_xp: WotValue,
    achievement_free_xp: WotValue,
    shots: WotValue,
    direct_hits: WotValue,
    direct_enemy_hits: WotValue,
    direct_team_hits: WotValue,
    explosion_hits: WotValue,
    piercings: WotValue,
    piercing_enemy_hits: WotValue,
    damage_dealt: WotValue,
    sniper_damage_dealt: WotValue,
    equipment_damage_dealt: WotValue,
    damage_assisted_radio: WotValue,
    damage_assisted_track: WotValue,
    damage_assisted_stun: WotValue,
    damage_assisted_smoke: WotValue,
    damage_assisted_inspire: WotValue,
    stun_num: WotValue,
    stun_duration: WotValue,
    damage_received: WotValue,
    damage_received_from_invisibles: WotValue,
    damage_blocked_by_armor: WotValue,
    direct_hits_received: WotValue,
    no_damage_direct_hits_received: WotValue,
    explosion_hits_received: WotValue,
    piercings_received: WotValue,
    tdamage_dealt: WotValue,
    tdestroyed_modules: WotValue,
    tkills: WotValue,
    is_team_killer: WotValue,
    capture_points: WotValue,
    capturing_base: WotValue,
    dropped_capture_points: WotValue,
    mileage: WotValue,
    life_time: WotValue,
    killer_id: WotValue,
    achievements: WotValue,
    in_battle_achievements: WotValue,
    potential_damage_received: WotValue,
    rollouts_count: WotValue,
    death_count: WotValue,
    flag_actions: WotValue,
    solo_flag_capture: WotValue,
    flag_capture: WotValue,
    win_points: WotValue,
    resource_absorbed: WotValue,
    stop_respawn: WotValue,
    num_recovered: WotValue,
    vehicle_num_captured: WotValue,
    destructibles_num_destroyed: WotValue,
    destructibles_damage_dealt: WotValue,
    destructibles_hits: WotValue,
    num_defended: WotValue,
    account_dbid: WotValue,
    type_comp_descr: WotValue,
    index: WotValue,
    death_reason: WotValue,
    team: WotValue,
    kills: WotValue,
    spotted: WotValue,
    damaged: WotValue,
    damaged_hp: WotValue,
    stunned: WotValue,
    repair: WotValue,
    free_xp: WotValue,

    #[custom_parser = "parse_bytes"]
    details: WotValue,

    credits_penalty: WotValue,
    credits_contribution_in: WotValue,
    credits_contribution_out: WotValue,
    original_credits_to_draw: WotValue,
    credits_to_draw: WotValue,
    damage_before_team_was_damaged: WotValue,
    kills_before_team_was_damaged: WotValue,
    percent_from_total_team_damage: WotValue,
    win_alone_against_vehicle_count: WotValue,
    percent_from_second_best_damage: WotValue,
    killed_and_damaged_by_all_squadmates: WotValue,
    damaged_while_moving: WotValue,
    damaged_while_enemy_moving: WotValue,
    committed_suicide: WotValue,
    crystal: WotValue,
    event_coin: WotValue,
    bpcoin: WotValue,
    piggy_bank: WotValue,
    event_credits: WotValue,
    event_xp: WotValue,
    event_free_xp: WotValue,
    event_t_men_xp: WotValue,
    event_gold: WotValue,
    event_crystal: WotValue,
    event_event_coin: WotValue,
    event_bpcoin: WotValue,
    original_credits: WotValue,

    #[custom_parser = "parse_bytes"]
    credits_replay: WotValue,

    original_xp: WotValue,

    #[custom_parser = "parse_bytes"]
    xp_replay: WotValue,

    original_free_xp: WotValue,

    #[custom_parser = "parse_bytes"]
    free_xp_replay: WotValue,

    original_t_men_xp: WotValue,

    #[custom_parser = "parse_bytes"]
    tmen_xp_replay: WotValue,

    tmen_xp: WotValue,
    original_gold: WotValue,

    #[custom_parser = "parse_bytes"]
    gold_replay: WotValue,

    gold: WotValue,
    original_crystal: WotValue,

    #[custom_parser = "parse_bytes"]
    crystal_replay: WotValue,

    original_event_coin: WotValue,
    original_bpcoin: WotValue,

    #[custom_parser = "parse_bytes"]
    event_coin_replay: WotValue,

    #[custom_parser = "parse_bytes"]
    bpcoin_replay: WotValue,
    
    factual_xp: WotValue,
    factual_free_xp: WotValue,
    factual_credits: WotValue,
    subtotal_credits: WotValue,
    subtotal_xp: WotValue,
    subtotal_free_xp: WotValue,
    subtotal_t_men_xp: WotValue,
    subtotal_gold: WotValue,
    subtotal_crystal: WotValue,
    subtotal_event_coin: WotValue,
    subtotal_bpcoin: WotValue,
    event_credits_list: WotValue,
    event_xp_list: WotValue,
    event_free_xp_list: WotValue,
    event_t_men_xp_list: WotValue,
    event_gold_list: WotValue,
    event_crystal_list: WotValue,
    event_event_coin_list: WotValue,
    event_bpcoin_list: WotValue,
    event_credits_factor1000_list: WotValue,
    event_credits_factor100_list: WotValue,
    event_xp_factor100_list: WotValue,
    event_free_xp_factor100_list: WotValue,
    event_t_men_xp_factor100_list: WotValue,
    event_gold_factor100_list: WotValue,
    original_xp_penalty: WotValue,
    original_credits_penalty: WotValue,
    original_credits_contribution_in: WotValue,
    original_credits_contribution_out: WotValue,
    premium_vehicle_xp: WotValue,
    premium_vehicle_xp_factor100: WotValue,
    squad_xp: WotValue,
    squad_xp_factor100: WotValue,
    referral20_xp: WotValue,
    referral20_xp_factor100: WotValue,
    referral20_credits: WotValue,
    referral20_credits_factor100: WotValue,
    premium_xp_factor100: WotValue,
    premium_plus_xp_factor100: WotValue,
    applied_premium_xp_factor100: WotValue,
    premium_tmen_xp_factor100: WotValue,
    premium_plus_tmen_xp_factor100: WotValue,
    applied_premium_tmen_xp_factor100: WotValue,
    premium_credits_factor100: WotValue,
    premium_plus_credits_factor100: WotValue,
    applied_premium_credits_factor100: WotValue,
    prem_squad_credits_factor100: WotValue,
    original_prem_squad_credits: WotValue,
    prem_squad_credits: WotValue,
    daily_xp_factor10: WotValue,
    additional_xp_factor10: WotValue,
    igr_xp_factor10: WotValue,
    aogas_factor10: WotValue,
    ref_system_xp_factor10: WotValue,
    fairplay_factor10: WotValue,
    order_credits: WotValue,
    order_xp: WotValue,
    order_free_xp: WotValue,
    order_t_men_xp: WotValue,
    order_credits_factor100: WotValue,
    order_xp_factor100: WotValue,
    order_free_xp_factor100: WotValue,
    order_t_men_xp_factor100: WotValue,
    booster_credits: WotValue,
    booster_xp: WotValue,
    booster_free_xp: WotValue,
    booster_t_men_xp: WotValue,
    booster_credits_factor100: WotValue,
    booster_xp_factor100: WotValue,
    booster_free_xp_factor100: WotValue,
    booster_t_men_xp_factor100: WotValue,
    player_rank_xp: WotValue,
    player_rank_xp_factor100: WotValue,
    is_premium: WotValue,
    prem_mask: WotValue,
    xp_by_tmen: WotValue,
    auto_repair_cost: WotValue,
    auto_load_cost: WotValue,
    auto_equip_cost: WotValue,
    auto_equip_boosters_cost: WotValue,
    prev_mark_of_mastery: WotValue,
    mark_of_mastery: WotValue,
    dossier_pop_ups: WotValue,
    dossier_log_records: WotValue,
    veh_type_lock_time: WotValue,
    service_provider_id: WotValue,
    marks_on_gun: WotValue,
    moving_avg_damage: WotValue,
    damage_rating: WotValue,
    battle_num: WotValue,

    #[custom_parser = "parse_quests_progress"]
    quests_progress: WotValue,

    #[custom_parser = "parse_c11n_progress"]
    c11n_progress: WotValue,

    original_credits_to_draw_squad: WotValue,
    original_credits_penalty_squad: WotValue,
    original_credits_contribution_in_squad: WotValue,
    original_credits_contribution_out_squad: WotValue,
    setups_indexes: WotValue,

    // Steel Hunter?
    br_pos_in_battle: WotValue,
    achived_level: WotValue,

    // Frontlines

    // Random Battles

    // Maps Training (Recon Mode?)

    // Ranked Battles
}

impl VehicleSelf {
    pub fn parse_bytes(&mut self, _item: serde_pickle::Value) -> WotValue {
        WotValue::None
    }

    pub fn parse_c11n_progress(&mut self, _item: serde_pickle::Value) -> WotValue {
        WotValue::None
    }

    //TODO: This custom parser was not needed before. find out why its needed now
    pub fn parse_quests_progress(&mut self, _item: serde_pickle::Value) -> WotValue {
        if let Ok(value) = serde_pickle::from_value(_item) {
            value
        } else {
            WotValue::None
        }
    }
}
