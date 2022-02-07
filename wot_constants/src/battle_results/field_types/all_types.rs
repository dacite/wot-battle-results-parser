use crate::battle_results::field_types::ResultField;
use crate::battle_results::ResultFieldType;

pub const ALL_TYPES: [ResultField; 346] = [
    ResultField("health", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("maxHealth", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("credits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("xp", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("xp/attack", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("xp/assist", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("xp/other", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("xpPenalty", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("achievementCredits", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("achievementXP", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("achievementFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("shots", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("directHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("directEnemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("directTeamHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("explosionHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("piercings", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("piercingEnemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("sniperDamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("equipmentDamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageAssistedRadio", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageAssistedTrack", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageAssistedStun", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageAssistedSmoke", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageAssistedInspire", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("stunNum", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("stunDuration", "<type 'float'>", "0.0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageReceived", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageReceivedFromInvisibles", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damageBlockedByArmor", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("directHitsReceived", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("noDamageDirectHitsReceived", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("explosionHitsReceived", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("piercingsReceived", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("tdamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("tdestroyedModules", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("tkills", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("isTeamKiller", "<type 'bool'>", "False", "<type 'NoneType'>", "max", ResultFieldType::VehicleAll),
    ResultField("capturePoints", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("capturingBase", "None", "None", "<type 'NoneType'>", "any", ResultFieldType::VehicleAll),
    ResultField("droppedCapturePoints", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("mileage", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("lifeTime", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("killerID", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleAll),
    ResultField("achievements", "<type 'list'>", "[]", "<type 'NoneType'>", "extend", ResultFieldType::VehicleAll),
    ResultField("inBattleAchievements", "<type 'list'>", "[]", "<type 'NoneType'>", "extend", ResultFieldType::VehicleAll),
    ResultField("potentialDamageReceived", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("rolloutsCount", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("deathCount", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("flagActions", "<type 'list'>", "[0, 0, 0, 0]", "<type 'NoneType'>", "sumInEachPos", ResultFieldType::VehicleAll),
    ResultField("soloFlagCapture", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("flagCapture", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("winPoints", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("resourceAbsorbed", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("stopRespawn", "<type 'bool'>", "False", "<type 'NoneType'>", "max", ResultFieldType::VehicleAll),
    ResultField("numRecovered", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("vehicleNumCaptured", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("destructiblesNumDestroyed", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("destructiblesDamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("destructiblesHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("numDefended", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("accountDBID", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleAll),
    ResultField("typeCompDescr", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("index", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("deathReason", "<type 'int'>", "-1", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("team", "<type 'int'>", "1", "<type 'NoneType'>", "skip", ResultFieldType::VehicleAll),
    ResultField("kills", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("spotted", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damaged", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("damagedHp", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("stunned", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleAll),
    ResultField("repair", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("freeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("details", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("creditsPenalty", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("creditsContributionIn", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("creditsContributionOut", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsToDraw", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("creditsToDraw", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("damageBeforeTeamWasDamaged", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("killsBeforeTeamWasDamaged", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("percentFromTotalTeamDamage", "<type 'float'>", "0.0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("winAloneAgainstVehicleCount", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("percentFromSecondBestDamage", "<type 'float'>", "0.0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("killedAndDamagedByAllSquadmates", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("damagedWhileMoving", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("damagedWhileEnemyMoving", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("committedSuicide", "<type 'bool'>", "False", "<type 'NoneType'>", "max", ResultFieldType::VehicleSelf),
    ResultField("crystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventCoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("bpcoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("piggyBank", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventGold", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventCrystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventEventCoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventBpcoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("originalCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("creditsReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("xpReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("freeXPReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("tmenXPReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("tmenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("originalGold", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("goldReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("gold", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("originalCrystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("crystalReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalEventCoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("originalBpcoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventCoinReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("bpcoinReplay", "<type 'str'>", "", "<type 'instance'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("factualXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("factualFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("factualCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalGold", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalCrystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalEventCoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("subtotalBpcoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("eventCreditsList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventXPList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventFreeXPList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventTMenXPList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventGoldList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventCrystalList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventEventCoinList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventBpcoinList", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventCreditsFactor1000List", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventXPFactor100List", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventFreeXPFactor100List", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventTMenXPFactor100List", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("eventGoldFactor100List", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalXPPenalty", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsPenalty", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsContributionIn", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsContributionOut", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("premiumVehicleXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("premiumVehicleXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("squadXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("squadXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("referral20XP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("referral20XPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("referral20Credits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("referral20CreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("premiumXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premiumPlusXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("appliedPremiumXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premiumTmenXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premiumPlusTmenXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("appliedPremiumTmenXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premiumCreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premiumPlusCreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("appliedPremiumCreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premSquadCreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("originalPremSquadCredits", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premSquadCredits", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("dailyXPFactor10", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::VehicleSelf),
    ResultField("additionalXPFactor10", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("igrXPFactor10", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::VehicleSelf),
    ResultField("aogasFactor10", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::VehicleSelf),
    ResultField("refSystemXPFactor10", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("fairplayFactor10", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("orderCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("orderXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("orderFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("orderTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("orderCreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("orderXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("orderFreeXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("orderTMenXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("boosterCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("boosterXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("boosterFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("boosterTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("boosterCreditsFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("boosterXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("boosterFreeXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("boosterTMenXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("playerRankXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("playerRankXPFactor100", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("isPremium", "<type 'bool'>", "False", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("premMask", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("xpByTmen", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("autoRepairCost", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("autoLoadCost", "<type 'tuple'>", "(0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("autoEquipCost", "<type 'tuple'>", "(0, 0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("autoEquipBoostersCost", "<type 'tuple'>", "(0, 0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("prevMarkOfMastery", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("markOfMastery", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("dossierPopUps", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("dossierLogRecords", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("vehTypeLockTime", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("serviceProviderID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("marksOnGun", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("movingAvgDamage", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("damageRating", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("battleNum", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("questsProgress", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinDicts", ResultFieldType::VehicleSelf),
    ResultField("c11nProgress", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsToDrawSquad", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsPenaltySquad", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsContributionInSquad", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::VehicleSelf),
    ResultField("originalCreditsContributionOutSquad", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::VehicleSelf),
    ResultField("avatarDamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("avatarKills", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("avatarDamaged", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("totalDamaged", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("fairplayViolations", "<type 'tuple'>", "(0, 0, 0)", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("badges", "<type 'tuple'>", "([], [])", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("rankChange", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("avatarAmmo", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("avatarDamageEventList", "<type 'set'>", "set([])", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("accountDBID", "<type 'int'>", "0", "<type 'NoneType'>", "any", ResultFieldType::AccountSelf),
    ResultField("team", "<type 'int'>", "1", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("clanDBID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("fortClanDBIDs", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("winnerIfDraw", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("isPrematureLeave", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("watchedBattleToTheEnd", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("vseBattleResults", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("squadBonusInfo", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("progressiveReward", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("eligibleForCrystalRewards", "<type 'bool'>", "False", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("activeRents", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("recruitsIDs", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("recruiterID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("fareTeamXPPosition", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("questsProgress", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinDicts", ResultFieldType::AccountSelf),
    ResultField("PM2Progress", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("dogTags", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("eventCredits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventFreeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventTMenXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventGold", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventCrystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventEventCoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("eventBpcoin", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("credits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("xp", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("freeXP", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("crystal", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::AccountSelf),
    ResultField("name", "<type 'str'>", "", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("realName", "<type 'str'>", "", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("clanDBID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("clanAbbrev", "<type 'str'>", "", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("prebattleID", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("team", "<type 'int'>", "1", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
    ResultField("igrType", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::PlayerInfo),
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
    ResultField("canStun", "<type 'bool'>", "False", "<type 'NoneType'>", "any", ResultFieldType::Server),
    ResultField("potentialDamageDealt", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("soloHitsAssisted", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("isEnemyBaseCaptured", "<type 'bool'>", "False", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("stucks", "<type 'list'>", "[]", "<class 'DictPackers.DeltaPacker'>", "extend", ResultFieldType::Server),
    ResultField("autoAimedShots", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("presenceTime", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("spotList", "<type 'list'>", "[]", "<type 'NoneType'>", "extend", ResultFieldType::Server),
    ResultField("ammo", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("crewActivityFlags", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("series", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("tkillRating", "<type 'float'>", "0.0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("thitPenalties", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinTHitPenalties", ResultFieldType::Server),
    ResultField("destroyedObjects", "<type 'dict'>", "{}", "<type 'NoneType'>", "sumByEackKey", ResultFieldType::Server),
    ResultField("discloseShots", "<type 'list'>", "[]", "<class 'DictPackers.DeltaPacker'>", "extend", ResultFieldType::Server),
    ResultField("critsCount", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("aimerSeries", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("observedByEnemyTime", "<type 'int'>", "-1", "<type 'NoneType'>", "any", ResultFieldType::Server),
    ResultField("critsByType", "<type 'dict'>", "{}", "<class 'DictPackers.DictPacker'>", "joinCritsByType", ResultFieldType::Server),
    ResultField("innerModuleCritCount", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("innerModuleDestrCount", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("isAnyOurCrittedInnerModules", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("killsAssistedTrack", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("killsAssistedRadio", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("killsAssistedStun", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damagedVehicleCntAssistedTrack", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damagedVehicleCntAssistedRadio", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damagedVehicleCntAssistedStun", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("isNotSpotted", "<type 'bool'>", "True", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("isAnyHitReceivedWhileCapturing", "<type 'bool'>", "False", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("damageAssistedRadioWhileInvisible", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damageAssistedTrackWhileInvisible", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damageAssistedStunWhileInvisible", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damageEventList", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinTargetEventLists", ResultFieldType::Server),
    ResultField("stunEventList", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinTargetEventLists", ResultFieldType::Server),
    ResultField("assistEventList", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinTargetEventLists", ResultFieldType::Server),
    ResultField("damageFromEnemiesEventList", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinTargetEventLists", ResultFieldType::Server),
    ResultField("multiDamageEvents", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinDicts", ResultFieldType::Server),
    ResultField("multiStunEvents", "<type 'dict'>", "{}", "<type 'NoneType'>", "joinDicts", ResultFieldType::Server),
    ResultField("inBattleMaxSniperSeries", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("inBattleMaxKillingSeries", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("inBattleMaxPiercingSeries", "<type 'int'>", "0", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("firstDamageTime", "<type 'int'>", "0", "<type 'NoneType'>", "min", ResultFieldType::Server),
    ResultField("consumedAmmo", "None", "None", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("ironShieldDamage", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("occupyingForceDestruction", "<type 'bool'>", "False", "<type 'NoneType'>", "max", ResultFieldType::Server),
    ResultField("occupyingForceBasePoints", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("directEnemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("explosionEnemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("piercingEnemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("indirectEnemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("enemyHits", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("spottedBeforeWeBecameSpotted", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("spottedAndDamagedSPG", "<type 'int'>", "0", "<type 'NoneType'>", "sum", ResultFieldType::Server),
    ResultField("damageList", "<type 'list'>", "[]", "<type 'NoneType'>", "extend", ResultFieldType::Server),
    ResultField("killList", "<type 'list'>", "[]", "<type 'NoneType'>", "extend", ResultFieldType::Server),
    ResultField("vehLockTimeFactor", "<type 'float'>", "0.0", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("misc", "<type 'dict'>", "{}", "<type 'NoneType'>", "any", ResultFieldType::Server),
    ResultField("vehsByClass", "<type 'dict'>", "{}", "<type 'NoneType'>", "any", ResultFieldType::Server),
    ResultField("avatarAmmoEquipped", "<type 'set'>", "set([])", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("cybersportRatingDeltas", "<type 'tuple'>", "(0.0, 0.0)", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("vehRankRaised", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("eventGoldByEventID", "<type 'dict'>", "{}", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("playerRank", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountAll),
    ResultField("quickShellChangerUsageCount", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("goldBankGain", "<type 'int'>", "0", "<type 'NoneType'>", "skip", ResultFieldType::AccountSelf),
    ResultField("setupsIndexes", "<type 'dict'>", "{}", "<type 'NoneType'>", "any", ResultFieldType::VehicleSelf),
    ResultField("startAmmo", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::Server),
    ResultField("initialVehicleAmmo", "<type 'list'>", "[]", "<type 'NoneType'>", "skip", ResultFieldType::Server),
];