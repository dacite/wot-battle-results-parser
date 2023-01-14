use std::collections::HashMap;

use itertools::Itertools;
/// Sometimes, a manual parser is needed for some PickleValues when serde is not
/// able to parse it.
use nom::multi::count;
use nom::number::complete::{le_u16, le_u32};
use nom::AsBytes;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use utils::IntoSubValue;
use wot_types::WotValue;

use crate::battle_results::Field;
use crate::error::Error::{self, *};
use crate::{AnyErr, PickleValue};


pub fn pickle_val_to_json_manual(
    pickle_value: PickleValue, field: &Field,
) -> Result<JSONValue, (Error, JSONValue)> {
    // Check the field name to see if there is a manual parser
    // If not, we use the 'catch all' manual parser for the field
    let result = match field.name {
        "accountCompDescr" => parse_account_comp_descr(pickle_value).map_err(AccountCompDescrError),

        "xpReplay" | "creditsReplay" | "freeXPReplay" | "eventCoinReplay" | "goldReplay" | "tmenXPReplay"
        | "bpcoinReplay" | "crystalReplay" => parse_value_replay(pickle_value).map_err(ValueReplayError),

        _ => pickle_to_wotvalue_to_json(pickle_value).map_err(ManualParserError),
    };

    result.map_err(|err| (err, field.default.to_json_value()))
}


// There are some pickle values that are not supported by JSON (PickleValue do
// not implement serde::Serialize). However, most of these are not needed by us
// or can be converted easily. Therefore, we convert pickle value to this
// intermediate type (WotValue) before converting to JSON
fn pickle_to_wotvalue_to_json(pickle: PickleValue) -> Result<JSONValue, AnyErr> {
    let wot_value: WotValue = serde_pickle::from_value(pickle)?;

    let json_value = if let WotValue::Bytes(bytes) = wot_value {
        JSONValue::String(hex::encode_upper(bytes))
    } else {
        serde_json::to_value(wot_value)?
    };


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
fn parse_account_comp_descr(pickle_value: PickleValue) -> Result<JSONValue, AnyErr> {
    use PickleValue::*;
    let format_err = || "format error".into();

    // We expect dict to be BTreeMap<i64, [(i64, Vec<u8>)]>
    let dict = pickle_value.try_dict(format_err)?;

    let mut return_dict = HashMap::new();

    for (key, value) in dict.into_iter() {
        let int_key = key.try_i64(format_err)?;

        // We expect list to be [[i64, Vec<u8>]]. The list should have exactly one
        // element and if it doesn't the parse should fail
        let list_iter = value.try_list(format_err)?.into_iter();


        let Ok(Tuple(mut value)) = list_iter.exactly_one() else { return Err(format_err()) };

        let [I64(id), Bytes(val)] = value.as_mut_slice() else { return Err(format_err()) };

        let account_comp_descr = AccountCompDescr {
            id:  *id,
            val: std::mem::take(val),
        };

        // int_key is converted to string to ensure compatiblity with serde_json
        return_dict.insert(int_key.to_string(), account_comp_descr);
    }

    Ok(serde_json::to_value(return_dict)?)
}


struct NomError;

impl<I> nom::error::ParseError<I> for NomError {
    fn from_error_kind(_: I, _: nom::error::ErrorKind) -> Self {
        NomError
    }

    fn append(_: I, _: nom::error::ErrorKind, _: Self) -> Self {
        NomError
    }
}


// TODO: Ability to add context to low level errors instead of map err all over the place
fn parse_value_replay(wot_value: PickleValue) -> Result<JSONValue, AnyErr> {
    let nom_err = |_: nom::Err<NomError>| "nom error";
    let packed_value = wot_value.try_bytes(|| "expected bytes")?;

    let (packed_value, size) = le_u16(packed_value.as_bytes()).map_err(nom_err)?;

    let (rest, value_list) = count(le_u32, size as usize)(packed_value).map_err(nom_err)?;

    if !rest.is_empty() {
        return Err("Buffer not empty".into());
    }

    Ok(serde_json::to_value(value_list)?)
}
