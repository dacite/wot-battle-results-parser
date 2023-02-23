use nom::{bytes::complete::take, number::complete::le_u32};

use crate::{entity_defs::find_entity_type, packet_parser::prelude::*};

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct EntityCreate {
    pub entity_id:      i32,
    pub entity_type_id: u16,

    #[version([0, 9, 14, 0])]
    pub vehicle_id: Option<i32>,

    pub space_id:  i32,
    pub unknown:   i32,
    pub position:  Vector3,
    pub direction: Vector3,
}

impl PacketParser for EntityCreate {
    fn parse_mut(packet: &Packet, context: &mut Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (remaining, entity_create) = from_slice_unchecked::<EntityCreate>(data, context.get_version())?;
        let (remaining, size) = le_u32(remaining)?;
        let (remaining, _data) = take(size)(remaining)?; // TODO: We will ignore the data for now...

        let key = utils::version_as_string(context.get_version());
        let entity_type_id = entity_create.entity_type_id;

        let entity_type =
            find_entity_type(&key, entity_type_id as usize).ok_or_else(|| PacketError::NotFoundError {
                err: format!("entity type with id: {entity_type_id} not found for version: {key}"),
            })?;

        context.add_entity(entity_create.entity_id, entity_type);

        if !remaining.is_empty() {
            Err(PacketError::UnconsumedInput)
        } else {
            Ok(BattleEvent::EntityCreate(entity_create))
        }
    }
}
