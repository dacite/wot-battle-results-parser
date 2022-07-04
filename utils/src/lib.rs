use std::{
    fs::{self, DirEntry},
    path::Path,
};

use anyhow::{Context, Result};
use serde::Deserializer;

/// A macro that tries to destructure an Enum to the given variant,
/// wrapped in a `Result`. Used to avoid using if let everywhere and have the
/// entire code shift right. Once if let chains stablize, this is probably not
/// needed.
#[macro_export]
macro_rules! try_variant {
    ($target: expr, $pattern: path) => {{
        if let $pattern(value) = $target {
            Ok(value)
        } else {
            Err(anyhow::anyhow!(
                "Wrong variant. Expected: {}",
                stringify!($pattern)
            ))
        }
    }};
}

/// Get files in directory, given directory path (only direct childs of the directory)
pub fn parse_dir<P: AsRef<Path>>(path: P) -> Result<Vec<DirEntry>> {
    let file_paths = fs::read_dir(path).with_context(|| "failed to read dir")?;

    Ok(file_paths
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .collect())
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum BoolableValue {
    Bool(bool),
    Int(i32),
}

/// WoT sometimes uses int and boolean interchangeably for the same field and we can't have that
pub fn bool_to_int<'de, D>(de: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let val: BoolableValue = serde::de::Deserialize::deserialize(de)?;

    match val {
        BoolableValue::Bool(val) => Ok(val as i32),
        BoolableValue::Int(val) => Ok(val),
    }
}
