use std::collections::BTreeMap;
pub use serde_pickle::Value as PickleValue;
pub use serde_pickle::HashableValue as HashablePickleValue;
use anyhow::{Result, Context, anyhow};
use serde_pickle::{HashableValue, Value};

pub fn load_pickle(input: &[u8]) -> Result<PickleValue> {
    let result = serde_pickle::value_from_slice(input, Default::default())
        .context("Parse failed: check if the input data is a result of a pickle dump (protocol 2)")?;

    Ok(result)
}

pub fn decompress_vec(compressed:&[u8]) -> Result<Vec<u8>> {
    let result = miniz_oxide::inflate::decompress_to_vec_zlib(compressed).map_err(|err| {
        anyhow!("Decompression failed: {:?}", err)
    })?;

    Ok(result)
}

pub fn access_tuple(x: &PickleValue) -> Result<Vec<PickleValue>> {
    return if let PickleValue::Tuple(value) = x {
        Ok(value.clone())
    } else {
        Err(anyhow!("Underlying PickleValue is not a tuple"))
    }
}

pub fn access_i64(x: &PickleValue) -> Result<i64> {
    return if let PickleValue::I64(value) = x {
        Ok(*value)
    } else {
        Err(anyhow!("Underlying PickleValue is not an i64"))
    }
}

pub fn access_bytes(x: &PickleValue) -> Result<Vec<u8>> {
    return if let PickleValue::Bytes(value) = x {
        Ok(value.clone())
    } else {
        Err(anyhow!("Underlying PickleValue is not a bytes buffer"))
    }
}

pub fn access_list(x: &PickleValue) -> Result<Vec<Value>> {
    return if let PickleValue::List(value) = x {
        Ok(value.clone())
    } else {
        Err(anyhow!("Underlying PickleValue is not a python list"))
    }
}

pub fn access_dict(x: &Value) -> Result<BTreeMap<HashablePickleValue, Value>> {
    return if let Value::Dict(value) = x {
        Ok(value.clone())
    } else {
        Err(anyhow!("Underlying PickleValue is not a python dictionary"))
    }
}

