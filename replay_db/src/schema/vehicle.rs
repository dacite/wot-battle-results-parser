use macros::SqlInsert;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, SqlInsert)]
#[serde(rename_all = "camelCase")]
/// Information on how a vehicle performed during the battle. Same player may have multiple vehicles (for ex.
/// Frontlines)
pub struct Vehicle {
    #[serde(rename = "arenaUniqueID", default)]
    pub arena_unique_id: String, // Primary key

    #[serde(rename = "avatarSessionID", default)]
    pub avatar_session_id: i32, // Primary key

    pub type_comp_descr: i32, // Primary key

    pub is_team_killer: bool,
    #[serde(default)]
    pub name: String,
    pub max_health: i32,
    #[serde(default)]
    pub fake_name: String,
    pub team: i32,
    #[serde(default)]
    pub vehicle_type: String,
    #[serde(default)]
    pub total_damaged: i32,
    #[serde(rename = "accountDBID")]
    pub account_dbid: i64,
    pub achievement_credits: i32,
    #[serde(rename = "achievementFreeXP")]
    pub achievement_free_xp: i32,
    #[serde(rename = "achievementXP")]
    pub achievement_xp: i32,
    pub capture_points: i32,
    pub capturing_base: Option<i32>,
    pub credits: i32,
    pub damage_assisted_inspire: i32,
    pub damage_assisted_radio: i32,
    pub damage_assisted_smoke: i32,
    pub damage_assisted_stun: i32,
    pub damage_assisted_track: i32,
    pub damage_blocked_by_armor: i32,
    pub damage_dealt: i32,
    pub damage_received: i32,
    pub damage_received_from_invisibles: i32,
    pub damaged: i32,
    pub damaged_hp: i32,
    pub death_count: i32,
    pub death_reason: i32,
    pub destructibles_damage_dealt: i32,
    pub destructibles_hits: i32,
    pub destructibles_num_destroyed: i32,
    pub direct_enemy_hits: i32,
    pub direct_hits: i32,
    pub direct_hits_received: i32,
    pub direct_team_hits: i32,
    pub dropped_capture_points: i32,
    pub equipment_damage_dealt: i32,
    pub explosion_hits: i32,
    pub explosion_hits_received: i32,
    pub flag_capture: i32,
    pub health: i32,
    #[serde(rename = "killerID")]
    pub killer_id: i32,
    pub kills: i32,
    pub life_time: i32,
    pub mileage: i32,
    pub no_damage_direct_hits_received: i32,
    pub num_defended: i32,
    pub num_recovered: i32,
    pub piercing_enemy_hits: i32,
    pub piercings: i32,
    pub piercings_received: i32,
    pub potential_damage_received: i32,
    pub resource_absorbed: i32,
    pub rollouts_count: i32,
    pub shots: i32,
    pub sniper_damage_dealt: i32,
    pub solo_flag_capture: i32,
    pub spotted: i32,
    pub stop_respawn: bool,
    pub stun_duration: f32,
    pub stun_num: i32,
    pub stunned: i32,
    pub tdamage_dealt: i32,
    pub tdestroyed_modules: i32,
    pub tkills: i32,

    pub vehicle_num_captured: i32,
    pub win_points:           i32,
    pub xp:                   i32,
    #[serde(rename = "xp/assist")]
    pub xp_assist:            i32,
    #[serde(rename = "xp/attack")]
    pub xp_attack:            i32,
    #[serde(rename = "xp/other")]
    pub xp_other:             i32,
    pub xp_penalty:           i32,
    pub flag_actions:         sqlx::types::Json<Vec<i32>>,
}
