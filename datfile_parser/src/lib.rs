mod field_types;

use std::collections::HashMap;
use unpickler::{access_bytes, access_dict, access_i64, access_list, access_tuple, decompress_vec, HashablePickleValue, load_pickle, PickleValue};
use anyhow::{anyhow, Result};
use serde_json::Value;

/// A list of key value of pairs where the key is a String
type Dict = HashMap<String, PickleValue>;

/// A list of key value of pairs where the key is an i32
type IntDict = HashMap<i64, Dict>;



// Key value pair where the key is the tank_id and value is the
type VehicleAll = HashMap<String, PickleValue>;

pub fn parse(input: &[u8]) -> Result<()>{
    // Load the root pickle
    let root_pickle = load_pickle(input)?;

    // root pickle is a tuple of the shape : (i64, Tuple)
    let root_tuple = access_tuple(&root_pickle)?;

    // root tuple should contain the following: (arenaUniqueID, [u8], [u8], [u8])
    // the three u8 buffers (named buffer1, buffer2, buffer3 respectively) in this tuple are
    // compressed pickle dumps
    let data_tuple = access_tuple(&root_tuple[1])?;
    let mut pickle_list = Vec::new();
    for i in 1..data_tuple.len() {
        let compressed = access_bytes(&data_tuple[i])?;
        let decompressed = decompress_vec(&compressed)?;
        let pickle = load_pickle(&decompressed)?;

        pickle_list.push(pickle);
    }

    let list1 = parse_list1(&pickle_list[0]).unwrap();
    let list2 = parse_list2(&pickle_list[1]).unwrap();
    let list3 = parse_list3(&pickle_list[2]).unwrap();
    println!("{:#?}", list3);
    Ok(())

}

fn parse_list1(wrapped_list1: &PickleValue) -> Result<HashMap<String, PickleValue>> {
    let list1 = access_list(wrapped_list1)?;
    let mut result = HashMap::new();

    let mut i = 1;
    for val in field_types::ACCOUNT_ALL {
        result.insert(val.0.to_string(), list1[i].clone());
        i += 1;
    }
    for val in field_types::ACCOUNT_SELF {
        result.insert(val.0.to_string(), list1[i].clone());
        i += 1;
    }
    // if i != list1.len() {
    //     return Err(anyhow!("Wrong format: expected list1 to be {} items but found {} items", i, list1.len()));
    // }
    Ok(result)
}

fn parse_list2(x: &PickleValue) -> Result<HashMap<String, PickleValue>>{
    let dict = access_dict(x)?;
    let list2;

    if let Some((tank_id, wrapped_list2)) = dict.into_iter().next() {
        list2 = access_list(&wrapped_list2)?;
    } else {
        return Err(anyhow!("Found empty list2"));
    }

    let mut result = HashMap::new();

    let mut i = 1;
    for val in [field_types::VEHICLE_ALL.as_slice(), field_types::VEHICLE_SELF.as_slice()].concat() {
        result.insert(val.0.to_string(), list2[i].clone());
        i += 1;
    }

    if i != list2.len() {
        return Err(anyhow!("Wrong format: expected list2 to be {} items but found {} items", i, list2.len()));
    }
    Ok(result)
}

fn parse_list3(x: &PickleValue)  -> Result<(HashMap<String, PickleValue>, HashMap<String, HashMap<String, PickleValue>>, HashMap<String, IntDict>)>{

    let tuple = access_tuple(x)?;
    let common_list_input = access_list(&tuple[0])?;
    let player_info_list_input = access_dict(&tuple[1])?;
    let account_all_list_input = access_dict(&tuple[2])?;

    let mut common = HashMap::new();
    let mut player_info_list = HashMap::new();
    let mut account_info_list = HashMap::new();
    let mut i = 1;
    for val in field_types::COMMON {
        common.insert(val.0.to_string(), common_list_input[i].clone());
        i += 1;
    }

    for player_info in &player_info_list_input {
        let player_wg_id = access_i64(&player_info.0.clone().into_value())?;

        player_info_list.insert(player_wg_id.to_string(), get_player_info(player_info.1)?);
    }

    for account_all_info in &account_all_list_input {
        let player_avatar_id = access_i64(&account_all_info.0.clone().into_value())?;

        account_info_list.insert(player_avatar_id.to_string(), get_vehicle_all(account_all_info.1)?);
    }


    Ok((common, player_info_list, account_info_list))

}

fn get_player_info(x: &PickleValue) -> Result<HashMap<String, PickleValue>> {
    let values_list = access_list(x)?;

    let mut player_info = HashMap::new();
    let mut i = 1;
    for val in field_types::PLAYER_INFO {
        player_info.insert(val.0.to_string(), values_list[i].clone());
        i+= 1;
    }

    Ok(player_info)
}

fn get_vehicle_all(x: &PickleValue) -> Result<HashMap<i64, Dict>> {
    let outer_dict = access_dict(x)?;
    let inner_dict: (HashablePickleValue, PickleValue)  = get_single_item(outer_dict)?;
    let tank_id = access_i64(&inner_dict.0.into_value())?;
    let account_all_list = access_list(&inner_dict.1)?;
    let mut account_all = HashMap::new();
    let mut i = 1;
    for val in field_types::VEHICLE_ALL {
        account_all.insert(val.0.to_string(), account_all_list[i].clone());
        i += 1;
    }
    let tank_id_account_all= [(tank_id, account_all)].into_iter().collect();
    Ok(tank_id_account_all)
}

fn get_single_item<T: IntoIterator + IntoIterator<Item = V>, V>(x: T) -> Result<V> {
    let mut result;
    let mut dict = x.into_iter();
    if let Some(item) = dict.next() {
        result = Ok(item);
    } else {
        result = Err(anyhow!("Input dictionary is empty"));
    }

    if dict.next().is_some() {
        result = Err(anyhow!("Input dictionary cannot contain more than one item"));
    }

    return result;
}

fn fill_field_identifiers(iden_list:(&str, &str), value_list: &PickleValue) -> HashMap<String, PickleValue>{

}