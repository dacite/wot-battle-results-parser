use crate::packet_parser::prelude::*;

#[derive(Debug, Clone)]
pub struct VehicleMiscStatus {
    pub avatar_id:     i32,
    pub status_detail: VehicleMiscStatusDetail,
}
#[derive(Debug, Clone)]
pub enum VehicleMiscStatusDetail {
    IsObservedByEnemy(bool),
}
impl VehicleMiscStatus {
    pub fn from(data: &[u8], version: [u16; 4]) -> Result<Self, PacketError> {
        let misc_status: UpdateVehicleMiscStatus = from_slice(data, version)?;

        use VehicleMiscStatusCode as Code;
        use VehicleMiscStatusDetail::*;
        let status_detail = {
            match misc_status.status_code {
                _ => IsObservedByEnemy(!(misc_status.status_data == 0)),
            }
        };

        Ok(VehicleMiscStatus {
            avatar_id: misc_status.avatar_id,
            status_detail,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
/// This info is present for the player vehicle or the observed vehicle
pub struct UpdateVehicleMiscStatus {
    pub avatar_id:   i32,
    pub status_code: u8,
    pub status_data: i32,
    pub other_args:  Vec<f32>,
}


enum VehicleMiscStatusCode {
    OtherVehicleDamagedDevicesVisible = 0,
    IsObservedByEnemy          = 1,
    LoaderIntuitionWasUsed     = 2,
    VehicleIsOverturned        = 3,
    VehicleDrownWarning        = 4,
    InDeathZone                = 5,
    DestroyedDeviceIsRepairing = 7,
    SiegeModeStateChanged      = 9,
    BurnoutWarning             = 10,
    BurnoutUnavailableDueToBrokenEngine = 11,
    DualgunChargerState        = 14,
}

// trait DecodeVehicleMiscStatus {
//     fn decode(misc_status: UpdateVehicleMiscStatus) -> Result<Self> where Self: std::marker::Sized;
// }

// struct ObservedByEnemy(bool);

// impl DecodeVehicleMiscStatus for ObservedByEnemy {
//     fn decode(misc_status: UpdateVehicleMiscStatus) -> Result<Self> {
//         assert!(misc_status.status_code == VehicleMiscStatusCode::IsObservedByEnemy as u8);

//         if misc_status.status_data == 0 {
//             Ok(ObservedByEnemy(true))
//         } else {
//             Err(crate::Error::Other("Unknown status data for observed by enemy".into()))
//         }

//     }
// }
