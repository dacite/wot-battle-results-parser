use crate::battle_results::field_types::ResultField;
use crate::battle_results::ResultFieldType;

pub const ACCOUNT_ALL: [ResultField; 7] = [
    ResultField("avatarDamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("avatarKills", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("avatarDamaged", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("totalDamaged", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("fairplayViolations", "<type 'tuple'>", "(0, 0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("badges", "<type 'tuple'>", "([], [])", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("playerRank", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
];
