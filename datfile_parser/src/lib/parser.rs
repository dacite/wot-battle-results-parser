use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::{anyhow, ensure, Context, Result};
use serde::de::DeserializeOwned;
use serde_json::from_value as from_json_value;
use serde_json::Value as JSONValue;
use serde_pickle::Value as PickleValue;
use standard_format::{AccountSelf, ArenaFieldsGetter, Battle};
use unpickler::decompress_and_load_pickle;
use wot_constants::battle_results::Field;

use crate::{
    fields::{matches_version, FieldCollection},
    get_checksum, manual_parser, to_default_if_none,
    utils::try_variant,
};

/// An instantiation of a `Parser` is used to parse a single `.dat` file
pub struct Parser<'a> {
    /// Result of the parsing
    result: Option<Battle>,

    /// Identifier manager. Identifier lists can be retrieved with a checksum
    /// value
    fields: Rc<&'a FieldCollection>,

    /// Fields that were not present for this particular datfile
    not_present: HashSet<String>,

    /// Fields that were not parsed automatically with serde
    manually_parsed: HashSet<String>,

    /// Fields that were unable to parse even manually
    failed: HashSet<String>,
}

/// The raw data structure from the datfile is not very easy to work with. So we
/// break down into the following structure
struct DatfileFormat {
    arena_unique_id: String,
    account_self:    Vec<PickleValue>,
    vehicle_self:    HashMap<String, Vec<PickleValue>>,

    common:      Vec<PickleValue>,
    account_all: HashMap<String, Vec<PickleValue>>,
    vehicle_all: HashMap<String, Vec<PickleValue>>,
    player_info: HashMap<String, Vec<PickleValue>>,
}

/// A container to hold some nested output objects(ex: AccountAll, VehicleAll
/// etc.)
struct ObjectList {
    common:      Vec<PickleValue>,
    account_all: HashMap<String, Vec<PickleValue>>,
    vehicle_all: HashMap<String, Vec<PickleValue>>,
    player_info: HashMap<String, Vec<PickleValue>>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self, input: &[u8]) -> Result<()> {
        // Load the root pickle
        let root_pickle = unpickler::load_pickle(input)?;

        // Convert the deeply nested root pickle into objects that can be easily parsed
        let datfile_format = parse_root_pickle(root_pickle)?;

        // Parse the pickle objects to make a battle
        match self.parse_datfile_format(datfile_format) {
            Ok(battle) => {
                self.result = Some(battle);

                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn new(field_collection: &'a FieldCollection) -> Self {
        Self {
            result: None,
            fields: Rc::new(field_collection),

            not_present:     HashSet::new(),
            manually_parsed: HashSet::new(),
            failed:          HashSet::new(),
        }
    }

    pub fn into_battle(self) -> Battle {
        self.result.unwrap()
    }

    fn parse_datfile_format(&mut self, datfile_format: DatfileFormat) -> Result<Battle> {
        let arena_unique_id = datfile_format.arena_unique_id;

        let common = self.pickle_list_to_output_object(datfile_format.common)?;

        let account_self: AccountSelf = self.pickle_list_to_output_object(datfile_format.account_self.clone())?;
        let account_self = HashMap::from([(account_self.get_account_dbid().to_string(), account_self)]);

        let vehicle_self = self.parse_list(datfile_format.vehicle_self)?;
        let player_info = self.parse_list(datfile_format.player_info)?;
        let account_all = self.parse_list(datfile_format.account_all)?;
        let vehicle_all = self.parse_list(datfile_format.vehicle_all)?;

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

    fn parse_list<T>(&mut self, input: HashMap<String, Vec<PickleValue>>) -> Result<HashMap<String, T>>
    where
        T: DeserializeOwned + ArenaFieldsGetter,
    {
        let mut output = HashMap::new();
        for (key, value) in input.into_iter() {
            output.insert(key, self.pickle_list_to_output_object(value)?);
        }

        Ok(output)
    }

    fn pickle_list_to_output_object<T>(&mut self, input: Vec<PickleValue>) -> Result<T>
    where
        T: DeserializeOwned + ArenaFieldsGetter,
    {
        let json = self.pickle_list_to_json_object(input)?;
        let output: T = from_json_value(json)?;

        // At this point, there may be some fields that were not present in the output object (`T`). This is parsed as
        // arena fields because these fields are only common to a specific gamemode. Nevertheless, we try to
        // serialize these fields to one of the `extra` enums to make sure it is indeed the gamemode specific
        // fields If there are fields that are clearly not part of a specific gamemode then that's an error and
        // should be parsed as one of the member of the `T`
        output.validate_arena_fields().map_err(|err| {
            anyhow!(
                "unknown fields found. standard format might be out of date. {}",
                err.to_string()
            )
        })?;
        Ok(output)
    }

    fn pickle_list_to_json_object(&mut self, value_list: Vec<PickleValue>) -> Result<JSONValue> {
        // checksum is used to find the correct list of identifiers for the `value_list`
        let checksum = get_checksum(&value_list)?;

        // If we cannot find the correct the identifier list, we cannot parse the
        // datfile so we return with error
        let (iden_list, version) = self.fields.get_fields_list(checksum).ok_or(anyhow!(
            "Value list has unrecognized checksum({}). Identifier list won't match",
            checksum
        ))?;


        // We skip the first element of the `value_list` because it is the checksum
        let mut value_list_iter = value_list.into_iter().skip(1);

        let mut map = HashMap::new();
        for iden in iden_list.iter() {
            // Identifer list is always the latest version, but the datfile itself might be
            // a bit older so we need to insert a default value for that missing identifier
            if !matches_version(version, iden) {
                self.not_present.insert(iden.name.to_owned());

                map.insert(iden.name, iden.default.to_json_value());
                continue;
            }

            if let Some(value) = value_list_iter.next() {
                map.insert(iden.name, self.pickle_val_to_json(iden, value));
            } else {
                // If this case happens, it will be really nasty bug (involving checksums).
                return Err(anyhow!("value list exhausted before populating all fields"));
            }
        }

        ensure!(
            value_list_iter.next().is_none(),
            "value list not empty after populating fields"
        );
        Ok(serde_json::to_value(map)?)
    }

    /// Convert a `PickleValue` that contains a field value(for ex. field value
    /// of `damageDealt` is of type `i32`) to JSON. Note that even if the
    /// parsing fails we get a JSON because it will be the default value for
    /// the field We make the distinction between `Ok` and `Err` based on
    /// whether the field value was parsed succesfully to JSON
    fn pickle_val_to_json(&mut self, identifier: &Field, input: PickleValue) -> JSONValue {
        let value = to_default_if_none(identifier, input);

        match serde_pickle::from_value(value.clone()) {
            Ok(json_value) => json_value,

            // Simple parsing did not work so we delegate to the more
            // powerful manual parser
            Err(_) => {
                self.manually_parsed.insert(identifier.name.to_owned());

                manual_parser::pickle_val_to_json_manual(value, &identifier).unwrap_or_else(|e| {
                    // If manual parser was not able to get the job done, we log the problem and
                    // return a default value
                    log::warn!("Could not parse {}. {}", identifier.name, e.to_string());
                    self.failed.insert(identifier.name.to_owned());
                    identifier.default.to_json_value()
                })
            }
        }
    }
}

fn parse_root_pickle(root_pickle: PickleValue) -> Result<DatfileFormat> {
    // root pickle is a tuple of the shape : (i64, Tuple)
    let mut root_tuple = try_variant!(root_pickle, PickleValue::Tuple)?;

    // data tuple should contain the following: (arenaUniqueID, [u8], [u8], [u8])
    // the three u8 buffers in this tuple are compressed pickle dumps
    let data_tuple = try_variant!(root_tuple.remove(1), PickleValue::Tuple)?;
    let mut data_tuple = data_tuple.into_iter();

    let arena_unique_id = data_tuple.next().context("unexpected pickle format")?;
    let arena_unique_id = try_variant!(arena_unique_id, PickleValue::I64)?.to_string();

    let account_self = decompress_and_load_pickle(&data_tuple.next().context("unexpected pickle format")?)?;
    let account_self = try_variant!(account_self, PickleValue::List)?;

    let vehicle_self = decompress_and_load_pickle(&data_tuple.next().context("unexpected pickle format")?)?;
    let vehicle_self = parse_nested(vehicle_self)?;

    let multiple = decompress_and_load_pickle(&data_tuple.next().context("unexpected pickle format")?)?;
    let ObjectList {
        common,
        account_all,
        vehicle_all,
        player_info,
    } = parse_multiple_pickle(multiple)?;

    Ok(DatfileFormat {
        arena_unique_id,
        account_self,
        vehicle_self,
        common,
        account_all,
        vehicle_all,
        player_info,
    })
}

fn parse_nested(input: PickleValue) -> Result<HashMap<String, Vec<PickleValue>>> {
    let dict = try_variant!(input, PickleValue::Dict)?;

    let mut output_map = HashMap::new();
    for (key, value) in dict.into_iter() {
        let output_value = try_variant!(value, PickleValue::List)?;

        // TODO: Remove this panic
        if output_map.insert(key.to_string(), output_value).is_some() {
            panic!("Vehicle Self with same key not supported");
        }
    }

    Ok(output_map)
}

fn to_rust_dict(pickle_object: PickleValue) -> Result<HashMap<String, Vec<PickleValue>>> {
    let input_dict = try_variant!(pickle_object, PickleValue::Dict)?;

    input_dict
        .into_iter()
        .map(|(key, value)| match value {
            PickleValue::List(list) => Ok((key.to_string(), list)),
            PickleValue::Dict(dict) => {
                let mut dict_iter = dict.into_iter();
                let (inner_key, value) = dict_iter.next().context("dict was empty")?;
                ensure!(
                    dict_iter.next().is_none(),
                    "values that are dictionaries may only have one (key, value)"
                );

                Ok((
                    format!("{} {}", key.to_string(), inner_key.to_string()),
                    try_variant!(value, PickleValue::List)?,
                ))
            }
            _ => return Err(anyhow!("to rust map found unexpected pickle object")),
        })
        .collect()
}

fn parse_multiple_pickle(multiple: PickleValue) -> Result<ObjectList> {
    let mut tuple = try_variant!(multiple, PickleValue::Tuple)?.into_iter();

    let common = try_variant!(tuple.next().unwrap(), PickleValue::List)?;
    let player_info = to_rust_dict(tuple.next().unwrap())?;
    let vehicle_all = to_rust_dict(tuple.next().unwrap())?;
    let account_all = to_rust_dict(tuple.next().unwrap())?;

    Ok(ObjectList {
        common,
        account_all,
        vehicle_all,
        player_info,
    })
}
