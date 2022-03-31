mod battle_results;
use anyhow::{anyhow, Context, Error, Result};
use wot_constants::battle_results::{FieldType, Field};
use std::collections::HashMap;
use unpickler::{HashablePickleValue, PickleValue};

use standard_format::{AccountAll, AccountSelf, Battle, Common, FieldAccess, PlayerInfo, VehicleAll, VehicleSelf,};
use battle_results::BattleResultsManager;

type MixedResult = (
    Common,
    HashMap<String, PlayerInfo>,
    HashMap<String, VehicleAll>,
    HashMap<String, AccountAll>,
);

pub struct DatFileParser {
    battle_results: BattleResultsManager,
}
impl Default for DatFileParser {
    fn default() -> Self {
        Self::new()
    }
}
impl DatFileParser {
    pub fn new() -> Self {
        Self {
            battle_results: BattleResultsManager::new(),
        }
    }

    pub fn parse(&self, input: &[u8]) -> Result<Battle> {
        // Load the root pickle
        let root_pickle = unpickler::load_pickle(input)?;

        // root pickle is a tuple of the shape : (i64, Tuple)
        let root_tuple = unpickler::access_tuple(&root_pickle)?;

        // root tuple should contain the following: (arenaUniqueID, [u8], [u8], [u8])
        // the three u8 buffers (named buffer1, buffer2, buffer3 respectively) in this tuple are
        // compressed pickle dumps
        let data_tuple = unpickler::access_tuple(&root_tuple[1])?;

        let arena_unique_id = unpickler::access_i64(&data_tuple[0])?.to_string();
        let account_self_pickle = unpickler::decompress_and_load_pickle(&data_tuple[1])?;
        let vehicle_self_pickle = unpickler::decompress_and_load_pickle(&data_tuple[2])?;
        let mixed_pickle = unpickler::decompress_and_load_pickle(&data_tuple[3])?;

        // Once we get the pickle dumps, we parse them separately:

        // Pickle dump @0 is AccountSelf of the "recording" player. We do not have AccountSelf of
        // other players unless we get their dat file
        let account_self: AccountSelf = self.parse_collection(
            unpickler::access_list(&account_self_pickle)?,
            FieldType::AccountSelf,
        )?;

        // Pickle dump@1 is a dict with one element. The element has a key that refers to "recording"
        // player's tank_id. We can discard it because it appears again inside the value pointed to
        // by the key. The Value is VehicleSelf. The nature of AccountSelf(See above) applies to this
        // structure as well.
        let dict = unpickler::access_dict(&vehicle_self_pickle)?;
        let item = dict
            .into_iter()
            .next()
            .context("Vehicle Self parse failed")?;
        let (_tank_id, vehicle_self_list) = self.extract_from_item(item)?;
        let vehicle_self: VehicleSelf =
            self.parse_collection(vehicle_self_list, FieldType::VehicleSelf)?;

        // Pickle dump@2 contains the following:
        // common attributes of the battle
        // player_info of all players
        // account_all of all players
        // vehicle_all of all players
        let (common, player_info, vehicle_all, account_all) = self.parse_mixed_list(&mixed_pickle).unwrap();

        // Make battle
        return if let Some(serde_json::Value::Number(account_dbid)) =
            account_self.get("account_dbid")
        {
            let mut vehicle_self_list = HashMap::new();
            let mut account_self_list = HashMap::new();

            vehicle_self_list.insert(account_dbid.to_string(), vehicle_self);
            account_self_list.insert(account_dbid.to_string(), account_self);

            Ok(Battle {
                arena_unique_id,
                common,
                player_info,
                account_all,
                vehicle_all,
                vehicle_self: vehicle_self_list,
                account_self: account_self_list,
            })
        } else {
            Err(anyhow!("unable to get account_dbid"))
        };
    }

    fn parse_mixed_list(&self, wrapped_list3: &PickleValue) -> Result<MixedResult> {
        let tuple = unpickler::access_tuple(wrapped_list3)?;

        let common: Common =
            self.parse_collection(unpickler::access_list(&tuple[0])?, FieldType::Common)?;
        let player_info_list = self.parse_player_info_list(&tuple[1])?;
        let vehicle_all_list = self.parse_vehicle_all_list(&tuple[2])?;
        let account_info_list = self.parse_all_account_info(&tuple[3])?;

        Ok((
            common,
            player_info_list,
            vehicle_all_list,
            account_info_list,
        ))
    }

    fn parse_collection<T: FieldAccess + Default>(
        &self,
        value_list: Vec<PickleValue>,
        field_type: FieldType,
    ) -> Result<T, Error> {
        let checksum = get_checksum(&value_list)?;

        let mut target: T = Default::default();
        return if let Some(iden_list) = self.battle_results.get_iden_list(field_type, checksum) {
            let collection = fill_field_identifiers(iden_list, &value_list[1..])?;
            for item in collection {
                target.set(&item.0.to_lowercase().replace("/", ""), item.1)?;
            }

            Ok(target)
        } else {
            Err(anyhow!(
                "Value list of {:?} has unrecognized checksum({}). Format won't match",
                field_type,
                checksum
            ))
        };
    }

    /// The data structure that contains player info is a dict
    /// with wg_account_dbid as the key and an array(playerinfo) as the value
    fn parse_player_info_list(&self, input: &PickleValue) -> Result<HashMap<String, PlayerInfo>> {
        let dict = unpickler::access_dict(input)?;

        let mut player_info_list = HashMap::with_capacity(dict.len());

        for item in dict.into_iter() {
            let (account_dbid, value_list) = self.extract_from_item(item)?;
            let player_info: PlayerInfo = self.parse_collection(value_list, FieldType::PlayerInfo)?;

            player_info_list.insert(account_dbid, player_info);
        }

        Ok(player_info_list)
    }
    
    /// The data structure that contains account info is a dict
    /// with wg_account_dbid as the key and an array(playerinfo) as the value
    fn parse_all_account_info(&self, input: &PickleValue) -> Result<HashMap<String, AccountAll>> {
        let dict = unpickler::access_dict(input)?;

        let mut account_info_list = HashMap::with_capacity(dict.len());

        for item in dict.into_iter() {
            let (account_dbid, value_list) = self.extract_from_item(item)?;
            let account_info: AccountAll = self.parse_collection(value_list, FieldType::AccountAll)?;

            account_info_list.insert(account_dbid, account_info);
        }
        Ok(account_info_list)
    }

    /// The data structure that contains player info is a dict
    /// with wg_account_dbid as the key and an array(playerinfo) as the value
    fn parse_vehicle_all_list(&self, input: &PickleValue) -> Result<HashMap<String, VehicleAll>> {
        let dict = unpickler::access_dict(input)?;

        let mut vehicle_all_list = HashMap::with_capacity(dict.len());

        for item in dict.into_iter() {
            let (avatar_id, value_list) = self.extract_from_item(item)?;
            let vehicle_all: VehicleAll =
                self.parse_collection(value_list, FieldType::VehicleAll)?;

            vehicle_all_list.insert(avatar_id, vehicle_all);
        }
        Ok(vehicle_all_list)
    }
    
    /// Return the following when given a hashmap item:
    /// - `key` This is either the account_dbid or the avatar_id
    /// - `value_list` This finally should be a Vec but might have to parsed from either a dict or a list
    fn extract_from_item(
        &self,
        item: (HashablePickleValue, PickleValue),
    ) -> Result<(String, Vec<PickleValue>)> {
        let key = item.0.to_string();
        let value_list;

        // Item can either be a list or a dict
        // If dict we need to get the vec that is value of the first item in the dict
        match item.1 {
            PickleValue::List(list) => value_list = list,
            PickleValue::Dict(map) => {
                value_list = unpickler::access_list(&map.into_iter().next().unwrap().1)?
            }
            _ => {
                return Err(anyhow!(
                    "Value in (key,value) pair should be a list or dict"
                ))
            }
        }

        Ok((key, value_list))
    }
}

/// Given a vec of values (parsed from the dat file), the first element is the checksum
fn get_checksum(data_list: &[PickleValue]) -> Result<i32> {
    let checksum = unpickler::access_i64(&data_list[0])?;

    i32::try_from(checksum).context("checksum conversion error")
}

/// Generate a HashMap when given a list of identifiers and then a list of values for that identifiers
fn fill_field_identifiers(iden_list: Vec<Field>, value_list: &[PickleValue]) -> Result<HashMap<String, PickleValue>> {
    let mut result = HashMap::with_capacity(iden_list.len());

    iden_list
        .into_iter()
        .zip(value_list.iter())
        .for_each(|pair| {
            let (identifier, value) = pair;
            if *value == PickleValue::None {
                result.insert(
                    identifier.name.to_string(),
                    identifier.default.to_pickle_value(),
                );
            } else {
                result.insert(identifier.name.to_string(), value.clone());
            }
        });

    Ok(result)
}
