use nom::number::complete::le_u8;

use crate::packet_parser::prelude::*;
use crate::packet_parser::serde_packet;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateArena {
    pub update_type: u8,
}

impl UpdateArena {
    pub fn from(data: &[u8], _version: [u16; 4]) -> Result<Self, PacketError> {
        let (remaining, update_type) = le_u8(data)?;
        let (_remaining, _arena_data) = serde_packet::parse_byte_array(remaining)?;

        // parse_arena_data(update_type, arena_data).unwrap();

        Ok(UpdateArena { update_type })
    }
}

// fn parse_arena_data(update_type: u8, arena_data: &[u8]) -> Result<(), PacketError> {
//     if update_type == ArenaUpdate::VehicleList as u8 {
//         // let decompressed = utils::decompress_vec(arena_data).unwrap();
//         // let pickle_value = serde_pickle::value_from_slice(
//         //     &decompressed,
//         //     serde_pickle::DeOptions::new().replace_unresolved_globals(),
//         // )
//         // .unwrap();

//         // // println!("{:#?}", pickle_value);

//         // let wot_value = serde_pickle::from_value(pickle_value).unwrap();
//         // let wot_value = try_variant!(wot_value, WotValue::Collection).unwrap();
//         // let wot_value = try_variant!(wot_value[0].clone(), WotValue::Collection).unwrap();
//         println!("{:#?}", wot_value[1]);
//         // let wot_value = try_variant!(wot_value[1].clone(), WotValue::Bytes).unwrap();
//         todo!()
//     }

//     Ok(())
// }

// fn get_vehicle_type(compact_description: Vec<u8>, extra_data: Option<i32>) {}
// fn split_vehicle_compact_descr(compact_description: Vec<u8>) {
//     let header = *compact_description.get(0).unwrap();
//     let vehicle_type_id = *compact_description.get(1).unwrap();
//     let nation_id = header >> 4 & 15;
// }
// // TODO: MOVE
// enum ArenaUpdate {
//     VehicleList               = 1,
//     VehicleAdded              = 2,
//     Period                    = 3,
//     Statistics                = 4,
//     VehicleStatistics         = 5,
//     VehicleKilled             = 6,
//     AvatarReady               = 7,
//     BasePoints                = 8,
//     BaseCaptured              = 9,
//     TeamKiller                = 10,
//     VehicleUpdated            = 11,
//     CombatEquipmentUsed       = 12,
//     FlagTeams                 = 16,
//     FlagStateChanged          = 17,
//     InteractiveStats          = 19,
//     ResourcePointStateChanged = 21,
//     OwnVehicleInsideRp        = 22,
//     OwnVehicleLockedForRp     = 23,
//     SyncObjects               = 24,
//     SyncObjectsDiff           = 25,
//     ViewPoints                = 26,
//     FogOfWar                  = 27,
//     VehicleRecovered          = 28,
//     RadarInfoReceived         = 29,
//     Settings                  = 30,
//     VehicleDescr              = 31,
//     BasesList                 = 32,
//     CommanderDataList         = 33,
//     CommanderDataVehicle      = 34,
//     GodModeChanged            = 35,
// }
