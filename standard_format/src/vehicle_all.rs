use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ArenaFieldsGetter;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of `VehicleAll` that always occur in the battle results
pub struct VehicleAll {
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
    type_comp_descr:             i32,

    #[serde(rename = "accountDBID")]
    account_dbid: u64,

    index:        i32,
    death_reason: i32,
    team:         i32,
    kills:        i32,
    spotted:      i32,
    damaged:      i32,
    damaged_hp:   i32,
    stunned:      i32,

    /// Holds fields that only occur in certain gamemodes.
    #[serde(flatten)]
    pub arena_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
/// This enum is only used so that serde can work its magic parsing
/// `VehicleAll` from different gamemodes
pub enum VehicleAllExtra {
    SteelHunter(SteelHunter),
    ArtOfStrategy(ArtOfStrategy),
}

impl ArenaFieldsGetter for VehicleAll {
    type EnumVariant = VehicleAllExtra;

    fn get_arena_fields(&self) -> HashMap<String, serde_json::Value> {
        self.arena_fields.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `VehicleAll` that only occurs in Steel Hunter Gamemode
pub struct SteelHunter {
    achived_level: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of type `VehicleAll` that only occurs in Art of Strategy Gamemode
pub struct ArtOfStrategy {
    supply_damage_dealt:         i32,
    damage_received_from_supply: i32,
    rts_event_points:            i32,
    rts_leader_points:           i32,
    spotted_supplies:            i32,
    damaged_supplies:            serde_json::Value,
    killed_supplies:             i32,
    damaged_tanks:               serde_json::Value,
}
