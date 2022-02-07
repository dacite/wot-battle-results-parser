mod wot_value;
mod common;
mod account_self;
mod player_info;
mod account_all;
mod vehicle_self;
mod vehicle_all;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub use crate::wot_value::WotValue;
pub use crate::common::Common;
pub use crate::account_self::AccountSelf;
pub use crate::player_info::PlayerInfo;
pub use crate::account_all::AccountAll;
pub use crate::vehicle_all::VehicleAll;
pub use crate::vehicle_self::VehicleSelf;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Battle {
    pub arena_unique_id: String,
    pub common: Common,
    pub player_info: HashMap<String, PlayerInfo>,
    pub account_all: HashMap<String, AccountAll>,
    pub vehicle_all: HashMap<String, VehicleAll>,
    pub vehicle_self: HashMap<String, VehicleSelf>,
    pub account_self: HashMap<String, AccountSelf>
}

pub trait FieldAccess {
    fn get(&self, index: &str) -> &WotValue;
    fn set(&mut self, index: &str, val: serde_pickle::Value) -> std::result::Result<(), String>;
}
