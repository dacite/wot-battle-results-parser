use std::collections::HashMap;

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

    // Note: The `Text` MUST BE above `Bytes` or player names will be parsed as HEX strings
    Text(String),

    #[serde(with = "serde_bytes")]
    Bytes(Vec<u8>),

    Collection(Vec<WotValue>),
    NamedCollection(HashMap<String, WotValue>),
    NamedIntCollection(HashMap<i64, WotValue>),
    NamedByteCollection(HashMap<Vec<u8>, WotValue>),
}

impl Default for WotValue {
    fn default() -> Self {
        WotValue::None
    }
}
