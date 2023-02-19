use phf::phf_map;
use serde::{Deserialize, Serialize};

use super::VariantDeserializer;
use crate::packet_parser::types::Vector3;

#[derive(Clone, Debug, strum::IntoStaticStr, macros::EnumVariantDeserialize, Serialize, Deserialize)]
pub enum AvatarProperties {
    Account, //Mailbox
    AccountDBID(i64),
    AccountDBIDOnCell(i64),
    AccountSessionID(String),
    AccountStartedAt, // Pickle
    AiRosterVehicles(Vec<i32>),
    Ammo(Vec<i32>),
    AmmoViews(AmmoViews),
    Arena,     //Mailbox
    ArenaBase, //Mailbox
    ArenaBonusType(u8),
    ArenaBonusTypeOnCell(u8),
    ArenaExtraData, // Pickle
    ArenaGameParamRev(u64),
    ArenaGuiType(u8),
    ArenaTypeID(i32),
    ArenaTypeIDOnCell(i32),
    ArenaUniqueID(u64),
    ArenaUniqueIDOnCell(u64),
    ArenaVehiclesDBIDs, // Pickle
    ArenaVehiclesIDs,   // Pickle
    ClientCtx(String),
    ClientData, // Pickle
    CommanderVehicleID(i32),
    Cp, // Pickle
    CustomizationDisplayType(u8),
    DenunciationsLeft(i16),
    GameParamsRev(u64),
    GoodiesSnapshot(Vec<Goodie>),
    HeatmapLoggingFlags(u32),
    HistoryLoggingFlags, // Not supported yet
    InBattleQuestNames(Vec<String>),
    InBattleQuestProgresses, // Pickle
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
    OrderingRoster(Vec<OrderingRoster>),
    OwnVehicleAuxPhysicsData(u64),
    OwnVehicleGear(u8),
    PlayLimits,        // Not supported yet
    PlayerVehicle,     //Mailbox
    PlayerVehicleBase, //Mailbox
    PlayerVehicleID(i32),
    PlayerVehicleTypeCompDescr, // Not supported yet
    QuestProgressDescriptor,    // Pickle
    RemoteCamera(RemoteCamera),
    RemoteCameraArcade(RemoteCameraArcade),
    RemoteCameraArty(RemoteCameraArty),
    RemoteCameraSniper(RemoteCameraSniper),
    RemoteCameraStrategic(RemoteCameraStrategic),
    SessionID(String),
    State(u16),
    SyncData, // Pickle
    Team(u8),
    TkillIsSuspected(u8),
    Viewpoints(Vec<Vector3>),
    WeatherPresetID(u8),
}

pub static AVATAR_PROPS: phf::Map<&str, &'static [&'static str]> = phf_map! {
    "0_9_12_0" => &["ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "IsOwnVehicleContactingWorld", "NormalisedRPMPacked", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_13_0" => &["ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "IsOwnVehicleContactingWorld", "NormalisedRPMPacked", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_14_0" => &["ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_15_0" => &["ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_16_0" => &["ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_17_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCameraStrategic", "RemoteCameraArcade", "RemoteCameraSniper", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_18_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCameraStrategic", "RemoteCameraArcade", "RemoteCameraSniper", "RemoteCameraArty", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_19_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_20_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_21_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_22_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "0_9_23_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_0_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_0_1_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_0_2_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_1_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx", "QuestProgressDescriptor"],
    "1_2_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_3_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_4_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_4_1_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_5_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_5_1_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_6_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_6_1_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "ArenaExtraData", "ClientCtx"],
    "1_7_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_7_1_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "PlayLimits"],
    "1_8_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_9_0_0" =>  &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_10_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_10_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_11_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_11_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_12_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_12_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_13_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "IsHistoricallyAccurate", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx"],
    "1_14_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews", "ClientData"],
    "1_14_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews", "ClientData"],
    "1_15_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews"],
    "1_16_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews"],
    "1_16_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "IsAICommander", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews", "AiRosterVehicles"],
    "1_17_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews"],
    "1_17_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews"],
    "1_18_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews"],
    "1_18_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews", "GoodiesSnapshot"],
    "1_19_0_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews", "GoodiesSnapshot"],
    "1_19_1_0" => &["IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "ArenaBonusType", "ArenaGuiType", "WeatherPresetID", "TkillIsSuspected", "Team", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "CustomizationDisplayType", "DenunciationsLeft", "ArenaTypeID", "PlayerVehicleID", "ArenaUniqueID", "OwnVehicleAuxPhysicsData", "PlayLimits", "RemoteCamera", "Name", "SessionID", "ArenaExtraData", "ClientCtx", "AmmoViews", "GoodiesSnapshot"],
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmmoViews {
    veh_type_comp_descrs: Vec<i32>,
    comp_descrs:          Vec<Vec<i32>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCamera {
    time:       f64,
    shot_point: Vector3,
    zoom:       u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderingRoster {
    vehicle_id:   i32,
    prebattle_id: i32,
    team:         i8,
    observer:     u8,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraArcade {
    time:            f64,
    rel_translation: Vector3,
    shot_point:      Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraArty {
    time:        f64,
    shot_point:  Vector3,
    translation: Vector3,
    rotation:    Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraSniper {
    time:                   f64,
    cam_matrix_translation: Vector3,
    cam_matrix_rotation:    Vector3,
    zoom:                   u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraStrategic {
    time:       f64,
    shot_point: Vector3,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
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
