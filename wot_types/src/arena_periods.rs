use num_enum::TryFromPrimitive;
use serde::Serialize;

// Took from https://github.com/IzeBerg/wot-src/blob/EU/sources/res/scripts/common/constants.py
#[repr(i32)]
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive, strum::Display, Serialize)]
pub enum ArenaPeriod {
    Idle        = 0,
    Waiting     = 1,
    PreBattle   = 2,
    Battle      = 3,
    AfterBattle = 4,
}

// Took from https://github.com/IzeBerg/wot-src/blob/EU/sources/res/scripts/common/constants.py
#[repr(i32)]
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive, strum::Display, Serialize)]
pub enum FinishReason {
    Unknown             = 0,
    Extermination       = 1,
    Base                = 2,
    Timeout             = 3,
    Failure             = 4,
    Technical           = 5,
    WinPointsCap        = 6,
    WinPoints           = 7,
    AllyKilled          = 8,
    OwnVehicleDestroyed = 9,
    DestroyedObjects    = 10,
}
