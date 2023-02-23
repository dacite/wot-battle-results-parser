mod avatar_ready;
mod base;
mod fog_of_war;
mod period;
mod vehicle_added;
mod vehicle_descr;
mod vehicle_killed;
mod vehicle_list;
mod vehicle_statistics;
mod vehicle_updated;

use avatar_ready::{parse_avatar_ready, AvatarReady};
use base::{parse_base_captured, parse_base_points, BaseCaptured, BasePoints};
use fog_of_war::{parse_fog_of_war, FogOfWar};
use nom::number::complete::le_u8;
use period::{parse_period, Period};
use serde_pickle::Value as PickleVal;
use vehicle_added::parse_vehicle_added;
use vehicle_descr::{parse_vehicle_descr, VehicleDescr};
use vehicle_killed::{parse_vehicle_killed, VehicleKilled};
use vehicle_list::{parse_vehicle_list, VehicleData};
use vehicle_statistics::{parse_statistics, parse_vehicle_statistics, VehicleStatistics};
use vehicle_updated::parse_vehicle_updated;
use wot_types::ArenaUpdate;

use crate::packet_parser::prelude::*;
use crate::packet_parser::serde_packet;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateArena {
    pub update_type: ArenaUpdate,
    pub update_data: ArenaUpdateData,
}

#[derive(Debug, Clone, Serialize)]
pub enum ArenaUpdateData {
    VehicleList(Vec<VehicleData>),
    AvatarReady(AvatarReady),
    VehicleKilled(VehicleKilled),
    BasePoints(BasePoints),
    BaseCaptured(BaseCaptured),
    VehicleStatistics(VehicleStatistics),
    Statistics(Vec<VehicleStatistics>),
    Period(Period),
    VehicleDescr(VehicleDescr),
    FogOfWar(FogOfWar),
    VehicleAdded(VehicleData),
    VehicleUpdated(VehicleData),
    Unimplemented,
}

impl UpdateArena {
    pub fn from(data: &[u8], _version: [u16; 4]) -> Result<Self, PacketError> {
        let (remaining, update_type) = le_u8(data)?;
        let (_remaining, arena_data) = serde_packet::parse_byte_array(remaining)?;

        let update_type =
            ArenaUpdate::try_from(update_type as i32).map_err(|_| PacketError::ConversionError {
                err: format!("Unable to parse {update_type} into ArenaUpdate"),
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
            Period => parse_period(arena_data)?,
            VehicleDescr => parse_vehicle_descr(arena_data)?,
            FogOfWar => parse_fog_of_war(arena_data)?,
            VehicleAdded => parse_vehicle_added(arena_data)?,
            VehicleUpdated => parse_vehicle_updated(arena_data)?,
            _ => ArenaUpdateData::Unimplemented,
        };

        Ok(UpdateArena {
            update_type,
            update_data,
        })
    }
}

fn parse_value<'de, T: Deserialize<'de>>(index: usize, pickle_val: &[PickleVal]) -> Result<T, PacketError> {
    let pickle_val = pickle_val
        .get(index)
        .ok_or_else(|| PacketError::PickleError {
            err: format!("Cannot get index: {index}"),
        })?
        .clone();

    serde_pickle::from_value(pickle_val).map_err(|err| PacketError::PickleError { err: err.to_string() })
}

fn parse_truthy_value(index: usize, pickle_val: &[PickleVal]) -> Result<i64, PacketError> {
    let pickle_val = pickle_val.get(index).ok_or_else(|| PacketError::PickleError {
        err: format!("Cannot get index: {index}"),
    })?;

    match pickle_val {
        PickleVal::Bool(value) => Ok(*value as i64),
        PickleVal::I64(value) => Ok(*value),
        _ => Err(PacketError::PickleError {
            err: "Pickle error: expected value to be boolean or integer".into(),
        }),
    }
}
