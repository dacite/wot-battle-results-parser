use serde_pickle::Value as PickleVal;

use super::ArenaUpdateData;
use crate::{
    events::entity_method::avatar_methods::update_arena::vehicle_list::parse_vehicle_data,
    packet_parser::prelude::*,
};

pub fn parse_vehicle_added(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let decompressed =
        utils::decompress_vec(arena_data, |err| PacketError::ConversionError(err.to_string()))?;
    let pickle_value = serde_pickle::value_from_slice(
        &decompressed,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .unwrap();

    let PickleVal::Tuple(vehicle_data) = pickle_value  else { todo!() };

    Ok(ArenaUpdateData::VehicleAdded(parse_vehicle_data(vehicle_data)?))
}
