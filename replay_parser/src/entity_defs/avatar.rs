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
    "0_9_12_0" => &["State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "IsOwnVehicleContactingWorld", "NormalisedRPMPacked", "Ammo", "Cp", "HistoryLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_13_0" => &["State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "IsOwnVehicleContactingWorld", "NormalisedRPMPacked", "Ammo", "Cp", "HistoryLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_14_0" => &["State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_15_0" => &["State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_16_0" => &["State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_17_0" => &["RemoteCameraArcade", "RemoteCameraSniper", "RemoteCameraStrategic", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_18_0" => &["RemoteCameraArcade", "RemoteCameraSniper", "RemoteCameraStrategic", "RemoteCameraArty", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_19_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_20_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_21_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_22_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "0_9_23_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "1_0_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints"],
    "1_0_1_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate"],
    "1_0_2_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate"],
    "1_1_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "QuestProgressDescriptor"],
    "1_2_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_3_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_4_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_4_1_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_5_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_5_1_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_6_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_6_1_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "Account", "PlayerVehicle", "Arena", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_7_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesDBIDs", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses"],
    "1_7_1_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesDBIDs", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_8_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesDBIDs", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_9_0_0" =>  &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesDBIDs", "ArenaUniqueID", "ArenaTypeID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_10_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_10_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_11_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_11_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_12_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_12_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_13_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "IsHistoricallyAccurate", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_14_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits", "ClientData"],
    "1_14_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits", "ClientData"],
    "1_15_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_16_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_16_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits", "AiRosterVehicles", "IsAICommander", "CommanderVehicleID"],
    "1_17_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_17_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_18_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits"],
    "1_18_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits", "ArenaGameParamRev", "GoodiesSnapshot"],
    "1_19_0_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits", "ArenaGameParamRev", "GoodiesSnapshot"],
    "1_19_1_0" => &["RemoteCamera", "IsObserverFPV", "ObserverFPVControlMode", "NumOfObservers", "State", "Name", "SessionID", "Account", "PlayerVehicle", "Arena", "ArenaVehiclesIDs", "AccountStartedAt", "AccountSessionID", "ArenaUniqueID", "SyncData", "ArenaTypeID", "AccountDBID", "ArenaBonusType", "ArenaGuiType", "ArenaExtraData", "WeatherPresetID", "DenunciationsLeft", "ClientCtx", "TkillIsSuspected", "ArenaBase", "Team", "PlayerVehicleBase", "PlayerVehicleID", "PlayerVehicleTypeCompDescr", "IsObserverBothTeams", "ObservableTeamID", "IsGunLocked", "OwnVehicleGear", "OwnVehicleAuxPhysicsData", "Ammo", "AmmoViews", "Cp", "HistoryLoggingFlags", "HeatmapLoggingFlags", "GameParamsRev", "AccountDBIDOnCell", "ArenaUniqueIDOnCell", "ArenaTypeIDOnCell", "ArenaBonusTypeOnCell", "OrderingRoster", "Viewpoints", "CustomizationDisplayType", "InBattleQuestNames", "InBattleQuestProgresses", "PlayLimits", "ArenaGameParamRev", "GoodiesSnapshot"],

};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmmoViews {
    vehTypeCompDescrs: Vec<i32>,
    compDescrs:        Vec<Vec<i32>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCamera {
    time:      f64,
    shotPoint: Vector3,
    zoom:      u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderingRoster {
    vehicleID:   i32,
    prebattleID: i32,
    team:        i8,
    observer:    u8,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraArcade {
    time:           f64,
    relTranslation: Vector3,
    shotPoint:      Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraArty {
    time:        f64,
    shotPoint:   Vector3,
    translation: Vector3,
    rotation:    Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraSniper {
    time:                 f64,
    camMatrixTranslation: Vector3,
    camMatrixRotation:    Vector3,
    zoom:                 u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteCameraStrategic {
    time:      f64,
    shotPoint: Vector3,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Goodie {
    goodieID:   u32,
    lifetime:   u16,
    useby:      u64,
    resource:   GoodieResource,
    state_info: GoodieStateInfo,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodieResource {
    type_:        u8,
    value:        u16,
    isPercentage: u8,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoodieStateInfo {
    state:      u8,
    finishTime: f64,
    count:      u16,
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
