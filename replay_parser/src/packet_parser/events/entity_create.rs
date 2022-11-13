use nom::{bytes::complete::take, number::complete::le_u32};

use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct EntityCreate {
    pub entity_id:   i32,
    pub entity_type: i16,

    #[version([0, 9, 14, 0])]
    pub vehicle_id: Option<i32>,

    pub space_id:  i32,
    pub unknown:   i32,
    pub position:  Vector3,
    pub direction: Vector3,
}

impl PacketParser for EntityCreate {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (remaining, entity_create) = from_slice_unchecked::<EntityCreate>(data, context.get_version())?;
        let (remaining, size) = le_u32(remaining)?;
        let (remaining, _data) = take(size)(remaining)?; // TODO: We will ignore the data for now...

        if !remaining.is_empty() {
            Err(PacketError::UnconsumedInput)
        } else {
            Ok(BattleEvent::EntityCreate(entity_create))
        }
    }
}
