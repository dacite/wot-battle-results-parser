use serde_pickle::Value as PickleVal;

use super::{parse_value, ArenaUpdateData};
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct VehicleCompactDescr {
    pub nation_id:             u8,
    pub vehicle_type_id:       u8,
    pub components:            Vec<u8>,
    pub optional_device_slots: i32,
    pub optional_devices:      Vec<u8>,
}

pub fn parse_compact_descr(compact_descr: Vec<u8>) -> VehicleCompactDescr {
    let header = compact_descr[0];
    assert!(header & 15 == 1);

    let nation_id = header >> 4 & 15;
    let vehicle_type_id = compact_descr[1];

    let mut idx = 10 + (1) * 4;
    let components = compact_descr[2..idx].to_vec();

    let flags = compact_descr[idx];
    idx += 1;

    let mut count = 0;
    let mut optional_device_slots = 0;

    for i in 0..3 {
        if (flags & 1 << i) != 0 {
            count += 1;
            optional_device_slots |= 1 << i;
        }
    }

    let optional_devices = compact_descr[idx..(idx + count * 2)].to_vec();

    assert!(optional_devices.len() % 2 == 0);
    VehicleCompactDescr {
        nation_id,
        vehicle_type_id,
        components,
        optional_device_slots,
        optional_devices,
    }
}

#[derive(Debug, Clone, Serialize, Version)]
pub struct VehicleDescr {
    pub vehicle_id:    i32,
    pub compact_descr: VehicleCompactDescr,
    pub max_health:    i32,
}

pub fn parse_vehicle_descr(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::Tuple(thing) = pickle_value else { todo!() };

    let compact_descr = if let PickleVal::Bytes(compact_descr) = thing.get(1).unwrap() {
        parse_compact_descr(compact_descr.clone())
    } else {
        return Err(PacketError::PickleError{ err: format!(
            "Invalid vehicle compact description"
        )});
    };

    Ok(ArenaUpdateData::VehicleDescr(VehicleDescr {
        vehicle_id: parse_value(0, &thing)?,
        compact_descr,
        max_health: parse_value(2, &thing)?,
    }))
}
