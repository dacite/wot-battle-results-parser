use serde_json::Map;
use serde_json::Value as JsonVal;
use time::Duration;

use crate::{ReplayError, VERSIONS};

/// `[0, 9, 15, 0]` => `"0_9_15_0"`
pub fn version_as_string(version: [u16; 4]) -> String {
    version.map(|x| x.to_string()).join("_")
}

pub fn get_replay_time(start_time: f64, current_time: f64, duration: i64) -> String {
    let total_time = Duration::minutes(duration);

    let actual_time = total_time - Duration::seconds_f64(current_time - start_time);

    format!(
        "{}:{:02}",
        actual_time.whole_minutes(),
        actual_time.whole_seconds() % 60
    )
}

pub fn as_map<'a>(path: &'static str, json: &'a JsonVal) -> Result<&'a Map<String, JsonVal>, ReplayError> {
    json.pointer(path)
        .ok_or(ReplayError::JsonPathError(path))?
        .as_object()
        .ok_or_else(|| {
            ReplayError::JsonTypeError(format!("expected map for {path}, found {}", get_json_type(json)))
        })
}

#[allow(dead_code)]
pub fn as_array<'a>(path: &'static str, json: &'a JsonVal) -> Result<&'a Vec<JsonVal>, ReplayError> {
    json.pointer(path)
        .ok_or(ReplayError::JsonPathError(path))?
        .as_array()
        .ok_or_else(|| {
            ReplayError::JsonTypeError(format!(
                "expected array for {path}, found {}",
                get_json_type(json)
            ))
        })
}

#[allow(dead_code)]
pub fn as_string(path: &'static str, json: &JsonVal) -> Result<String, ReplayError> {
    let as_str = json
        .pointer(path)
        .ok_or(ReplayError::JsonPathError(path))?
        .as_str()
        .ok_or_else(|| {
            ReplayError::JsonTypeError(format!("expected str for {path}, found {}", get_json_type(json)))
        })?;

    Ok(as_str.into())
}

pub fn as_i64(path: &'static str, json: &JsonVal) -> Result<i64, ReplayError> {
    json.pointer(path)
        .ok_or(ReplayError::JsonPathError(path))?
        .as_i64()
        .ok_or_else(|| {
            ReplayError::JsonTypeError(format!("expected i64 for {path}, found {}", get_json_type(json)))
        })
}

pub const fn get_json_type(json: &JsonVal) -> &'static str {
    match json {
        JsonVal::Null => "null",
        JsonVal::Bool(_) => "boolean",
        JsonVal::Number(_) => "number",
        JsonVal::String(_) => "string",
        JsonVal::Array(_) => "array",
        JsonVal::Object(_) => "object",
    }
}

/// Validate this version by checking if we have definition files for this version. If not return version
/// closest to the input version
pub fn validate_version(version: [u16; 4]) -> [u16; 4] {
    let mut smallest_diff = [u16::MAX, u16::MAX, u16::MAX, u16::MAX];
    let mut best_candidate = version;

    for &curr_version in VERSIONS.iter() {
        let diff = [
            version[0].abs_diff(curr_version[0]),
            version[1].abs_diff(curr_version[1]),
            version[2].abs_diff(curr_version[2]),
            version[3].abs_diff(curr_version[3]),
        ];
        if smallest_diff > diff {
            best_candidate = curr_version;
            smallest_diff = diff;
        }
    }

    best_candidate
}
