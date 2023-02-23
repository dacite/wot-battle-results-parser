mod avatar_props;
mod vehicle_props;


#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum EntityProperty {
    NotImplemented {
        entity_type: EntityType,
        property_id: usize,
    },
    Vehicle(VehicleProperties),
    Avatar(AvatarProperties),
}


pub(crate) trait PropertyParser {
    fn parse(input: &[u8], property_id: usize, context: &Context) -> Result<EntityProperty, PacketError>
    where
        Self: Sized;
}
use nom::number::complete::{le_i32, le_u32};

use self::avatar_props::AvatarProperties;
use self::vehicle_props::VehicleProperties;
use crate::entity_defs::EntityType;
use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, EventPrinter, Version, Serialize)]
pub struct EntityPropertyEvent {
    pub entity_id: i32,
    pub property:  EntityProperty,
}

impl PacketParser for EntityPropertyEvent {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (remaining, entity_id) = le_i32(data)?;
        let (remaining, property_id) = le_i32(remaining)?;
        let (remaining, size) = le_u32(remaining)?;

        if remaining.len() != size as usize {
            return Err(PacketError::UnconsumedInput);
        }

        let entity_type = context.find_entity_type(entity_id)?;
        let property_id = property_id as usize;

        use EntityType::*;
        let property = match entity_type {
            Vehicle => VehicleProperties::parse(remaining, property_id, context),
            Avatar => AvatarProperties::parse(remaining, property_id, context),
            _ => Ok(EntityProperty::NotImplemented {
                entity_type,
                property_id,
            }),
        }?;

        Ok(BattleEvent::EntityProperty(EntityPropertyEvent {
            entity_id,
            property,
        }))
    }
}
