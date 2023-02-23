use super::{EntityProperty, PropertyParser};
use crate::{
    entity_defs::{EntityType, VEHICLE_PROPS},
    packet_parser::prelude::*,
};

impl PropertyParser for VehicleProperties {
    fn parse(
        input: &[u8], property_id: usize, context: &Context,
    ) -> Result<super::EntityProperty, PacketError>
    where
        Self: Sized,
    {
        let version = context.get_version();
        let version_str = crate::utils::version_as_string(version);

        let not_found_err = |err_msg| {
            PacketError::NotFoundError(format!(
                "{err_msg} version={version_str} property_id={property_id}"
            ))
        };

        let props = VEHICLE_PROPS
            .get(&version_str)
            .ok_or_else(|| not_found_err("version not found"))?;

        let discrim = props
            .get(property_id)
            .ok_or_else(|| not_found_err("property not found"))?;

        let property = match *discrim {
            "Debuff" => parse_debuff_values(input, context.get_version()),
            _ => VariantDeserializer::deserialize_variant(discrim, input, &context),
        }
        .map_err(|err| PacketError::entity_prop_err(EntityType::Vehicle, discrim, err.to_string()))?;

        Ok(EntityProperty::Vehicle(property))
    }
}


#[derive(Clone, Debug, strum::IntoStaticStr, Serialize, Deserialize, macros::EnumVariantDeserialize)]
pub enum VehicleProperties {
    ArenaBonusType(u8),
    ArenaTypeID(i32),
    ArenaUniqueID(u64),
    AvatarID(i32),
    BotKind(u8),
    Buffs(Vec<u16>),
    BurnoutLevel(u8),
    CrewCompactDescrs(Vec<String>),
    CustomRoleSlotTypeId(u8),
    DamageStickers(Vec<u64>),

    #[variant_de(manual)]
    Debuff(DebuffValues),

    DisabledSwitches(Vec<i32>),
    DotEffect(DotEffect),
    EngineMode((u8, u8)),
    Enhancements, //<Python>
    Gear(i8),
    GunAnglesPacked(u16),
    GunAnglesPackedArr(Vec<u16>), // This field is manually added by us to support weird behavior in 0.9.20

    #[variant_de(delegate)]
    HealOverTime(HealOverTime),

    #[variant_de(delegate)]
    Healing(Healing),

    #[variant_de(delegate)]
    HealingEffect(HealingEffect),

    Health(i16),

    #[variant_de(delegate)]
    Inspired(InspiredValues),

    #[variant_de(delegate)]
    InspiringEffect(InspiringEffect),

    IsBlockingCapture(u8),
    IsCapturingTeamBase(u8),
    IsCrewActive(u8),
    IsDisappeared(u8), /* This field is enigmatic. It only appears in 0.9.20 (and seems to be a bug in WG
                        * code?) */
    IsMyVehicle(u8),
    IsSpeedCapturing(u8),
    IsStrafing(u8),
    IsSurfaceContact(u8),
    LastCheckpointID(String),
    LastStandEnabled(u8),
    MasterVehID(u32),
    OnRespawnReloadTimeFactor(f32),

    #[variant_de(delegate)]
    OwnVehiclePosition(OwnVehiclePosition),

    PhysicsMode(u8),
    PublicInfo, //PublicInfoValues
    PublicStateModifiers(Vec<u8>),
    QuickShellChangerFactor(f32),
    RaceFinishTime(f32),
    RacePosition(i32),
    SecondGunAnglesPacked(u16),
    Setups,        // Python
    SetupsIndexes, // Python
    SiegeState(u8),
    SteeringAngle(f32),
    SteeringAngles(Vec<u8>),
    StunInfo(f64),
    TeamBasePoints(u16),
    TrackScrolling(u16),
    VehPerks, // Python
    VehPostProgression(Vec<i32>),
    WheelsScroll(Vec<u8>),
    WheelsState(u64),
}

impl VehicleProperties {
    pub fn entity_type() -> EntityType {
        EntityType::Vehicle
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct HealingEffect {
    radius:             f32,
    start_time:         f64,
    end_time:           f64,
    inactivation_delay: f32,
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct Healing {
    sender_key:              String,
    start_time:              f64,
    end_time:                f64,
    inactivation_start_time: f64,
    inactivation_end_time:   f64,
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct HealOverTime {
    sender_key:              String,
    start_time:              f64,
    end_time:                f64,
    inactivation_start_time: f64,
    inactivation_end_time:   f64,
    is_influence_zone:       u8,
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct DotEffect {
    end_time:         f64,
    period:           f32,
    group_id:         u8,
    attack_reason_id: u8,
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct OwnVehiclePosition {
    position:       Vector3,
    direction:      Vector3,
    speed:          f32,
    rotation_speed: f32,
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct InspiringEffect {
    radius:             f32,
    start_time:         f64,
    end_time:           f64,
    inactivation_delay: f32,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum DebuffValues {
    U8(u8),
    I32(i32),
}

pub fn parse_debuff_values(input: &[u8], de_version: [u16; 4]) -> Result<VehicleProperties, PacketError> {
    if de_version >= [1, 17, 0, 0] {
        Ok(VehicleProperties::Debuff(DebuffValues::I32(from_slice_prim(
            input, de_version,
        )?)))
    }
    // de_version >= [1, 10, 0, 0]
    else {
        Ok(VehicleProperties::Debuff(DebuffValues::I32(from_slice_prim(
            input, de_version,
        )?)))
    }
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub enum GunAnglesPackedValues {
//     // #[version([0, 9, 20, 0])]
//     U16Array(Vec<u16>),
//     // #[version([0, 9, 12, 0])]
//     U16(u16),
// }

// pub fn parse_gun_angles_packed_values(
//     input: &[u8], de_version: [u16; 4],
// ) -> Result<GunAnglesPackedValues, PacketError> {
//     match de_version {
//         // [0, 9, 20, 0] => Ok(GunAnglesPackedValues::U16Array(from_slice_prim(
//         //     input, de_version,
//         // )?)),
//         _ => Ok(GunAnglesPackedValues::U16(from_slice_prim(input, de_version)?)),
//     }
// }

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct InspiredValues {
    #[version([1, 10, 0, 0])]
    primary:                 Option<u8>,
    start_time:              f64,
    end_time:                f64,
    inactivation_start_time: f64,
    inactivation_end_time:   f64,

    #[version([1, 14, 0, 0])]
    inactivation_source: Option<u8>,

    #[version([1, 10, 0, 0])]
    equipment_id: Option<u16>,
}

// // #[version([0, 9, 12, 0])]
// pub struct PublicInfoValues {
//     name: String,
//     compDescr: String,

//     #[0, 9, 21, 0]
//     outfit: String,

//     #[0, 9, 14, 0]
//     index: u8,

//     team: u8,
//     prebattleID: i32,
//     marksOnGun: u8

//     #[version([1, 4, 1, 0])]
//     crewGroup: CrewGroups,

//     #[version([1, 4, 1, 0])]
//     commanderSkinID: u16,

//     #[version[1, 10, 0, 0]]
//     maxHealth: u16


// }

// pub enum CrewGroups {
//     #[version([1, 4, 1, 0])]
//     U16(u16),
//     #[version([1, 12, 1, 0])]
//     U16Array(Vec<u16>),
//     #[version([1, 19, 1, 0])]
//     U32Array(Vec<u32>),
// }
