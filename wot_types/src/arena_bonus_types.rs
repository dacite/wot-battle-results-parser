use num_enum::TryFromPrimitive;

/// Gamemode. It could be a random battle or a frontlines battle etc.
#[repr(i32)]
#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug, TryFromPrimitive, strum::Display)]
pub enum ArenaBonusType {
    Unknown              = 0,
    Regular              = 1,
    Training             = 2,
    Company              = 3,
    Tournament           = 4,
    Clan                 = 5,
    Tutorial             = 6,
    Cybersport           = 7,
    Historical           = 8,
    EventBattles         = 9,
    Sortie               = 10,
    FortBattle           = 11,
    RatedCybersport      = 12,
    GlobalMap            = 13,
    TournamentRegular    = 14,
    TournamentClan       = 15,
    RatedSandbox         = 16,
    Sandbox              = 17,
    FalloutClassic       = 18,
    FalloutMultiteam     = 19,
    Sortie2              = 20,
    FortBattle2          = 21,
    Ranked               = 22,
    Bootcamp             = 23,
    EpicRandom           = 24,
    EpicRandomTraining   = 25,
    EventBattles2        = 26,
    EpicBattle           = 27,
    EpicBattleTraining   = 28,
    BattleRoyaleSolo     = 29,
    BattleRoyaleSquad    = 30,
    TournamentEvent      = 31,
    Bob                  = 32,
    EventRandom          = 33,
    BattleRoyaleTrnSolo  = 34,
    BattleRoyaleTrnSquad = 35,
    WeekendBrawl         = 36,
    Mapbox               = 37,
    MapsTraining         = 38,

    /// `Rts` is the Art of Strategy Gameode
    Rts                  = 39,
    Rts1x1               = 40,
    RtsBootcamp          = 41,
    FunRandom            = 42,
    Comp7                = 43,
}
