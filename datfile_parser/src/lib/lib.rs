pub mod error;
mod fields;
mod manual_parser;
mod utils;

use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use fields::{gen_collection, FieldCollection};

use serde_json::Value as JSONValue;
use standard_format::{AccountAll, AccountSelf, Battle, Common, PlayerInfo, VehicleAll, VehicleSelf};
use unpickler::{HashablePickleValue, PickleValue};
use utils::try_variant;
use wot_constants::battle_results::{Field};

use crate::fields::matches_version;

type MixedResult = (
    Common,
    HashMap<String, PlayerInfo>,
    HashMap<String, VehicleAll>,
    HashMap<String, AccountAll>,
);

pub struct DatFileParser {
    collections: FieldCollection,
}
impl Default for DatFileParser {
    fn default() -> Self {
        Self::new()
    }
}
impl DatFileParser {
    pub fn new() -> Self {
        Self {
            collections: gen_collection(),
        }
    }

    pub fn parse(&self, input: &[u8]) -> Result<Battle> {
        // Load the root pickle
        let root_pickle = unpickler::load_pickle(input)?;

        // root pickle is a tuple of the shape : (i64, Tuple)
        let root_tuple = unpickler::access_tuple(&root_pickle)?;

        // root tuple should contain the following: (arenaUniqueID, [u8], [u8], [u8])
        // the three u8 buffers (named buffer1, buffer2, buffer3 respectively) in this
        // tuple are compressed pickle dumps
        let data_tuple = unpickler::access_tuple(&root_tuple[1])?;

        let arena_unique_id = unpickler::access_i64(&data_tuple[0])?.to_string();
        let account_self_pickle = unpickler::decompress_and_load_pickle(&data_tuple[1])?;
        let vehicle_self_pickle = unpickler::decompress_and_load_pickle(&data_tuple[2])?;
        let mixed_pickle = unpickler::decompress_and_load_pickle(&data_tuple[3])?;

        // Once we get the pickle dumps, we parse them separately:

        // Pickle dump @0 is AccountSelf of the "recording" player. We do not have
        // AccountSelf of other players unless we get their dat file
        let account_self: AccountSelf = serde_json::from_value(
            self.value_list_to_json(unpickler::access_list(&account_self_pickle)?)
                .unwrap(),
        )
        .unwrap();

        // Pickle dump@1 is a dict with one element. The element has a key that refers
        // to "recording" player's tank_id. We can discard it because it appears
        // again inside the value pointed to by the key. The Value is
        // VehicleSelf. The nature of AccountSelf(See above) applies to this
        // structure as well.
        let dict = unpickler::access_dict(&vehicle_self_pickle)?;
        let item = dict.into_iter().next().context("Vehicle Self parse failed")?;
        let (_tank_id, vehicle_self_list) = self.extract_from_item(item)?;
        let vehicle_self: VehicleSelf =
            serde_json::from_value(self.value_list_to_json(vehicle_self_list)?)?;

        // Pickle dump@2 contains the following:
        // common attributes of the battle
        // player_info of all players
        // account_all of all players
        // vehicle_all of all players
        let (common, player_info, vehicle_all, account_all) = self.parse_mixed_list(&mixed_pickle)?;

        // Make battle

        let mut vehicle_self_list = HashMap::new();
        let mut account_self_list = HashMap::new();

        vehicle_self_list.insert(account_self.get_account_dbid().to_string(), vehicle_self);
        account_self_list.insert(account_self.get_account_dbid().to_string(), account_self);

        Ok(Battle {
            arena_unique_id,
            common,
            player_info,
            account_all,
            vehicle_all,
            vehicle_self: vehicle_self_list,
            account_self: account_self_list,
        })
    }

    fn parse_mixed_list(&self, wrapped_list3: &PickleValue) -> Result<MixedResult> {
        let mut tuple = unpickler::access_tuple(wrapped_list3)?.into_iter();

        let common: Common =
            serde_json::from_value(self.value_list_to_json(try_variant!(tuple.next().unwrap(), PickleValue::List)?)?)
                .with_context(|| anyhow!("Common parse failed."))?;
        let player_info_list = serde_json::from_value(self.pickle_to_json(tuple.next().unwrap())?)?;
        let vehicle_all_list = serde_json::from_value(self.pickle_to_json(tuple.next().unwrap())?)?;
        let account_info_list = serde_json::from_value(self.pickle_to_json(tuple.next().unwrap())?)?;

        Ok((common, player_info_list, vehicle_all_list, account_info_list))
    }


    fn pickle_to_json(&self, input: PickleValue) -> Result<JSONValue> {
        let map = pickle_dict_to_hashmap(input)?;
        
        let mut json_map = serde_json::Map::new();
        for (key, value) in map.into_iter() {
            json_map.insert(key, self.value_list_to_json(value)?);
        }

        Ok(JSONValue::Object(json_map))
    }

    fn value_list_to_json(&self, value_list: Vec<PickleValue>) -> Result<JSONValue> {
        // Get checksum so that we can get the correct list of identifiers for the value
        // list
        let checksum = get_checksum(&value_list)?;

        // If we cannot find the correct the identifier list, we cannot parse the
        // datfile so we return with error
        let (iden_list, version) = self.collections.get_fields_list(checksum).ok_or(anyhow!(
            "Value list has unrecognized checksum({}). Format won't match",
            checksum
        ))?;

        // Zip the identifier list and the value list (we skip value_list[0] because it
        // is the checksum) and fill the map
        let mut map = HashMap::new();

        let mut value_list_iter = value_list.into_iter().skip(1);

        for iden in iden_list.iter() {
            if !matches_version(version, iden) {
                log::info!(
                    "dat file(version: {}) do not have {}(version: {}, max_version: {}",
                    version,
                    iden.name,
                    iden.version,
                    iden.max_version
                );
                map.insert(iden.name, iden.default.to_json_value());
                continue;
            }
            if let Some(value) = value_list_iter.next() {
                map.insert(iden.name, parse_wotvalue_pickle_to_json(iden, value));
            } else {
                panic!("UNEXPECTED");
            }
        }

        Ok(serde_json::to_value(map)?)
    }

    
    /// Return the following when given a hashmap item:
    /// - `key` This is either the account_dbid or the avatar_id
    /// - `value_list` This finally should be a Vec but might have to parsed
    ///   from either a dict or a list
    fn extract_from_item(&self, item: (HashablePickleValue, PickleValue)) -> Result<(String, Vec<PickleValue>)> {
        let key = item.0.to_string();
        let value_list;

        // Item can either be a list or a dict
        // If dict we need to get the vec that is value of the first item in the dict
        match item.1 {
            PickleValue::List(list) => value_list = list,
            PickleValue::Dict(map) => value_list = unpickler::access_list(&map.into_iter().next().unwrap().1)?,
            _ => return Err(anyhow!("Value in (key,value) pair should be a list or dict")),
        }

        Ok((key, value_list))
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

fn parse_to_pickle_val_list(input: PickleValue) -> Result<Vec<PickleValue>> {
    // Item can either be a list or a dict
    // If dict we need to get the vec that is value of the first item in the dict
    match input {
        PickleValue::List(val_list) => Ok(val_list),
        PickleValue::Dict(map) => {
            let (_key, nested_value) = map
                .into_iter()
                .next()
                .context("Expected map to have atleast one (key, value)")?;
            let list = try_variant!(nested_value, PickleValue::List)?;

            Ok(list)
        }
        _ => return Err(anyhow!("Value in (key,value) pair should be a list or dict")),
    }
}

/// Convert a `PickleValue` that contains a field value(for ex. field value of
/// `damageDealt` is of type `i32`) to JSON. Note that even if the parsing fails
/// we get a JSON because it will be the default value for the field We make the
/// distinction between `Ok` and `Err` based on whether the field value was
/// parsed succesfully to JSON
fn parse_wotvalue_pickle_to_json(identifier: &Field, input: PickleValue) -> JSONValue {
    let value = to_default_if_none(identifier, input);

    match serde_pickle::from_value(value.clone()) {
        Ok(json_value) => json_value,

        // Simple parsing did not work so we delegate to the more
        // powerful manual parser
        Err(_) => {
            manual_parser::pickle_val_to_json(value, &identifier).unwrap_or_else(|e| {
                // If manual parser was not able to get the job done, we log the problem and
                // return a default value
                log::warn!("Could not parse {}. {}", identifier.name, e.to_string());

                identifier.default.to_json_value()
            })
        }
    }
}

/// Try to convert a `PickleValue::Dict` to `HashMap<String, PickleValue>`.
    /// Solves parsing the structure of the following field types (and only
    /// them) `AccountAll`, `PlayerInfo`, `VehicleAll`
    fn pickle_dict_to_hashmap(input: PickleValue) -> Result<HashMap<String, Vec<PickleValue>>> {
        if let PickleValue::Dict(dict) = input {
            let mut map = HashMap::new();
            for (key, value) in dict.into_iter() {
                let key = key.to_string();
                let value = parse_to_pickle_val_list(value)?;

                map.insert(key, value);
            }

            Ok(map)
        } else {
            Err(anyhow!("Expected a pickle dictionary"))
        }
    }
