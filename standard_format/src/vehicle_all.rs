use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use macros::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct VehicleAll {
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

    // Steel Hunter?
    achived_level: WotValue,

    // Frontlines

    // Random Battles

    // Maps Training (Recon Mode?)

    // Ranked Battles
}


