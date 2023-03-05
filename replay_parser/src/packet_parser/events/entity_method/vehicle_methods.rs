use super::MethodParser;
use crate::{
    entity_defs::{EntityType, VEHICLE_METHODS},
    packet_parser::prelude::*,
};

#[derive(Debug, Clone, Serialize, macros::EnumVariantDeserialize)]
#[non_exhaustive]
/// Different methods that can be called on the Vehicle entity. If the enum is a unit variant, it means
/// that we don't yet support parsing the data of the packet
pub enum VehicleMethods {
    OnDropCapturePointsOnDamagedByEnemy,
    OnExtraHitted,
    OnFestivalRaceRepair,

    #[variant_de(delegate)]
    OnHealthChanged(OnHealthChanged),

    OnPushed,

    #[variant_de(delegate)]
    OnStaticCollision(OnStaticCollision),

    OnVehiclePickup,
    ReceiveHorn,
    ShowAmmoBayEffect,

    #[variant_de(delegate)]
    ShowDamageFromExplosion(ShowDamageFromExplosion),

    #[variant_de(delegate)]
    ShowDamageFromShot(ShowDamageFromShot),

    ShowNotDamage,
    ShowRammingEffect,

    #[variant_de(delegate)]
    ShowShooting(ShowShooting),

    UpdateLaserSight,
}

impl MethodParser for VehicleMethods {
    fn parse(input: &[u8], method_id: usize, context: &Context) -> Result<super::EntityMethod, PacketError>
    where
        Self: Sized,
    {
        let version = context.get_version();
        let version_str = crate::utils::version_as_string(version);

        let not_found_err = |err_msg| PacketError::NotFoundError {
            err: format!("{err_msg} version={version_str} method_id={method_id}"),
        };

        let methods = VEHICLE_METHODS
            .get(&version_str)
            .ok_or_else(|| not_found_err("version not found"))?;

        let discrim = methods
            .get(method_id)
            .ok_or_else(|| not_found_err("method not found"))?;


        let method = VariantDeserializer::deserialize_variant(discrim, input, &context).map_err(|err| {
            PacketError::EntityMethodError {
                entity_type: VehicleMethods::entity_type(),
                method:      discrim,
                root_cause:  err.to_string(),
            }
        })?;

        Ok(super::EntityMethod::Vehicle(method))
    }
}

impl VehicleMethods {
    pub fn entity_type() -> EntityType {
        EntityType::Vehicle
    }
}

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
