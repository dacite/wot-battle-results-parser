use serde_pickle::Value as PickleVal;

use super::{parse_value, ArenaUpdateData};
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize, Version)]
pub struct VehicleStatistics {
    pub vehicle_id: i32,
    pub frags:      i32,
}

pub fn parse_vehicle_statistics(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let decompressed =
        utils::decompress_vec(arena_data, |err| PacketError::ConversionError(err.to_string()))?;
    let pickle_value = serde_pickle::value_from_slice(
        &decompressed,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .unwrap();

    let PickleVal::Tuple(thing) = pickle_value else { todo!() };

    let vehicle_statistics = VehicleStatistics {
        vehicle_id: parse_value(0, &thing)?,
        frags:      parse_value(1, &thing)?,
    };

    Ok(ArenaUpdateData::VehicleStatistics(vehicle_statistics))
}

pub fn parse_statistics(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let decompressed =
        utils::decompress_vec(arena_data, |err| PacketError::ConversionError(err.to_string()))?;
    let pickle_value = serde_pickle::value_from_slice(
        &decompressed,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .unwrap();

    let PickleVal::List(stats_list) = pickle_value  else {
        return Err(PacketError::PickleError("Expected List value for payload".into()))
    };

    let mut res: Vec<VehicleStatistics> = vec![];
    for stats in stats_list.into_iter() {
        let PickleVal::Tuple(items) = stats  else {
            return Err(PacketError::PickleError("Expected Tuple value for stats".into()))
        };
        res.push(VehicleStatistics {
            vehicle_id: parse_value(0, &items)?,
            frags:      parse_value(1, &items)?,
        })
    }

    Ok(ArenaUpdateData::Statistics(res))
}
