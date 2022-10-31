use macros::{EventPrinter, Version};
use serde::{Deserialize, Serialize};

use super::Vector3;
use crate::events::{EventPrinter, Version, VersionInfo};

#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
/// This info is present for the player vehicle or the observed vehicle
pub struct UpdateVehicleMiscStatus {
    avatar_id: i32,
    vehicle_misc_status: u8,
    status_data: i32,
    other_args: Vec<f32>
}


enum VehicleMiscStatus {
    OtherVehicleDamagedDevicesVisible = 0,
    IsObservedByEnemy = 1,
    LoaderIntuitionWasUsed = 2,
    VehicleIsOverturned = 3,
    VehicleDrownWarning = 4,
    InDeathZone = 5,
    DestroyedDeviceIsRepairing = 7,
    SiegeModeStateChanged = 9,
    BurnoutWarning = 10,
    BurnoutUnavailableDueToBrokenEngine = 11,
    DualgunChargerState = 14,
}