use crate::packet_parser::prelude::*;

/// Ex: Your teammate hits an enemy. Another ex: You get shot by artillery.
#[derive(Serialize, Deserialize, Debug, Clone, Version)]
pub struct ShowDamageFromShot {
    pub entity_id:     u32,
    pub points:        Vec<u64>,
    pub effects_index: u8,
    pub damage_factor: u8,

    #[version([1, 13, 0, 0])]
    pub last_material_shield: Option<bool>,
}

/// A vehicle fires a shot
#[derive(Serialize, Deserialize, Debug, Clone, Version)]
pub struct ShowShooting {
    /// This value seems to be `1` most times. Perhaps, with a double-barrel tank it could different.
    pub burst_count: u8,

    #[version([1, 6, 1, 0])]
    pub gun_index: Option<u8>,
}

/// Ex: A vehicle takes a shot and loses hp
#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
pub struct OnHealthChanged {
    pub new_health: i16,

    #[version([1, 11, 1, 0])]
    pub old_health: Option<i16>,

    #[event_debug(as_player)]
    pub attacker_id: i32,

    pub attack_reason: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
pub struct ShowDamageFromExplosion {
    #[event_debug(as_player)]
    pub attacker_id: i32,

    pub center:        Vector3,
    pub effects_idx:   u8,
    pub damage_factor: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Version, EventPrinter)]
pub struct ShowTracer {
    #[event_debug(as_player)]
    pub shooter_id: i32,

    pub shot_id:         i32,
    pub is_ricochet:     bool,
    pub effects_idx:     u8,
    pub ref_start_point: Vector3,
    pub velocity:        Vector3,
    pub gravity:         f32,
    pub max_shot_dist:   f32,
    pub gun_idx:         u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
pub struct OnStaticCollision {
    pub energy:     f32,
    pub point:      Vector3,
    pub normal:     Vector3,
    pub misc_flags: u8,

    #[version([0, 9, 16, 0])]
    pub damage: Option<f32>,

    #[version([0, 9, 17, 0])]
    pub destr_effects_idx: Option<i8>,

    #[version([0, 9, 23, 0])]
    pub destr_max_health: Option<u16>,
}
