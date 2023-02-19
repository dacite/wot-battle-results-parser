use serde_pickle::Value as PickleVal;

use super::{parse_value, UpdateData};
use crate::packet_parser::prelude::*;

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

pub fn parse_base_points(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
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

pub fn parse_base_captured(arena_data: &[u8]) -> Result<UpdateData, PacketError> {
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
