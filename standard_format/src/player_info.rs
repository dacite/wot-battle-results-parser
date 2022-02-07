use serde::{Serialize, Deserialize};

use crate::wot_value::WotValue;
use crate::FieldAccess;
use field_access_derive::FieldAccess;

#[derive(FieldAccess, Default, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    // Common
    name: WotValue,
    real_name: WotValue,
    clan_dbid: WotValue,
    clan_abbrev: WotValue,
    prebattle_id: WotValue,
    team: WotValue,
    igr_type: WotValue,

    // Steel Hunter?

    // Frontlines

    // Random Battles

    // Maps Training (Recon Mode?)

    // Ranked Battles
}


