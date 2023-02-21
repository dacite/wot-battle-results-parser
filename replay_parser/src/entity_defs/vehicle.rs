use phf::phf_map;

use super::{PropertyParser, VariantDeserializer};
use crate::packet_parser::prelude::*;

#[derive(Clone, Debug, strum::IntoStaticStr, macros::EnumVariantDeserialize, Serialize, Deserialize)]
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
    Debuff(DebuffValues),
    DisabledSwitches(Vec<i32>),
    DotEffect(DotEffect),
    EngineMode((u8, u8)),
    Enhancements, //<Python>
    Gear(i8),
    GunAnglesPacked(GunAnglesPackedValues),
    HealOverTime(HealOverTime),
    Healing(Healing),
    HealingEffect(HealingEffect),
    Health(i16),
    Inspired(InspiredValues),
    InspiringEffect(InspiringEffect),
    IsBlockingCapture(u8),
    IsCapturingTeamBase(u8),
    IsCrewActive(u8),
    IsDisappeared(u8),
    IsMyVehicle(u8),
    IsSpeedCapturing(u8),
    IsStrafing(u8),
    IsSurfaceContact(u8),
    LastCheckpointID(String),
    LastStandEnabled(u8),
    MasterVehID(u32),
    OnRespawnReloadTimeFactor(f32),
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


impl PropertyParser for VehicleProperties {
    type Discrim = &'static str;

    fn parse(input: &[u8], de_version: [u16; 4], discrim: Self::Discrim) -> Result<Self, PacketError> {
        let result = match discrim {
            "ArenaBonusType" => Self::ArenaBonusType(from_slice_prim(input, de_version)?),
            "ArenaTypeID" => Self::ArenaTypeID(from_slice_prim(input, de_version)?),
            "ArenaUniqueID" => Self::ArenaUniqueID(from_slice_prim(input, de_version)?),
            "AvatarID" => Self::AvatarID(from_slice_prim(input, de_version)?),
            "BotKind" => Self::BotKind(from_slice_prim(input, de_version)?),
            "Buffs" => Self::Buffs(from_slice_prim(input, de_version)?),
            "BurnoutLevel" => Self::BurnoutLevel(from_slice_prim(input, de_version)?),
            "CrewCompactDescrs" => Self::CrewCompactDescrs(from_slice_prim(input, de_version)?),
            "CustomRoleSlotTypeId" => Self::CustomRoleSlotTypeId(from_slice_prim(input, de_version)?),
            "DamageStickers" => Self::DamageStickers(from_slice_prim(input, de_version)?),
            "Debuff" => Self::Debuff(parse_debuff_values(input, de_version)),
            "DisabledSwitches" => Self::DisabledSwitches(from_slice_prim(input, de_version)?),
            "DotEffect" => Self::DotEffect(from_slice(input, de_version)?),
            "EngineMode" => Self::EngineMode(from_slice_prim(input, de_version)?),
            "Enhancements" => Self::Enhancements,
            "Gear" => Self::Gear(from_slice_prim(input, de_version)?),
            "GunAnglesPacked" => Self::GunAnglesPacked(parse_gun_angles_packed_values(input, de_version)?),
            "HealOverTime" => Self::HealOverTime(from_slice(input, de_version)?),
            "Healing" => Self::Healing(from_slice(input, de_version)?),
            "HealingEffect" => Self::HealingEffect(from_slice(input, de_version)?),
            "Health" => Self::Health(from_slice_prim(input, de_version)?),
            "Inspired" => Self::Inspired(from_slice(input, de_version)?),
            "InspiringEffect" => Self::InspiringEffect(from_slice(input, de_version)?),
            "IsBlockingCapture" => Self::IsBlockingCapture(from_slice_prim(input, de_version)?),
            "IsCapturingTeamBase" => Self::IsCapturingTeamBase(from_slice_prim(input, de_version)?),
            "IsCrewActive" => Self::IsCrewActive(from_slice_prim(input, de_version)?),
            "IsDisappeared" => Self::IsDisappeared(from_slice_prim(input, de_version)?),
            "IsMyVehicle" => Self::IsMyVehicle(from_slice_prim(input, de_version)?),
            "IsSpeedCapturing" => Self::IsSpeedCapturing(from_slice_prim(input, de_version)?),
            "IsStrafing" => Self::IsStrafing(from_slice_prim(input, de_version)?),
            "IsSurfaceContact" => Self::IsSurfaceContact(from_slice_prim(input, de_version)?),
            "LastCheckpointID" => Self::LastCheckpointID(from_slice_prim(input, de_version)?),
            "LastStandEnabled" => Self::LastStandEnabled(from_slice_prim(input, de_version)?),
            "MasterVehID" => Self::MasterVehID(from_slice_prim(input, de_version)?),
            "OnRespawnReloadTimeFactor" => {
                Self::OnRespawnReloadTimeFactor(from_slice_prim(input, de_version)?)
            }
            "OwnVehiclePosition" => Self::OwnVehiclePosition(from_slice(input, de_version)?),
            "PhysicsMode" => Self::PhysicsMode(from_slice_prim(input, de_version)?),
            "PublicInfo" => Self::PublicInfo,
            "PublicStateModifiers" => Self::PublicStateModifiers(from_slice_prim(input, de_version)?),
            "QuickShellChangerFactor" => Self::QuickShellChangerFactor(from_slice_prim(input, de_version)?),
            "RaceFinishTime" => Self::RaceFinishTime(from_slice_prim(input, de_version)?),
            "RacePosition" => Self::RacePosition(from_slice_prim(input, de_version)?),
            "SecondGunAnglesPacked" => Self::SecondGunAnglesPacked(from_slice_prim(input, de_version)?),
            "Setups" => Self::Setups,
            "SetupsIndexes" => Self::SetupsIndexes,
            "SiegeState" => Self::SiegeState(from_slice_prim(input, de_version)?),
            "SteeringAngle" => Self::SteeringAngle(from_slice_prim(input, de_version)?),
            "SteeringAngles" => Self::SteeringAngles(from_slice_prim(input, de_version)?),
            "StunInfo" => Self::StunInfo(from_slice_prim(input, de_version)?),
            "TeamBasePoints" => Self::TeamBasePoints(from_slice_prim(input, de_version)?),
            "TrackScrolling" => Self::TrackScrolling(from_slice_prim(input, de_version)?),
            "VehPerks" => Self::VehPerks,
            "VehPostProgression" => Self::VehPostProgression(from_slice_prim(input, de_version)?),
            "WheelsScroll" => Self::WheelsScroll(from_slice_prim(input, de_version)?),
            "WheelsState" => Self::WheelsState(from_slice_prim(input, de_version)?),
            _ => todo!(),
        };

        Ok(result)
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

pub fn parse_debuff_values(input: &[u8], de_version: [u16; 4]) -> DebuffValues {
    if de_version >= [1, 17, 0, 0] {
        DebuffValues::I32(from_slice_prim(input, de_version).unwrap())
    } else if de_version >= [1, 10, 0, 0] {
        DebuffValues::I32(from_slice_prim(input, de_version).unwrap())
    } else {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GunAnglesPackedValues {
    // #[version([0, 9, 20, 0])]
    U16Array(Vec<u16>),
    // #[version([0, 9, 12, 0])]
    U16(u16),
}

pub fn parse_gun_angles_packed_values(
    input: &[u8], de_version: [u16; 4],
) -> Result<GunAnglesPackedValues, PacketError> {
    match de_version {
        [0, 9, 20, 0] => Ok(GunAnglesPackedValues::U16Array(from_slice_prim(
            input, de_version,
        )?)),
        _ => Ok(GunAnglesPackedValues::U16(from_slice_prim(input, de_version)?)),
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, Version)]
pub struct InspiredValues {
    #[version([1, 10, 0, 0])]
    primary:               Option<u8>,
    start_time:             f64,
    end_time:               f64,
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


pub static VEHICLE_PROPS: phf::Map<&str, &'static [&'static str]> = phf_map! {
    "0_9_12_0" => &["IsStrafing", "PhysicsMode", "Gear", "IsCrewActive", "TrackScrolling", "GunAnglesPacked", "Health", "EngineMode", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_13_0" => &["IsStrafing", "PhysicsMode", "Gear", "IsCrewActive", "TrackScrolling", "GunAnglesPacked", "Health", "EngineMode", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_14_0" => &["IsStrafing", "PhysicsMode", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_15_0" => &["IsStrafing", "PhysicsMode", "IsCrewActive", "GunAnglesPacked", "SecondGunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_16_0" => &["IsStrafing", "PhysicsMode", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_17_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_18_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_19_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_20_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsDisappeared", "Health", "EngineMode", "SteeringAngle", "StunInfo", "GunAnglesPacked", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_21_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_22_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "0_9_23_0" => &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "1_0_0_0" =>  &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers"],
    "1_0_1_0" =>  &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_0_2_0" =>  &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_1_0_0" =>  &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_2_0_0" =>  &["IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "BotKind", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "SteeringAngle", "StunInfo", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_3_0_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_4_0_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_4_1_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_5_0_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_5_1_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "LastStandEnabled", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_6_0_0" =>  &["IsCapturingTeamBase", "BurnoutLevel", "IsStrafing", "IsSurfaceContact", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "TeamBasePoints", "GunAnglesPacked", "Health", "EngineMode", "RacePosition", "RaceFinishTime", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "LastCheckpointID", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_6_1_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_7_0_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_7_1_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_8_0_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired"],
    "1_9_0_0" =>  &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "Inspired", "Buffs"],
    "1_10_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime"],
    "1_10_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "Buffs"],
    "1_11_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime"],
    "1_11_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime"],
    "1_12_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime"],
    "1_12_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime"],
    "1_13_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "QuickShellChangerFactor", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime"],
    "1_14_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_14_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition", "Buffs"],
    "1_15_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_16_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "MasterVehID", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_16_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "Debuff", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_17_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "Debuff", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_17_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "Debuff", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_18_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "Debuff", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_18_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "Debuff", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_19_0_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "Debuff", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
    "1_19_1_0" => &["BurnoutLevel", "IsStrafing", "PhysicsMode", "SiegeState", "IsCrewActive", "CustomRoleSlotTypeId", "ArenaBonusType", "IsSpeedCapturing", "IsBlockingCapture", "IsMyVehicle", "GunAnglesPacked", "Health", "EngineMode", "AvatarID", "MasterVehID", "ArenaTypeID", "Debuff", "QuickShellChangerFactor", "OnRespawnReloadTimeFactor", "WheelsState", "StunInfo", "ArenaUniqueID", "SteeringAngles", "WheelsScroll", "PublicInfo", "DamageStickers", "PublicStateModifiers", "CrewCompactDescrs", "Enhancements", "Setups", "SetupsIndexes", "VehPerks", "VehPostProgression", "DisabledSwitches", "InspiringEffect", "HealingEffect", "DotEffect", "Inspired", "Healing", "HealOverTime", "OwnVehiclePosition"],
};
