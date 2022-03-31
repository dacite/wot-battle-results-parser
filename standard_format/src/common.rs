use serde::{Serialize, Deserialize};

use crate::FieldAccess;
use macros::FieldAccess;
use crate::WotValue;
#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct Common {
    arena_type_id: i32,
    arena_create_time: i32,
    winner_team: i32,
    finish_reason: i32,
    gas_attack_winner_team: i32,
    duration: i32,
    bonus_type: i32,
    gui_type: i32,
    veh_lock_mode: i32,
    division: serde_json::Value,
    bots: serde_json::Value,
    common_num_started: i32,
    common_num_destroyed: i32,
    common_num_defended: i32,
    common_num_captured: i32,

    #[custom_parser = "parse_account_comp_descr"]
    account_comp_descr: serde_json::Value,

    team_health: serde_json::Value,
}

impl Common {
    pub fn parse_account_comp_descr(&mut self, _item: serde_pickle::Value) -> serde_json::Value {
        serde_json::Value::Null
    }
}

