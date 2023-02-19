use serde_pickle::Value as PickleVal;

use super::UpdateData;
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize, Version)]
pub struct FogOfWar {
    pub is_enabled:          bool,
    pub has_hidden_vehicles: bool,
}

pub fn parse_fog_of_war(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::I64(status) = pickle_value else { todo!() };

    let fog_of_war = FogOfWar {
        is_enabled:          status & 1 != 0,
        has_hidden_vehicles: status & 2 != 0,
    };

    Ok(UpdateData::FogOfWar(fog_of_war))
}
