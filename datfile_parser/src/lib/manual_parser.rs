use std::collections::HashMap;

/// Sometimes, a manual parser is needed for some PickleValues when serde is not
/// able to parse it.
use anyhow::{anyhow, ensure, Context, Result};
use nom::multi::count;
use nom::number::complete::{le_u16, le_u32};
use nom::{AsBytes, Finish};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use standard_format::WotValue;
use unpickler::HashablePickleValue;
use unpickler::PickleValue;
use wot_constants::battle_results::Field;

use crate::utils::try_variant;

pub fn pickle_val_to_json_manual(pickle_value: PickleValue, field: &Field) -> Result<JSONValue> {
    // Check the field name to see if there is a manual parser
    // If not, we use the 'catch all' manual parser for the field
    match field.name {
        "accountCompDescr" => parse_account_comp_descr(pickle_value),

        "xpReplay" | "creditsReplay" | "freeXPReplay" | "eventCoinReplay" | "goldReplay" | "tmenXPReplay"
        | "bpcoinReplay" | "crystalReplay" => parse_value_replay(pickle_value),

        _ => pickle_to_wotvalue_to_json(pickle_value),
    }
}

// There are some pickle values that are not supported by JSON (PickleValue do
// not implement serde::Serialize). However, most of these are not needed by us
// or can be converted easily. Therefore, we convert pickle value to this
// intermediate type (WotValue) before converting to JSON
fn pickle_to_wotvalue_to_json(pickle: PickleValue) -> Result<JSONValue> {
    let wot_value: WotValue = serde_pickle::from_value(pickle.clone()).map_err(|e| {
        return anyhow!("{}", e.to_string());
    })?;

    let json_value;

    if let WotValue::Bytes(bytes) = wot_value {
        json_value = JSONValue::String(hex::encode_upper(bytes));
    } else {
        json_value = serde_json::to_value(wot_value)?;
    }

    Ok(json_value)
}

///////////////////////////////////////////////////////////////////////////////////////////////////////


#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
struct AccountCompDescr {
    id:  i64,
    #[serde_as(as = "serde_with::hex::Hex<serde_with::formats::Uppercase>")]
    val: Vec<u8>,
}

/// accountCompDescr is expect to have the following format in PickleValue:
/// `BTreeMap<i64, [[i64, Vec<u8>]]`. I cannot find any easy way to make this
/// work with serde, so here is a custom parser for it that turns it
/// into a `HashMap<String, AccountCompDescr>`
fn parse_account_comp_descr(pickle_value: PickleValue) -> Result<JSONValue> {
    // We expect dict to be BTreeMap<i64, [(i64, Vec<u8>)]>
    let dict = try_variant!(pickle_value, PickleValue::Dict)?;

    let mut return_dict = HashMap::new();

    for (key, value) in dict.into_iter() {
        let int_key = try_variant!(key, HashablePickleValue::I64)?;

        // We expect list to be [[i64, Vec<u8>]]. The list should have exactly one
        // element and if it doesn't the parse should fail
        let list = try_variant!(value, PickleValue::List)?;
        let list_value = list.into_iter().next().context("AccountCompDescr parse failed")?;

        // We expect tuple to be [i64, Vec<u8>]. tuple[0] is the i64 and tuple[1] is the
        // Vec<u8>
        let mut tuple = try_variant!(list_value, PickleValue::Tuple)?.into_iter();
        let id = tuple.next().context("AccountCompDescr parse failed: expected id")?;
        let val = tuple.next().context("AccountCompDescr parse failed: expected val")?;

        let account_comp_descr = AccountCompDescr {
            id:  try_variant!(id, PickleValue::I64)?,
            val: try_variant!(val, PickleValue::Bytes)?,
        };

        // int_key is converted to string to ensure compatiblity with serde_json
        return_dict.insert(int_key.to_string(), account_comp_descr);
    }

    serde_json::to_value(return_dict).with_context(|| {
        anyhow!("Conversion to JSON failed after successfully serializing accountCompDescr from PickleValue")
    })
}

fn parse_value_replay(wot_value: PickleValue) -> Result<JSONValue> {
    let packed_value = try_variant!(wot_value, PickleValue::Bytes)?;
    let (packed_value, size) = le_u16::<_, crate::error::NomErrorWrapper>(packed_value.as_bytes())?;

    let (rest, value_list) =
        count::<_, _, crate::error::NomErrorWrapper, _>(le_u32, size as usize)(packed_value.as_bytes()).finish()?;
    ensure!(rest.is_empty(), "Expected empty rest after parsing value replay");
    Ok(serde_json::to_value(value_list)?)
}
