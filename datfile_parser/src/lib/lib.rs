mod battle_results;
pub mod error;
mod fields;
mod manual_parser;
mod parser;

use std::collections::HashMap;

use anyhow::{Context, Result};
use battle_results::Field;
use fields::{gen_collection, FieldCollection};
use parser::Parser;
use serde::Deserialize;
use serde::Serialize;
pub use serde_pickle::HashableValue as HashablePickleValue;
pub use serde_pickle::Value as PickleValue;

pub struct DatFileParser {
    collections: FieldCollection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Battle {
    #[serde(rename(serialize = "arenaUniqueID"))]
    pub arena_unique_id: String,
    pub common:          serde_json::Value,
    pub player_info:     HashMap<String, serde_json::Value>,
    pub account_all:     HashMap<String, serde_json::Value>,
    pub vehicle_all:     HashMap<String, serde_json::Value>,
    pub vehicle_self:    HashMap<String, serde_json::Value>,
    pub account_self:    HashMap<String, serde_json::Value>,
}

impl DatFileParser {
    pub fn new() -> Self {
        Self {
            collections: gen_collection(),
        }
    }

    pub fn parse(&self, input: &[u8]) -> Result<Battle> {
        let mut parser = Parser::new(&self.collections);

        parser.parse(input)?;
        parser.print_parse_summary();
        Ok(parser.into_battle())
    }
}

impl Default for DatFileParser {
    fn default() -> Self {
        Self::new()
    }
}

/// The checksum describes the list of identifiers that are associated with that list of PickleValue. This
/// prevents us from blindly assigning, for example `damageDealt` identifier to `PickleValue::I64(5433)`
/// because `5433` looks like a `damageDealt` value. With checksum we can know for sure.
fn get_checksum(data_list: &[PickleValue]) -> Result<i32> {
    let PickleValue::I64(checksum) = data_list[0] else {
        return Err(anyhow::anyhow!("expected checksum to be an integer like value"))
    };

    i32::try_from(checksum).context("checksum conversion error")
}

/// `.dat` files pickles usually contain null values instead of the default
/// value. Therefore, we replace it with the default value.
/// For ex. `has_battle_pass = PickleValue::None` is replaced with
/// `has_battle_pass = PickleValue::Bool(false)`.
/// TODO: Explore making this like `#[serde(default)]`
fn to_default_if_none(identifier: &Field, value: PickleValue) -> PickleValue {
    if value == PickleValue::None {
        // Identifier knows what is the default value
        identifier.default.to_pickle_value()
    } else {
        value
    }
}

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
