pub mod error;
mod fields;
mod manual_parser;
mod parser;
pub mod utils;

use anyhow::{Context, Result};
use fields::{gen_collection, FieldCollection};
use parser::Parser;
use standard_format::Battle;
use unpickler::PickleValue;
use wot_constants::battle_results::Field;


pub struct DatFileParser {
    collections: FieldCollection,
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

        Ok(parser.into_battle())
    }
}

/// Given a vec of values (parsed from the dat file), the first element is the
/// checksum
fn get_checksum(data_list: &[PickleValue]) -> Result<i32> {
    let checksum = unpickler::access_i64(&data_list[0])?;

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
