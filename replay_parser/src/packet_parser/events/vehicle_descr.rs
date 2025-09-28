use serde::Serialize;

use super::EntMethodGeneralInfo;
use crate::packet_parser::InputStream;
use crate::utils::{make_pickle_val, PickleOps};
use crate::wot_data::OptionalDevice;
use crate::{Context, PacketError};

#[derive(Debug, Clone, Serialize)]
pub struct VehicleDescr {
    pub vehicle_id:    i32,
    pub compact_descr: VehicleCompactDescr,
    pub max_health:    i32,
}


impl VehicleDescr {
    // NOTE: Picking the values of the picle value ourselves is faster (like .list())
    pub fn parse_from(
        _gen_info: EntMethodGeneralInfo, arena_data: &[u8], context: &Context,
    ) -> Result<VehicleDescr, PacketError> {
        let data: (i32, Vec<u8>, i32) = make_pickle_val(arena_data)?.deser()?;
        let (vehicle_id, compact_descr_bytes, max_health) = data;

        Ok(VehicleDescr {
            vehicle_id,
            compact_descr: parse_compact_descr(compact_descr_bytes, context.version)?,
            max_health,
        })
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct VehicleCompactDescr {
    pub nation_id:        u8,
    pub vehicle_type_id:  u32,
    pub components:       Vec<u8>,
    pub optional_devices: Vec<Option<OptionalDevice>>,
    pub enhancements:     Option<Vec<u8>>,
}

const EXTENDED_VEHICLE_TYPE_ID_FLAG: u8 = 2;

pub fn parse_compact_descr(
    compact_descr: Vec<u8>, version: [u16; 4],
) -> Result<VehicleCompactDescr, PacketError> {
    let header = compact_descr[0];

    let mut vehicle_type_id = compact_descr[1] as u32;

    let mut veh_type_offset = 0;

    if (header & EXTENDED_VEHICLE_TYPE_ID_FLAG) > 0 {
        vehicle_type_id += (compact_descr[2] as u32) << 8;
        veh_type_offset += 1;
    }
    // if header & 15 != 1 {
    //     return Err(PacketError::Misc {
    //         err: format!("Unable to parse vehicle compact description"),
    //     });
    // }

    let nation_id = header >> 4 & 15;


    let mut idx = 10 + veh_type_offset + (1) * 4;
    let components = compact_descr[(2 + veh_type_offset)..idx].to_vec();

    let flags = compact_descr[idx];
    idx += 1;

    let mut count = 0;
    let mut optional_device_slots = 0;

    let max_optional_devices_slots: usize = if version >= [1, 10, 0, 0] { 4 } else { 3 };

    for i in 0..max_optional_devices_slots {
        if (flags & 1 << i) > 0 {
            count += 1;
            optional_device_slots |= 1 << i;
        }
    }

    let optional_devices = compact_descr[idx..(idx + count * 2)].to_vec();

    idx += count * 2;

    let enhancements = if version >= [1, 7, 1, 0] && (flags & 16 == 1) {
        count = compact_descr[idx] as usize;

        let result = compact_descr[idx..idx + 1 + count * 6].to_vec();
        // idx += 1 + count * 6; TODO: Uncomment if parsing emblems, inscriptions and camo

        Some(result)
    } else {
        None
    };

    // if optional_devices.len() % 2 != 0 {
    //     return Err(PacketError::Misc {
    //         err: format!("Unable to parse vehicle compact description"),
    //     });
    // }


    // while optionalDeviceSlots:
    // if optionalDeviceSlots & 1:
    //     self.optionalDevices[idx] = optDevsCache[unpack('<H', optionalDevices[:2])[0]]
    //     optionalDevices = optionalDevices[2:]
    // optionalDeviceSlots >>= 1
    // idx -= 1

    Ok(VehicleCompactDescr {
        nation_id,
        vehicle_type_id,
        components,
        optional_devices: vec![],
        enhancements,
    })
}

fn parse_optional_devices(
    mut optional_device_slots: i32, optional_devices: Vec<u8>,
) -> Vec<Option<OptionalDevice>> {
    let mut devices = vec![None, None, None];
    let total_possible_slots = 3; // lower tier tanks have less than 3 equpment slots?
    let mut idx = total_possible_slots - 1;

    // TODO: Unknown equipments are considered the same as not having an equipment mounted
    while optional_device_slots > 0 {
        if (optional_device_slots & 1) > 0 {
            let mut stream = InputStream::from(&optional_devices);
            devices[idx] = stream
                .le_u16()
                .ok()
                .and_then(|it| OptionalDevice::from_number(it as i32));
        }
        optional_device_slots >>= 1;
        idx -= 1;
    }

    devices
}
