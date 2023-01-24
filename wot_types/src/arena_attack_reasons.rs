use num_enum::TryFromPrimitive;
use serde::Serialize;


// Took from https://github.com/IzeBerg/wot-src/blob/EU/sources/res/scripts/common/constants.py
// ATTACK_REASONS = (...
#[repr(i32)]
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive, strum::Display, Serialize)]
pub enum AttackReason {
    Shot                    = 0,
    Fire                    = 1,
    Ram                     = 2,
    WorldCollision          = 3,
    DeathZone               = 4,
    Drowing                 = 5,
    GasAttack               = 6,
    Overturn                = 7,
    Manual                  = 8,
    ArtilleryProtection     = 9,
    ArtillerySector         = 10,
    Bombers                 = 11,
    Recovery                = 12,
    Artillery               = 13,
    Bomber                  = 14,
    Minefield               = 15,
    None                    = 16,
    SpawnedBotExplosion     = 17,
    Berserker               = 18,
    Smoke                   = 19,
    CorrogingShot           = 20,
    AdaptationHealthRestore = 21,
    ThunderStrike           = 22,
    FireCircle              = 23,
    ClingBrander            = 24,
    ClingBranderRam         = 25,
    BranderRam              = 26,
    FortArtillery           = 27,
}
