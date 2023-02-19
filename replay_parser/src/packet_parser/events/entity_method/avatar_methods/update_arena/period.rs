use serde_pickle::Value as PickleVal;
use wot_types::{ArenaPeriod, FinishReason};

use super::{parse_value, ArenaUpdateData};
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, Serialize, Version)]
pub struct ArenaEnded {
    pub winner_team:   i32,
    pub finish_reason: FinishReason,
}

#[derive(Debug, Clone, Serialize)]
pub enum PeriodAdditionalInfo {
    ActivitiesStartTimes(Vec<f32>),
    ArenaEnded(ArenaEnded),
}

#[derive(Debug, Clone, Serialize, Version)]
pub struct Period {
    pub period:          ArenaPeriod,
    pub end_time:        f32,
    pub length:          f32,
    pub additional_info: PeriodAdditionalInfo,
}

pub fn parse_period(arena_data: &[u8]) -> Result<ArenaUpdateData, PacketError> {
    let decompressed =
        utils::decompress_vec(arena_data, |err| PacketError::ConversionError(err.to_string()))?;
    let pickle_value = serde_pickle::value_from_slice(
        &decompressed,
        serde_pickle::DeOptions::new().replace_unresolved_globals(),
    )
    .unwrap();

    let PickleVal::Tuple(thing) = pickle_value else { todo!() };
    let period_type: i32 = parse_value(0, &thing)?;

    let additional_info = match &thing[3] {
        PickleVal::Tuple(info) => {
            let finish_reason = parse_value::<i32>(1, &info)?;
            Ok(PeriodAdditionalInfo::ArenaEnded(ArenaEnded {
                winner_team:   parse_value(0, &info)?,
                finish_reason: FinishReason::try_from(finish_reason).map_err(|_| {
                    PacketError::WrongEnumVariant(format!("finish reason of {finish_reason} is invalid"))
                })?,
            }))
        }
        PickleVal::List(start_times) => {
            let mut res: Vec<f32> = vec![];
            for i in 0..start_times.len() {
                res.push(parse_value::<f32>(i, start_times)?);
            }
            Ok(PeriodAdditionalInfo::ActivitiesStartTimes(res))
        }
        _ => Err(PacketError::PickleError(format!(
            "Invalid additional info payload"
        ))),
    }?;

    let period = Period {
        period: ArenaPeriod::try_from(period_type as i32).map_err(|_| {
            PacketError::WrongEnumVariant(format!("arena period of {period_type} is invalid"))
        })?,
        end_time: parse_value(1, &thing)?,
        length: parse_value(2, &thing)?,
        additional_info,
    };

    Ok(ArenaUpdateData::Period(period))
}
