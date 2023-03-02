use std::collections::BTreeMap;
pub use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use serde_json::Value as JSONValue;
use serde_pickle::{HashableValue, Value as PickleValue};

use crate::{fields::gen_collection, manual_parser::pickle_val_to_json_manual, InterceptFn, Result};
type Dict<T> = HashMap<String, T>;
use crate::{
    battle_results::{Field, FieldType},
    error::Error,
    fields::{matches_version, FieldCollection},
    to_default_if_none, Battle,
};


pub struct DatFileParser {
    /// Identifier manager. Identifier lists can be retrieved with a checksum
    /// value
    fields: FieldCollection,
}

/// The raw data structure from the datfile is not very easy to work with. So we
/// break it down into the following structure
struct DatfileFormat {
    arena_unique_id: String,
    account_self:    Vec<PickleValue>,
    vehicle_self:    HashMap<String, Vec<PickleValue>>,

    common:      Vec<PickleValue>,
    account_all: HashMap<String, Vec<PickleValue>>,
    vehicle_all: HashMap<String, Vec<PickleValue>>,
    player_info: HashMap<String, Vec<PickleValue>>,
}


#[derive(Debug)]
pub enum Intercept {
    Success(&'static Field, serde_json::Value),
    NotPresent(&'static Field, serde_json::Value),
    ManuallyParsed(&'static Field, serde_json::Value),
    Failed(&'static Field, serde_json::Value, String),
}

impl Intercept {
    pub fn original_result(self) -> serde_json::Value {
        use Intercept::*;
        match self {
            Success(_, val) | NotPresent(_, val) | ManuallyParsed(_, val) | Failed(_, val, _) => val,
        }
    }
}

impl DatFileParser {
    /// Parse a datfile into a Battle struct
    pub fn parse(&self, input: &[u8]) -> Result<Battle> {
        // Load the root pickle
        let root_pickle = utils::load_pickle(input).unwrap();

        // Convert the deeply nested root pickle into objects that can be easily parsed
        let datfile_format = parse_root_pickle(root_pickle).unwrap();

        // Parse the pickle objects to make a battle
        self.parse_datfile_format(datfile_format, |intercept, _| intercept.original_result())
    }

    /// Same as `parse` but takes a function that you can use to implement your own parsing code to convert a
    /// pickle value to its JSON counterpart
    pub fn parse_intercept(&self, input: &[u8], intercept: InterceptFn) -> Result<Battle> {
        // Load the root pickle
        let root_pickle = utils::load_pickle(input).unwrap();

        // Convert the deeply nested root pickle into objects that can be easily parsed
        let datfile_format = parse_root_pickle(root_pickle).unwrap();

        // Parse the pickle objects to make a battle
        self.parse_datfile_format(datfile_format, intercept)
    }

    /// Construct a parser. You can then use this parser to parse any number of datfiles
    pub fn new() -> Self {
        Self {
            fields: gen_collection(),
        }
    }

    fn parse_datfile_format(&self, datfile: DatfileFormat, intercept: InterceptFn) -> Result<Battle> {
        use FieldType::*;

        let arena_unique_id = datfile.arena_unique_id;

        let common = pickle_to_json(&self.fields, Common, datfile.common, intercept)?;

        let account_self = pickle_to_json(&self.fields, AccountSelf, datfile.account_self, intercept)?;
        let account_self = HashMap::from([(
            account_self.pointer("/accountDBID").unwrap().to_string(),
            account_self,
        )]);

        let vehicle_self = parse_list(&self.fields, VehicleSelf, datfile.vehicle_self, intercept)?;
        let player_info = parse_list(&self.fields, PlayerInfo, datfile.player_info, intercept)?;
        let account_all = parse_list(&self.fields, AccountAll, datfile.account_all, intercept)?;
        let vehicle_all = parse_list(&self.fields, VehicleAll, datfile.vehicle_all, intercept)?;

        Ok(Battle {
            arena_unique_id,
            common,
            player_info,
            account_all,
            vehicle_all,
            vehicle_self,
            account_self,
        })
    }
}

fn parse_list(
    fields: &FieldCollection, field_type: FieldType, input: Dict<Vec<PickleValue>>, intercept: InterceptFn,
) -> Result<Dict<JSONValue>> {
    input
        .into_iter()
        .map(|(key, value)| {
            pickle_to_json(fields, field_type.clone(), value, intercept).map(|value| (key, value))
        })
        .collect()
}

fn decompress_and_load_pickle(input: &PickleValue) -> Result<PickleValue> {
    let PickleValue::Bytes(input) = input else { return Err(Error::PickleFormatError) };
    let decompressed =
        miniz_oxide::inflate::decompress_to_vec_zlib(input).map_err(|_| Error::DecompressionError)?;

    Ok(serde_pickle::value_from_slice(&decompressed, Default::default())?)
}

fn parse_root_pickle(root_pickle: PickleValue) -> Result<DatfileFormat> {
    use PickleValue::*;
    // root pickle is a tuple of the shape : (i64, Tuple)
    let Tuple(root_tuple) = root_pickle else { return Err(Error::PickleFormatError) };

    // data tuple should contain the following: (arenaUniqueID, [u8], [u8], [u8])
    // the three u8 buffers in this tuple are compressed pickle dumps
    let [_, Tuple(data_tuple)] = root_tuple.as_slice() else { return Err(Error::PickleFormatError) };

    let [I64(arena_unique_id), rest @ ..] = data_tuple.as_slice() else {
        return Err(Error::PickleFormatError)
    };

    let Some((List(account_self), Dict(vehicle_self), Tuple(multiple))) = rest.into_iter().map(decompress_and_load_pickle).flatten().next_tuple() else {
        return Err(Error::PickleFormatError)
    };

    let Some((List(common), Dict(player_info), Dict(vehicle_all), Dict(account_all))) = multiple.into_iter().next_tuple() else {
        return Err(Error::PickleFormatError)
    };

    Ok(DatfileFormat {
        arena_unique_id: arena_unique_id.to_string(),
        account_self,
        common,
        account_all: to_rust_dict(account_all)?,
        vehicle_all: to_rust_dict(vehicle_all)?,
        player_info: to_rust_dict(player_info)?,
        vehicle_self: to_rust_dict(vehicle_self)?,
    })
}


fn pickle_to_json(
    fields: &FieldCollection, field_type: FieldType, value_list: Vec<PickleValue>, intercept: InterceptFn,
) -> Result<JSONValue> {
    let mut value_list = value_list.into_iter();

    // The checksum describes the list of identifiers that are associated with that list of PickleValue.
    // This prevents us from blindly assigning, for example `damageDealt` identifier to
    // `PickleValue::I64(5433)` because `5433` looks like a `damageDealt` value. With checksum we
    // can know for sure.
    let Some(PickleValue::I64(checksum)) = value_list.next() else {
        return Err(Error::OtherError("Value list is empty"))
    };

    // If we cannot find the correct the identifier list, we cannot parse the
    // datfile so we return with error
    let (iden_list, version) = fields
        .get_fields_list(checksum)
        .ok_or_else(|| Error::UnknownChecksum(field_type.to_str(), checksum))?;

    let mut map = HashMap::new();
    for iden in iden_list {
        if !matches_version(version, iden) {
            let value = intercept(
                Intercept::NotPresent(iden, iden.default.to_json_value()),
                PickleValue::None,
            );

            map.insert(iden.name, value);
        } else {
            let value = value_list.next().ok_or_else(|| Error::DecompressionError)?;

            map.insert(iden.name, pickle_val_to_json(iden, value, intercept));
        }
    }

    assert!(value_list.next().is_none());
    Ok(serde_json::to_value(map).unwrap())
}


/// Convert a `PickleValue` that contains a field value(for ex. field value
/// of `damageDealt` is of type `i32`) to JSON. Note that even if the
/// parsing fails we get a JSON because it will be the default value for
/// the field We make the distinction between `Ok` and `Err` based on
/// whether the field value was parsed succesfully to JSON
fn pickle_val_to_json(iden: &'static Field, input: PickleValue, intercept: InterceptFn) -> JSONValue {
    let value = to_default_if_none(iden, input);

    match serde_pickle::from_value(value.clone()) {
        Ok(json_value) => intercept(Intercept::Success(iden, json_value), value),

        // Simple parsing did not work so we delegate to the more
        // powerful manual parser
        Err(_) => match pickle_val_to_json_manual(value.clone(), iden) {
            Ok(json_value) => intercept(Intercept::ManuallyParsed(iden, json_value), value),
            Err((err, json_value)) => intercept(Intercept::Failed(iden, json_value, err.to_string()), value),
        },
    }
}


fn to_rust_dict(input: BTreeMap<HashableValue, PickleValue>) -> Result<Dict<Vec<PickleValue>>> {
    input
        .into_iter()
        .map(|(key, value)| match value {
            PickleValue::List(list) => Ok((key.to_string(), list)),
            PickleValue::Dict(dict) => {
                let mut dict_iter = dict.into_iter();
                let Some((inner_key, PickleValue::List(value))) = dict_iter.next() else { return Err(Error::PickleFormatError) };

                Ok((format!("{} {}", key, inner_key), value))
            }
            _ => Err(Error::DecompressionError.into()),
        })
        .collect()
}
