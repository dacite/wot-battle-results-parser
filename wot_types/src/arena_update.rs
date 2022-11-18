use num_enum::TryFromPrimitive;
use serde::Serialize;

#[repr(i32)]
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive, strum::Display, Serialize)]
pub enum ArenaUpdate {
    VehicleList               = 1,
    VehicleAdded              = 2,
    Period                    = 3,
    Statistics                = 4,
    VehicleStatistics         = 5,
    VehicleKilled             = 6,
    AvatarReady               = 7,
    BasePoints                = 8,
    BaseCaptured              = 9,
    TeamKiller                = 10,
    VehicleUpdated            = 11,
    CombatEquipmentUsed       = 12,
    FlagTeams                 = 16,
    FlagStateChanged          = 17,
    InteractiveStats          = 19,
    ResourcePointStateChanged = 21,
    OwnVehicleInsideRp        = 22,
    OwnVehicleLockedForRp     = 23,
    SyncObjects               = 24,
    SyncObjectsDiff           = 25,
    ViewPoints                = 26,
    FogOfWar                  = 27,
    VehicleRecovered          = 28,
    RadarInfoReceived         = 29,
    Settings                  = 30,
    VehicleDescr              = 31,
    BasesList                 = 32,
    CommanderDataList         = 33,
    CommanderDataVehicle      = 34,
    GodModeChanged            = 35,
}
