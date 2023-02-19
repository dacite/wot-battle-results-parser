use serde_pickle::Value as PickleVal;
use wot_types::AttackReason;

use super::{parse_value, UpdateData};
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize, Version)]
pub struct VehicleKilled {
    pub victim_id:     i32,
    pub killer_id:     i32,
    pub equipment_id:  i32,
    pub attack_reason: AttackReason,

    #[version([1, 17, 0, 0])]
    pub num_vehicles_affected: Option<i32>,
}

pub fn parse_vehicle_killed(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::Tuple(thing) = pickle_value else { todo!() };

    let num_vehicles_affected = if thing.len() > 4 {
        parse_value(4, &thing)?
    } else {
        None
    };

    let attack_reason: i32 = parse_value(3, &thing)?;

    let vehicle_killed = VehicleKilled {
        attack_reason: AttackReason::try_from(attack_reason as i32).map_err(|_| {
            PacketError::WrongEnumVariant(format!("arena attack reason of {attack_reason} is invalid"))
        })?,
        victim_id: parse_value(0, &thing)?,
        killer_id: parse_value(1, &thing)?,
        equipment_id: parse_value(2, &thing)?,
        num_vehicles_affected,
    };

    Ok(UpdateData::VehicleKilled(vehicle_killed))
}
