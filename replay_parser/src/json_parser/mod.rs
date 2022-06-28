mod schemas;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{Error, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReplaySummary {
    #[serde(flatten)]
    summary: schemas::ReplayCommon,

    damage_dealt: Option<i64>,
    kills:        Option<i64>,
    xp:           Option<i64>,
    credits:      Option<i64>,
}

pub trait JsonParser {
    fn get_json(&self) -> &[JsonValue];

    /// Whether complete JSON information about a replay is present. It should be present if the player
    /// watched the battle to the end
    fn has_complete_json(&self) -> bool {
        self.get_json().len() > 1
    }

    /// Get VehicleSelf(data of the recording player) is only present if the player watched the battle to the
    /// end
    fn get_vehicle_self(&self) -> Option<Vec<&JsonValue>> {
        let json = self.get_json().get(1)?; // index 1 is available for only for complete replays
        let json = json.pointer("/0/personal")?;

        // This json object contains objects that maybe "avatar" or "vehicle_ids"
        let json = json.as_object()?;

        let mut vehicle_self_list = Vec::new();
        for (key, val) in json {
            if key != "avatar" {
                vehicle_self_list.push(val);
            }
        }

        Some(vehicle_self_list)
    }

    // TODO: Remove unwraps
    fn get_replay_summary(&self) -> Result<ReplaySummary> {
        let json = self
            .get_json()
            .get(0)
            .ok_or(Error::JsonKeyError("no json in replay"))?;

        let summary = serde_json::from_value(json.clone()).unwrap();

        let (damage_dealt, kills, xp, credits) = if let Some(vehicle_self_list) = self.get_vehicle_self() {
            let mut damage_dealt = 0;
            let mut kills = 0;
            let mut credits = 0;
            let mut xp = 0;
            // In Some gamemodes you can use multiple vehicles so we add all them up in the summary
            for vehicle_self in vehicle_self_list {
                damage_dealt += get_value(vehicle_self, "/damageDealt")?.as_i64().unwrap();
                kills += get_value(vehicle_self, "/kills")?.as_i64().unwrap();
                xp += get_value(vehicle_self, "/factualXP")?.as_i64().unwrap();
                credits += get_value(vehicle_self, "/factualCredits")?.as_i64().unwrap();
            }

            (Some(damage_dealt), Some(kills), Some(xp), Some(credits))
        } else {
            (None, None, None, None)
        };

        Ok(ReplaySummary {
            summary,
            damage_dealt,
            kills,
            credits,
            xp,
        })
    }
}

fn get_value<'a>(json: &'a JsonValue, key: &'static str) -> Result<&'a JsonValue> {
    if let Some(value) = json.pointer(key) {
        Ok(value)
    } else {
        Err(Error::JsonKeyError(key))
    }
}
