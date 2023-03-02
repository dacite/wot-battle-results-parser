use super::{EntityProperty, PropertyParser};
use crate::{
    entity_defs::{EntityType, AVATAR_PROPS},
    packet_parser::prelude::*,
};

impl PropertyParser for AvatarProperties {
    fn parse(
        input: &[u8], property_id: usize, context: &Context,
    ) -> Result<super::EntityProperty, PacketError>
    where
        Self: Sized,
    {
        let version = context.get_version();
        let version_str = crate::utils::version_as_string(version);

        let not_found_err = |err_msg| PacketError::NotFoundError {
            err: format!("{err_msg} version={version_str} property_id={property_id}"),
        };

        let props = AVATAR_PROPS
            .get(&version_str)
            .ok_or_else(|| not_found_err("version not found"))?;

        let discrim = props
            .get(property_id)
            .ok_or_else(|| not_found_err("property not found"))?;


        let property = VariantDeserializer::deserialize_variant(discrim, input, &context)?;

        Ok(EntityProperty::Avatar(property))
    }
}
#[derive(Clone, Debug, strum::IntoStaticStr, Serialize, macros::EnumVariantDeserialize, Deserialize)]
pub enum AvatarProperties {
    AiRosterVehicles(Vec<i32>),

    #[variant_de(delegate)]
    AmmoViews(AmmoViews),

    ArenaBonusType(u8),
    ArenaExtraData, //Python
    ArenaGuiType(u8),
    ArenaTypeID(i32),
    ArenaUniqueID(u64),
    ClientCtx(String),
    ClientData, //Python
    CustomizationDisplayType(u8),
    DenunciationsLeft(i16),

    #[variant_de(delegate)]
    GoodiesSnapshot(Goodie),

    IsAICommander(u8),
    IsGunLocked(u8),
    IsHistoricallyAccurate(u8),
    IsObserverBothTeams(u8),
    IsObserverFPV(u8),
    IsOwnVehicleContactingWorld(u8),
    Name(String),
    NormalisedRPMPacked(u8),
    NumOfObservers(u8),
    ObservableTeamID(u8),
    ObserverFPVControlMode(u8),
    OwnVehicleAuxPhysicsData(u64),
    OwnVehicleGear(u8),
    PlayLimits, // PlayLimit Values
    PlayerVehicleID(i32),
    QuestProgressDescriptor, //Python

    #[variant_de(delegate)]
    RemoteCamera(RemoteCamera),

    #[variant_de(delegate)]
    RemoteCameraArcade(RemoteCameraArcade),

    #[variant_de(delegate)]
    RemoteCameraArty(RemoteCameraArty),

    #[variant_de(delegate)]
    RemoteCameraSniper(RemoteCameraSniper),

    #[variant_de(delegate)]
    RemoteCameraStrategic(RemoteCameraStrategic),

    SessionID(String),
    Team(u8),
    TkillIsSuspected(u8),
    WeatherPresetID(u8),
}

impl AvatarProperties {
    pub fn entity_type() -> EntityType {
        EntityType::Avatar
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct AmmoViews {
    veh_type_comp_descrs: Vec<i32>,
    comp_descrs:          Vec<Vec<i32>>,
}


#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct RemoteCamera {
    time:       f64,
    shot_point: Vector3,
    zoom:       u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct OrderingRoster {
    vehicle_id:   i32,
    prebattle_id: i32,
    team:         i8,
    observer:     u8,
}


#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct RemoteCameraArcade {
    time:            f64,
    rel_translation: Vector3,
    shot_point:      Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct RemoteCameraArty {
    time:        f64,
    shot_point:  Vector3,
    translation: Vector3,
    rotation:    Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct RemoteCameraSniper {
    time:                   f64,
    cam_matrix_translation: Vector3,
    cam_matrix_rotation:    Vector3,
    zoom:                   u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct RemoteCameraStrategic {
    time:       f64,
    shot_point: Vector3,
}


#[derive(Clone, Debug, Serialize, Deserialize, macros::Version)]
pub struct Goodie {
    goodie_id:  u32,
    lifetime:   u16,
    useby:      u64,
    resource:   GoodieResource,
    state_info: GoodieStateInfo,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodieResource {
    type_:         u8,
    value:         u16,
    is_percentage: u8,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodieStateInfo {
    state:       u8,
    finish_time: f64,
    count:       u16,
}

// pub enum HistoryLoggingFlagsValues {
//     #[version([0, 9, 12, 0])]
//    U16(u16)
//     #[version([0, 9, 15, 0])]
//    I32(i32)
//     #[version([1, 18, 1, 0])]
//    (<UInt64>)

// }

// pub enum PlayLimitsValues {
//     #[version([1, 7, 1, 0])]
//    (<Python>)
//     #[version([1, 8, 0, 0])]
//    (<FixedDict> {'curfew': <Int32>, 'weeklyPlayLimit': <Int32>, 'dailyPlayLimit': <Int32>})
//     #[version([1, 17, 1, 0])]
//    (<FixedDict> {'curfew': <Int32>, 'weeklyPlayLimit': <Int32>, 'dailyPlayLimit': <Int32>, 'sessionLimit':
// <Int32>})

// }

// pub enum PlayerVehicleTypeCompDescrValues {
//     #[version([0, 9, 12, 0])]
//    U16(u16),
//     #[version([1, 19, 1, 0])]
//    I32(i32),
// }
