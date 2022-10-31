use nom::number::complete::le_u8;
use standard_format::WotValue;
use utils::try_variant;

use crate::packet_parser::serde_packet;
use crate::Result;

#[derive(Debug, Clone)]
pub struct UpdateArena {
    _update_type: u8,
}

impl UpdateArena {
    pub fn from(data: &[u8], _version: [u16; 4]) -> Result<Self> {
        let (remaining, update_type) = le_u8(data)?;
        let (_remaining, arena_data) = serde_packet::parse_byte_array(remaining)?;

        parse_arena_data(update_type, arena_data).unwrap();

        Ok(UpdateArena {
            _update_type: update_type,
        })
    }
}

fn parse_arena_data(update_type: u8, arena_data: &[u8]) -> Result<()> {
    if update_type == ArenaUpdate::VehicleList as u8 {
        let decompressed = unpickler::decompress_vec(arena_data).unwrap();
        let pickle_value = serde_pickle::value_from_slice(
            &decompressed,
            serde_pickle::DeOptions::new().replace_unresolved_globals(),
        )
        .unwrap();

        // println!("{:#?}", pickle_value);

        let wot_value = serde_pickle::from_value(pickle_value).unwrap();
        let wot_value = try_variant!(wot_value, WotValue::Collection).unwrap();
        let wot_value = try_variant!(wot_value[0].clone(), WotValue::Collection).unwrap();
        println!("{:#?}", wot_value[1]);
        // let wot_value = try_variant!(wot_value[1].clone(), WotValue::Bytes).unwrap();
        todo!()
    }

    Ok(())
}

fn get_vehicle_type(compact_description: Vec<u8>, extra_data: Option<i32>) {}

fn split_vehicle_compact_descr(compact_description: Vec<u8>) {
    let header = *compact_description.get(0).unwrap();
    let vehicle_type_id = *compact_description.get(1).unwrap();
    let nation_id = header >> 4 & 15;
}
// TODO: We might need to do a versioned way of doing this because there is another dictionary that only
// TODO: contains some of these that's used by WOT
#[allow(dead_code)]
enum ArenaUpdate {
    VehicleList               = 1,
    VehicleAdded              = 2,
    Period                    = 3,
    Statistics                = 4,
    VehicleStatistics         = 5,
    VehicleKilled             = 6,
    AvatarReady               = 7,
    BasePoints                = 8,
    BaseCaptured              = 9,
    TeamKiller                = 10,
    VehicleUpdated            = 11,
    CombatEquipmentUsed       = 12,
    FlagTeams                 = 16,
    FlagStateChanged          = 17,
    InteractiveStats          = 19,
    ResourcePointStateChanged = 21,
    OwnVehicleInsideRp        = 22,
    OwnVehicleLockedForRp     = 23,
    SyncObjects               = 24,
    SyncObjectsDiff           = 25,
    ViewPoints                = 26,
    FogOfWar                  = 27,
    VehicleRecovered          = 28,
    RadarInfoReceived         = 29,
    Settings                  = 30,
    VehicleDescr              = 31,
    BasesList                 = 32,
    CommanderDataList         = 33,
    CommanderDataVehicle      = 34,
    GodModeChanged            = 35,
}
