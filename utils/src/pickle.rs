use std::collections::BTreeMap;

use serde_pickle::{HashableValue, Value};

pub trait IntoSubValue {
    fn try_bytes<E>(self, f: fn() -> E) -> Result<Vec<u8>, E>;
    fn try_dict<E>(self, _: fn() -> E) -> Result<BTreeMap<HashableValue, Value>, E>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn try_i64<E>(self, f: fn() -> E) -> Result<i64, E>;
    fn try_list<E>(self, _: fn() -> E) -> Result<Vec<Value>, E>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl IntoSubValue for Value {
    fn try_bytes<E>(self, f: fn() -> E) -> Result<Vec<u8>, E> {
        match self {
            Value::Bytes(bytes) => Ok(bytes),
            _ => Err(f()),
        }
    }

    fn try_dict<E>(self, f: fn() -> E) -> Result<BTreeMap<HashableValue, Value>, E> {
        match self {
            Value::Dict(dict) => Ok(dict),
            _ => Err(f()),
        }
    }

    fn try_list<E>(self, f: fn() -> E) -> Result<Vec<Value>, E> {
        match self {
            Value::List(list) => Ok(list),
            Value::Tuple(list) => Ok(list),
            _ => Err(f()),
        }
    }

    fn try_i64<E>(self, f: fn() -> E) -> Result<i64, E> {
        match self {
            Value::I64(x) => Ok(x),
            _ => Err(f()),
        }
    }
}

impl IntoSubValue for HashableValue {
    fn try_bytes<E>(self, f: fn() -> E) -> Result<Vec<u8>, E> {
        match self {
            HashableValue::Bytes(bytes) => Ok(bytes),
            _ => Err(f()),
        }
    }

    fn try_i64<E>(self, f: fn() -> E) -> Result<i64, E> {
        match self {
            HashableValue::I64(x) => Ok(x),
            _ => Err(f()),
        }
    }
}
