use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WotValue {
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),

    #[serde(with = "serde_bytes")]
    Bytes(Vec<u8>),

    Text(String),
    Collection(Vec<WotValue>),
    NamedCollection(HashMap<String, WotValue>),
    NamedIntCollection(HashMap<i64, WotValue>),
    NamedByteCollection(HashMap<Vec<u8>, WotValue>),
    OutOfBounds,
    NotAllowed,
}

impl Default for WotValue {
    fn default() -> Self {
        WotValue::None
    }
}
