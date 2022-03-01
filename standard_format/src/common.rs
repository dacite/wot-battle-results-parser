use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use field_access_derive::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct Common {
    arena_type_id: WotValue,
    arena_create_time: WotValue,
    winner_team: WotValue,
    finish_reason: WotValue,
    gas_attack_winner_team: WotValue,
    duration: WotValue,
    bonus_type: WotValue,
    gui_type: WotValue,
    veh_lock_mode: WotValue,
    division: WotValue,
    bots: WotValue,
    common_num_started: WotValue,
    common_num_destroyed: WotValue,
    common_num_defended: WotValue,
    common_num_captured: WotValue,

    #[custom_parser = "parse_account_comp_descr"]
    account_comp_descr: WotValue,

    team_health: WotValue,
}

impl Common {
    pub fn parse_account_comp_descr(&mut self, _item: serde_pickle::Value) -> WotValue {
        WotValue::None
    }
}

