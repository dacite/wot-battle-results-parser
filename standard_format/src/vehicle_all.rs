use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use macros::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct VehicleAll {
    // Common
    health: i32,
    max_health: i32,
    credits: i32,
    xp: i32,
    xp_attack: i32,
    xp_assist: i32,
    xp_other: i32,
    xp_penalty: i32,
    achievement_credits: i32,
    achievement_xp: i32,
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
    tdestroyed_modules: i32,
    tkills: i32,
    is_team_killer: bool,
    capture_points: i32,
    capturing_base: serde_json::Value,
    dropped_capture_points: i32,
    mileage: i32,
    life_time: i32,
    killer_id: i32,
    achievements: serde_json::Value,
    in_battle_achievements: serde_json::Value,
    potential_damage_received: i32,
    rollouts_count: i32,
    death_count: i32,
    flag_actions: serde_json::Value,
    solo_flag_capture: i32,
    flag_capture: i32,
    win_points: i32,
    resource_absorbed: i32,
    stop_respawn: bool,
    num_recovered: i32,
    vehicle_num_captured: i32,
    destructibles_num_destroyed: i32,
    destructibles_damage_dealt: i32,
    destructibles_hits: i32,
    num_defended: i32,
    type_comp_descr: i32,
    account_dbid: u64,
    index: i32,
    death_reason: i32,
    team: i32,
    kills: i32,
    spotted: i32,
    damaged: i32,
    damaged_hp: i32,
    stunned: i32,

    // Steel Hunter?
    achived_level: i32,

    // Frontlines

    // Random Battles

    // Maps Training (Recon Mode?)

    // Ranked Battles
}


