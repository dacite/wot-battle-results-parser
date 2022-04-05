use macros::FieldAccess;
use serde::{Deserialize, Serialize};

// use crate::wot_value::WotValue;
use crate::FieldAccess;
use crate::WotValue;
#[derive(FieldAccess, Default, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    // Common
    name:         String,
    real_name:    String,
    clan_dbid:    i64,
    clan_abbrev:  String,
    prebattle_id: i32,
    team:         i32,
    igr_type:     i32,
    // Steel Hunter?

    // Frontlines

    // Random Battles

    // Maps Training (Recon Mode?)

    // Ranked Battles
}
