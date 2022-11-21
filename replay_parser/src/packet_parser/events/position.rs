use crate::packet_parser::prelude::*;

// Credit: https://github.com/Monstrofil/replays_unpack/blob/master/replay_unpack/core/packets/Position.py
#[derive(Debug, Clone, Deserialize, Version, EventPrinter, Serialize)]
pub struct Position {
    pub entity_id: i32,

    #[version(range([0, 9, 7, 0], [0, 9, 13, 0]))]
    pub space_id: Option<i32>,

    pub vehicle_id:     i32,
    pub position:       Vector3,
    pub position_error: Vector3,
    pub yaw:            f32,
    pub pitch:          f32,
    pub roll:           f32,
    pub is_volatile:    u8,
}

impl PacketParser for Position {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let position = from_slice(packet.payload(), context.get_version())?;

        Ok(BattleEvent::Position(position))
    }
}
