pub mod update_arena;

use super::{MethodParser, UpdateArena};
use crate::{
    entity_defs::{EntityType, AVATAR_METHODS},
    packet_parser::prelude::*,
};


impl MethodParser for AvatarMethods {
    fn parse(input: &[u8], method_id: usize, context: &Context) -> Result<super::EntityMethod, PacketError>
    where
        Self: Sized,
    {
        let version = context.get_version();
        let version_str = crate::utils::version_as_string(version);

        let not_found_err = |err_msg| PacketError::NotFoundError {
            err: format!("{err_msg} version={version_str} method_id={method_id}"),
        };

        let methods = AVATAR_METHODS
            .get(&version_str)
            .ok_or_else(|| not_found_err("version not found"))?;

        let discrim = methods
            .get(method_id)
            .ok_or_else(|| not_found_err("method not found"))?;

        let method = match *discrim {
            "UpdateArena" => Ok(AvatarMethods::UpdateArena(UpdateArena::from(input, version)?)),
            _ => VariantDeserializer::deserialize_variant(discrim, input, &context),
        }
        .map_err(|err| PacketError::EntityMethodError {
            entity_type: AvatarMethods::entity_type(),
            method:      discrim,
            root_cause:  err.to_string(),
        })?;

        Ok(super::EntityMethod::Avatar(method))
    }
}

#[derive(Clone, Debug, strum::IntoStaticStr, Serialize, macros::EnumVariantDeserialize)]
pub enum AvatarMethods {
    AddRespawnGroup,
    BattleEventsSummary,
    BroadcastAFKWarning,
    CloseChooseSpawnKeyPointsWindow,
    CreateArenaObstacle,
    DestroyArenaObstacle,
    DisplayLastStandOnVehicle,
    EnemySPGHit,
    EnemySPGShotSound,
    EnteringProtectionZone,
    EventUpdateRespawnVehicles,
    ExplodeProjectile,
    ExplodeVehicleBeforeRespawn,
    ExternalTrigger,
    HandleScriptEventFromServer,
    HideAuraVictimMarkerIcon,
    HideAuraWarningHint,
    HideHint,
    LeavingProtectionZone,
    MessengerOnActionByServerChat2,
    NotifyCancelled,
    NotifyCannotStartRecovering,
    NotifyClients,
    OnAutoAimVehicleLost,
    OnBattleEvent,
    OnBattleEvents,
    OnBootcampEvent,
    OnChatAction,
    OnCmdResponse,
    OnCmdResponseExt,
    OnCollisionWithVehicle,
    OnCombatEquipmentShotLaunched,
    OnDestructibleDestroyed,
    OnEventPointsEvent,
    OnFrictionWithVehicle,
    OnHealthDataChanged,
    OnIGRTypeChanged,
    OnKickedFromArena,
    OnKickedFromServer,
    OnLineOfFrontPenetrated,
    OnLootAction,
    OnRTSEvent,
    OnRTSQueryResult,
    OnRankUpdate,
    OnRepairPointAction,
    OnRoundFinished,
    OnSectorBaseAction,
    OnSectorShooting,
    OnSmoke,
    OnStartVehicleControlFailed,
    OnStepRepairPointAction,
    OnSwitchViewpoint,
    OnTeamLivesRestored,
    OnTokenReceived,
    OnVehicleDangerousZoneTriggered,
    OnVehicleHealthChanged,
    OnWrongWay,
    OnXPUpdated,
    ProcessInvitations,
    ProtectionZoneShooting,
    ReceiveAccountStats,
    ReceiveAvailableSpawnKeyPoints,
    ReceiveHorn,
    ReceiveNotification,
    ReceivePhysicsDebugInfo,
    ReceiveSpawnKeyPointsCloseTime,
    ReceiveTeamSpawnKeyPoints,
    RedrawVehicleOnRespawn,
    RemoveVehicle,
    SendEquipmentBattleLogNotification,
    SendPlayerBattleLogNotification,
    SetConfigForCurrentMap,
    SetLastStandActive,
    SetUpdatedGoodiesSnapshot,
    ShowAuraHealthWarningHint,
    ShowAuraSoulsWarningHint,
    ShowAuraVictimMarkerIcon,
    ShowCarpetBombing,
    ShowDestructibleShotResults,
    ShowDevelopmentInfo,
    ShowHint,
    ShowHittingArea,
    ShowOtherVehicleDamagedDevices,
    ShowOwnVehicleHitDirection,
    ShowShotResults,
    ShowTracer,
    ShowVehicleDamageInfo,
    StopTracer,
    StopVehicleControl,
    SyncVehicleAttrs,
    Update,

    #[variant_de(manual)]
    UpdateArena(UpdateArena),

    UpdateAvatarPrivateStats,
    UpdateBomberTrajectory,
    UpdateCarriedFlagPositions,
    UpdateDualGunState,
    UpdateEvilEyeStatus,
    UpdateGasAttackState,
    UpdateGunMarker,
    UpdateLeviathanProgress,
    UpdateOwnVehiclePosition,
    UpdatePlaneTrajectory,
    UpdatePlayerLives,
    UpdatePositions,
    UpdateQuestProgress,
    UpdateResourceAmount,
    UpdateRespawnCooldowns,
    UpdateRespawnInfo,
    UpdateRespawnVehicles,
    UpdateScenarioTimer,
    UpdateSpawnList,
    UpdateState,
    UpdateTargetVehicleID,
    UpdateTargetingInfo,
    UpdateTeammateLives,
    UpdateTeamsHealthPercentage,
    UpdateVehicleAmmo,
    UpdateVehicleClipReloadTime,
    UpdateVehicleGunReloadTime,
    UpdateVehicleHealth,
    UpdateVehicleLimits,
    UpdateVehicleMiscStatus,
    UpdateVehicleOptionalDeviceStatus,
    UpdateVehicleQuickShellChanger,
    UpdateVehicleSetting,
    VehicleEnteredLoFEffect,
    WelcomeToSector,
}

impl AvatarMethods {
    pub fn entity_type() -> EntityType {
        EntityType::Avatar
    }
}
