use serde_pickle::Value as PickleVal;

use super::{
    parse_truthy_value, parse_value,
    vehicle_descr::{parse_compact_descr, VehicleCompactDescr},
    ArenaUpdateData,
};
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct VehicleData {
    pub vehicle_compact_descr: Option<VehicleCompactDescr>,
    pub name:                  String,
    pub team:                  i64,
    pub is_alive:              i64,
    pub is_avatar_ready:       i64,
    pub is_team_killer:        i64,
    pub account_dbid:          i64,
    pub clan_abbrev:           String,
    pub clan_dbid:             i64,
    pub pre_battle_id:         i64,
}

pub fn parse_vehicle_data(vehicle_data: Vec<PickleVal>) -> Result<VehicleData, PacketError> {
    let vehicle_compact_descr = if let PickleVal::Bytes(compact_descr) = vehicle_data.get(1).unwrap() {
        Some(parse_compact_descr(compact_descr.clone()))
    } else {
        None
    };

    Ok(VehicleData {
        vehicle_compact_descr,
        name: parse_value(2, &vehicle_data)?,
        team: parse_value(3, &vehicle_data)?,
        is_alive: parse_truthy_value(4, &vehicle_data)?,
        is_avatar_ready: parse_truthy_value(5, &vehicle_data)?,
        is_team_killer: parse_truthy_value(6, &vehicle_data)?,
        account_dbid: parse_value(7, &vehicle_data)?,
        clan_abbrev: parse_value(8, &vehicle_data)?,
        clan_dbid: parse_value(9, &vehicle_data)?,
        pre_battle_id: parse_value(10, &vehicle_data)?,
    })
}

pub fn parse_vehicle_list(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let decompressed = utils::decompress_vec(arena_data, |err| PacketError::ConversionError {
        err: err.to_string(),
    })?;
    let pickle_value = serde_pickle::value_from_slice(
        &decompressed,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .unwrap();

    let PickleVal::List(list) = pickle_value  else { todo!() };
    let mut vehicle_list = Vec::new();
    for thing in list {
        let PickleVal::Tuple(thing) = thing  else { todo!() };
        let vehicle_data = parse_vehicle_data(thing)?;

        vehicle_list.push(vehicle_data);
    }
    Ok(ArenaUpdateData::VehicleList(vehicle_list))
}
