mod battle_results;
pub mod error;
mod fields;
mod manual_parser;
mod parser;

use std::collections::HashMap;

use battle_results::Field;
use error::Error;
pub use parser::DatFileParser;
pub use parser::Intercept;
use serde::Deserialize;
use serde::Serialize;
pub use serde_pickle::HashableValue as HashablePickleValue;
pub use serde_pickle::Value as PickleValue;

type Result<T> = core::result::Result<T, Error>;

/// Intercept Function that allows you to manipulate how a particular value is parsed
pub type InterceptFn = fn(Intercept, serde_pickle::Value) -> serde_json::Value;

pub(crate) type AnyErr = Box<dyn std::error::Error + Send + Sync + 'static>;
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
