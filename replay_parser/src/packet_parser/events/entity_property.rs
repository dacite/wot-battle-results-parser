use nom::number::complete::{le_i32, le_u32};
use serde::Deserializer;

use crate::entity_defs::{EntityProperty, EntityType, PropertyParser, VariantDeserializer};
use crate::packet_parser::{prelude::*, serde_packet};

#[derive(Debug, Clone, EventPrinter, Version, Deserialize, Serialize)]
pub struct EntityPropertyEvent {
    pub entity_type: EntityType,
    pub property:    Option<EntityProperty>,
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

        let mut d =
            serde_packet::Deserializer::from_slice(remaining, context.get_version(), VersionInfo::All, "");

        let version = utils::version_as_string(context.get_version());
        let entity_type = context.find_entity_type(entity_id).ok_or_else(|| {
            PacketError::NotFoundError(format!(
                "entity with id: {entity_id} not found for current replay context"
            ))
        })?;

        let discrim = entity_type
            .find_property(&version, property_id as usize)
            .map_err(PacketError::NotFoundError)?;

        let property =
            if let Some(discrim) = discrim {
                let property = parse_property(entity_type, remaining, context.get_version(), discrim)
                    .map_err(|err| PacketError::EntityPropertyError {
                        entity_type,
                        property: discrim.into(),
                        root_cause: err.to_string(),
                    })?;
                // if !d.is_empty() {
                //     return Err(PacketError::UnconsumedInput);
                // }
                property
            } else {
                None
            };


        Ok(BattleEvent::EntityProperty(EntityPropertyEvent {
            entity_type,
            property,
        }))
    }
}

pub fn parse_property(
    ent_type: EntityType, input: &[u8], version: [u16; 4], discrim: &'static str,
) -> Result<Option<EntityProperty>, PacketError> {
    use EntityProperty::*;
    use EntityType::*;

    let prop = match ent_type {
        // Avatar => Some(AvatarProperties(
        //     VariantDeserializer::deserialize_variant(discrim, d).map_err(err)?,
        // )),
        Vehicle => Some(VehicleProperties(PropertyParser::parse(input, version, discrim)?)),
        _ => None,
    };

    Ok(prop)
}
