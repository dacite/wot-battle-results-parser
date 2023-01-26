use nom::number::complete::le_u8;
use serde_pickle::Value as PickleVal;
use wot_types::ArenaUpdate;

use crate::packet_parser::prelude::*;
use crate::packet_parser::serde_packet;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateArena {
    pub update_type: ArenaUpdate,
    pub update_data: UpdateData,
}

#[derive(Debug, Clone, Serialize)]
pub enum UpdateData {
    VehicleList(Vec<VehicleData>),
    AvatarReady(AvatarReady),
    Unimplemented,
}

#[derive(Debug, Clone, Serialize)]
pub struct VehicleList {}
impl UpdateArena {
    pub fn from(data: &[u8], _version: [u16; 4]) -> Result<Self, PacketError> {
        let (remaining, update_type) = le_u8(data)?;
        let (_remaining, arena_data) = serde_packet::parse_byte_array(remaining)?;

        let update_type = ArenaUpdate::try_from(update_type as i32).map_err(|_| {
            PacketError::ConversionError(format!("Unable to parse {update_type} into ArenaUpdate"))
        })?;

        use ArenaUpdate::*;
        let update_data = match update_type {
            VehicleList => parse_vehicle_list(arena_data)?,
            AvatarReady => parse_avatar_ready(arena_data)?,
            _ => UpdateData::Unimplemented,
        };

        Ok(UpdateArena {
            update_type,
            update_data,
        })
    }
}

fn parse_vehicle_list(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
    let decompressed =
        utils::decompress_vec(arena_data, |err| PacketError::ConversionError(err.to_string()))?;
    let pickle_value = serde_pickle::value_from_slice(
        &decompressed,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .unwrap();

    let PickleVal::List(list) = pickle_value  else { todo!() };
    let mut vehicle_list = Vec::new();
    for thing in list {
        let PickleVal::Tuple(thing) = thing  else { todo!() };

        let vehicle_compact_descr;
        if let PickleVal::Bytes(compact_descr) = thing.get(1).unwrap() {
            vehicle_compact_descr = parse_compact_descr(compact_descr.clone());
        } else {
            // TODO: REMOVE PANIC
            panic!("OH NO");
        }
        let vehicle_data = VehicleData {
            vehicle_compact_descr,
            name: parse_value(2, &thing)?,
            team: parse_value(3, &thing)?,
            is_alive: parse_truthy_value(4, &thing)?,
            is_avatar_ready: parse_truthy_value(5, &thing)?,
            is_team_killer: parse_truthy_value(6, &thing)?,
            account_dbid: parse_value(7, &thing)?,
            clan_abbrev: parse_value(9, &thing)?,
            clan_dbid: parse_value(9, &thing)?,
            pre_battle_id: parse_value(10, &thing)?,
        };

        vehicle_list.push(vehicle_data);
    }
    Ok(UpdateData::VehicleList(vehicle_list))
}

fn parse_value<'de, T: Deserialize<'de>>(index: usize, pickle_val: &[PickleVal]) -> Result<T, PacketError> {
    let pickle_val = pickle_val
        .get(index)
        .ok_or_else(|| PacketError::PickleError(format!("Cannot get index: {index}")))?
        .clone();

    serde_pickle::from_value(pickle_val).map_err(|err| PacketError::PickleError(err.to_string()))
}

fn parse_truthy_value(index: usize, pickle_val: &[PickleVal]) -> Result<i64, PacketError> {
    let pickle_val = pickle_val
        .get(index)
        .ok_or_else(|| PacketError::PickleError(format!("Cannot get index: {index}")))?;

    match pickle_val {
        PickleVal::Bool(value) => Ok(*value as i64),
        PickleVal::I64(value) => Ok(*value),
        _ => Err(PacketError::PickleError(
            "Pickle error: expected value to be boolean or integer".into(),
        )),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VehicleData {
    pub vehicle_compact_descr: VehicleCompactDescr,
    pub name:                  String,
    pub team:                  i64,
    pub is_alive:              i64,
    pub is_avatar_ready:       i64,
    pub is_team_killer:        i64,
    pub account_dbid:          i64,
    pub clan_abbrev:           i64,
    pub clan_dbid:             i64,
    pub pre_battle_id:         i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct VehicleCompactDescr {
    pub nation_id:             u8,
    pub vehicle_type_id:       u8,
    pub components:            Vec<u8>,
    pub optional_device_slots: i32,
    pub optional_devices:      Vec<u8>,
}

fn parse_compact_descr(compact_descr: Vec<u8>) -> VehicleCompactDescr {
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

#[derive(Debug, Clone, Serialize)]
pub struct AvatarReady {
    vehicle_id: i32,
}

fn parse_avatar_ready(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::I64(vehicle_id) = pickle_value  else {
        return Err(PacketError::PickleError("Expected I64 value for vehicle id".into()))
    };

    let avatar_ready = AvatarReady {
        vehicle_id: vehicle_id as i32,
    };

    Ok(UpdateData::AvatarReady(avatar_ready))
}
#[derive(Debug, Clone, Serialize, Version)]
pub struct VehicleKilled {
    pub victim_id:    i32,
    pub killer_id:    i32,
    pub equipment_id: i32,
    pub reason:       i32,
    //TODO: More fields in later versions
}
