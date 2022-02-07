use crate::battle_results::field_types::{FieldArenaType, ResultField, ResultFieldType};


pub const PLAYER_INFO: [ResultField; 7] = [
    ResultField("name", "<type 'str'>", "", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("realName", "<type 'str'>", "", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("clanDBID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("clanAbbrev", "<type 'str'>", "", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("prebattleID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("team", "<type 'int'>", "1", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("igrType", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
];

