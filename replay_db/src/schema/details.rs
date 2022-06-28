use macros::SqlInsert;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, SqlInsert)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    #[serde(rename = "arenaUniqueID")]
    pub arena_unique_id: String,
    #[serde(rename = "attackerAvatarID")]
    pub attacker_avatar_id: i64,
    #[serde(rename = "receiverAvatarID")]
    pub receiver_avatar_id: i64,
    pub attacket_type_comp_descr: i32,
    pub crits: i64,
    pub damage_assisted_inspire: i64,
    pub damage_assisted_radio: i64,
    pub damage_assisted_smoke: i64,
    pub damage_assisted_stun: i64,
    pub damage_assisted_track: i64,
    pub damage_blocked_by_armor: i64,
    pub damage_dealt: i64,
    pub damage_received: i64,
    pub death_reason: i64,
    pub direct_enemy_hits: i64,
    pub direct_hits: i64,
    pub explosion_hits: i64,
    pub fire: i64,
    pub no_damage_direct_hits_received: i64,
    pub piercing_enemy_hits: i64,
    pub piercings: i64,
    pub rickochets_received: i64,
    pub spotted: i64,
    pub stun_duration: i64,
    pub stun_num: i64,
    pub target_kills: i64,
}
