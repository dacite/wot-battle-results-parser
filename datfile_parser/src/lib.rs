mod field_types;

use std::collections::HashMap;
use unpickler::{access_bytes, access_dict, access_list, access_tuple, decompress_vec, load_pickle, PickleValue};
use anyhow::{anyhow, Result};

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
    let s = serde_json::to_string(&list2).unwrap();
    println!("{}", s);
    Ok(())

}

fn parse_list1(wrapped_list1: &PickleValue) -> Result<HashMap<String, PickleValue>> {
    let list1 = access_list(wrapped_list1)?;
    let mut result = HashMap::new();

    let mut i: usize = 1;
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

fn parse_list2(x: &PickleValue) -> Result<HashMap<String, serde_json::Value>>{
    let dict = access_dict(x)?;
    let list2;

    if let Some((tank_id, wrapped_list2)) = dict.into_iter().next() {
        list2 = access_list(&wrapped_list2)?;
    } else {
        return Err(anyhow!("Found empty list2"));
    }

    let mut result = HashMap::new();

    let mut i: usize = 1;
    for val in field_types::VEHICLE_ALL {
        let p: serde_json::Value  = serde_pickle::from_value(list2[i].clone()).expect("FAIL");
        result.insert(val.0.to_string(), p);
        i += 1;
    }
    for val in field_types::VEHICLE_SELF {
        let p = serde_pickle::from_value(list2[i].clone()).unwrap_or(serde_json::json!("null"));
            result.insert(val.0.to_string(), p);
            i+= 1;

        // let p: serde_json::Value  = serde_pickle::from_value(list2[i].clone()).expect("FAIL");
        // result.insert(val.0.to_string(), p);
        // i += 1;
    }
    if i != list2.len() {
        return Err(anyhow!("Wrong format: expected list2 to be {} items but found {} items", i, list2.len()));
    }
    Ok(result)
}

fn parse_list3(wrapped_list3: &[PickleValue])  {

}