use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WotValue {
    None,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    Text(String),
    Collection(Vec<WotValue>),
    NamedCollection(HashMap<String, WotValue>),
    NamedIntCollection(HashMap<i64, WotValue>),
    OutOfBounds,
    NotAllowed,
}

impl Default for WotValue {
    fn default() -> Self { WotValue::None }
}

impl WotValue {
    pub fn as_string(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::Text(s) => s.clone(),
            Self::OutOfBounds => panic!("Incorrect usage: OutOfBounds Value"),
            _ => panic!("Incorrect usage")
        }
    }
}