use nom::number::complete::le_u8;
use serde_pickle::Value as PickleVal;
use wot_types::{ArenaUpdate, AttackReason};

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
    VehicleKilled(VehicleKilled),
    BasePoints(BasePoints),
    BaseCaptured(BaseCaptured),
    VehicleStatistics(VehicleStatistics),
    Statistics(Vec<VehicleStatistics>),
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
            VehicleKilled => parse_vehicle_killed(arena_data)?,
            BasePoints => parse_base_points(arena_data)?,
            BaseCaptured => parse_base_captured(arena_data)?,
            VehicleStatistics => parse_vehicle_statistics(arena_data)?,
            Statistics => parse_statistics(arena_data)?,
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
            clan_abbrev: parse_value(8, &thing)?,
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
    pub clan_abbrev:           String,
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
    pub victim_id:     i32,
    pub killer_id:     i32,
    pub equipment_id:  i32,
    pub attack_reason: AttackReason,

    #[version([1, 17, 0, 0])]
    pub num_vehicles_affected: Option<i32>,
}

fn parse_vehicle_killed(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
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

#[derive(Debug, Clone, Serialize, Version)]
pub struct BasePoints {
    pub team:    i32,
    pub base_id: i32,
    pub points:  i32,

    #[version([0, 9, 15, 0])]
    pub time_left: Option<i32>,

    #[version([0, 9, 15, 0])]
    pub invaders_cnt: Option<i32>,

    pub capturing_stopped: bool,
}

fn parse_base_points(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::Tuple(thing) = pickle_value else { todo!() };

    if thing.len() >= 6 {
        Ok(UpdateData::BasePoints(BasePoints {
            team:              parse_value(0, &thing)?,
            base_id:           parse_value(1, &thing)?,
            points:            parse_value(2, &thing)?,
            time_left:         parse_value(3, &thing)?,
            invaders_cnt:      parse_value(4, &thing)?,
            capturing_stopped: parse_value::<i64>(5, &thing)? != 0,
        }))
    } else {
        Ok(UpdateData::BasePoints(BasePoints {
            team:              parse_value(0, &thing)?,
            base_id:           parse_value(1, &thing)?,
            points:            parse_value(2, &thing)?,
            time_left:         None,
            invaders_cnt:      None,
            capturing_stopped: parse_value::<i64>(3, &thing)? != 0,
        }))
    }
}

#[derive(Debug, Clone, Serialize, Version)]
pub struct BaseCaptured {
    pub team:    i32,
    pub base_id: i32,
}

fn parse_base_captured(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
    let pickle_value = serde_pickle::value_from_slice(
        arena_data,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )?;

    let PickleVal::Tuple(thing) = pickle_value else { todo!() };

    Ok(UpdateData::BaseCaptured(BaseCaptured {
        team:    parse_value(0, &thing)?,
        base_id: parse_value(1, &thing)?,
    }))
}

#[derive(Debug, Clone, Serialize, Version)]
pub struct VehicleStatistics {
    pub vehicle_id: i32,
    pub frags:      i32,
}

fn parse_vehicle_statistics(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
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

    Ok(UpdateData::VehicleStatistics(vehicle_statistics))
}

fn parse_statistics(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
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

    Ok(UpdateData::Statistics(res))
}
