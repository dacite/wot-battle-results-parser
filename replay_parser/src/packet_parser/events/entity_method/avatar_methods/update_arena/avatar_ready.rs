use serde_pickle::Value as PickleVal;

use super::ArenaUpdateData;
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct AvatarReady {
    vehicle_id: i32,
}

pub fn parse_avatar_ready(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::I64(vehicle_id) = pickle_value  else {
        return Err(PacketError::PickleError{ err: "Expected I64 value for vehicle id".into()})
    };

    let avatar_ready = AvatarReady {
        vehicle_id: vehicle_id as i32,
    };

    Ok(ArenaUpdateData::AvatarReady(avatar_ready))
}
