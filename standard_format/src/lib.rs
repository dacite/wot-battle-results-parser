mod account_all;
mod account_self;
mod common;
mod player_info;
mod vehicle_all;
mod vehicle_self;
mod wot_value;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use crate::account_all::AccountAll;
pub use crate::account_self::AccountSelf;
pub use crate::common::Common;
pub use crate::player_info::PlayerInfo;
pub use crate::vehicle_all::VehicleAll;
pub use crate::vehicle_self::VehicleSelf;
pub use crate::wot_value::WotValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct Battle {
    pub arena_unique_id: String,
    pub common:          Common,
    pub player_info:     HashMap<String, PlayerInfo>,
    pub account_all:     HashMap<String, AccountAll>,
    pub vehicle_all:     HashMap<String, VehicleAll>,
    pub vehicle_self:    HashMap<String, VehicleSelf>,
    pub account_self:    HashMap<String, AccountSelf>,
}

// // To be removed
// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct BattleCSV {
//     #[serde(flatten)]
//     players: HashMap<String, PlayerCSV>,
// }

// // To be removed
// #[derive(Debug, Serialize, Deserialize)]
// struct PlayerCSV {
//     pub arena_unique_id: String,

//     pub avatar_id: String,

//     #[serde(flatten)]
//     pub common: Common,

//     #[serde(flatten)]
//     pub player_info: PlayerInfo,

//     #[serde(flatten)]
//     pub account_all: AccountAll,

//     #[serde(flatten)]
//     pub vehicle_all: VehicleAll,

//     #[serde(flatten)]
//     pub vehicle_self: Option<VehicleSelf>,

//     #[serde(flatten)]
//     pub account_self: Option<AccountSelf>,
// }

// impl Battle {
//     pub fn to_csv_compatible(&self) -> BattleCSV {
//         let mut players = HashMap::new();

//         // We will use account_dbids as the indexes
//         for player in self.vehicle_all.clone() {
//             let avatar_id = player.0.clone();
//             let vehicle_all = player.1.clone();

//             let account_dbid =
// vehicle_all.get("accountdbid").unwrap().as_str().unwrap().to_string();

//             let player_info =
// self.player_info.get(&account_dbid).unwrap().clone();             let
// account_all = self.account_all.get(&account_dbid).unwrap().clone();
//             let mut vehicle_self = None;
//             let mut account_self = None;

//             if let Some(result) = self.vehicle_self.get(&account_dbid) {
//                 vehicle_self = Some(result.clone())
//             }
//             if let Some(result) = self.account_self.get(&account_dbid) {
//                 account_self = Some(result.clone())
//             }

//             let player_csv = PlayerCSV {
//                 avatar_id,
//                 arena_unique_id: self.arena_unique_id.clone(),
//                 common: self.common.clone(),
//                 player_info,
//                 account_all,
//                 vehicle_all,
//                 vehicle_self,
//                 account_self,
//             };

//             players.insert(player.0.clone(), player_csv);
//         }

//         BattleCSV { players }
//     }
// }

pub trait FieldAccess {
    fn get(&self, index: &str) -> Option<serde_json::Value>;
    fn set(&mut self, index: &str, val: serde_pickle::Value) -> anyhow::Result<()>;
}
