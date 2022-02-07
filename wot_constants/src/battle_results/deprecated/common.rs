use crate::battle_results::field_types::ResultField;
use crate::battle_results::ResultFieldType;

pub const COMMON: [ResultField; 17] = [
    ResultField("arenaTypeID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("arenaCreateTime", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("winnerTeam", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("finishReason", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("gasAttackWinnerTeam", "<type 'int'>", "-1", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("duration", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("bonusType", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("guiType", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("vehLockMode", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("division", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("bots", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("commonNumStarted", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("commonNumDestroyed", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("commonNumDefended", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("commonNumCaptured", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("accountCompDescr", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::Common),
    ResultField("teamHealth", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::Common),
];