pub(crate) mod avatar_methods;
pub(crate) mod vehicle_methods;
// mod vehicle_misc_status;

pub use avatar_methods::update_arena::*;
use nom::number::complete::le_i32;
pub use vehicle_methods::*;

use self::avatar_methods::AvatarMethods;
use crate::{entity_defs::EntityType, packet_parser::prelude::*};

/// Represents all packets of type `0x08`. `0x08` packet seems to describe a method call on an entity.
/// Refers to multiple types of events (depending on which method was called on the entity).
#[derive(Debug, Clone, EventPrinter, Serialize)]
pub struct EntityMethodEvent {
    /// ID of the entity who called this method
    #[event_debug(as_player)]
    pub entity_id: i32,

    pub method: EntityMethod,
}

impl PacketParser for EntityMethodEvent {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();

        let (remaining, entity_id) = le_i32(data)?;
        let (remaining, method_id) = le_i32(remaining)?;
        let (method_data, size) = le_i32(remaining)?;

        let entity_type = context.find_entity_type(entity_id)?;
        let method_id = method_id as usize;

        use EntityType::*;
        let method = match entity_type {
            Vehicle => VehicleMethods::parse(method_data, method_id, context),
            Avatar => AvatarMethods::parse(method_data, method_id, context),
            _ => Ok(EntityMethod::NotImplemented {
                entity_type,
                method_id,
            }),
        }?;

        Ok(BattleEvent::EntityMethod(EntityMethodEvent { entity_id, method }))
    }
}

impl EntityMethodEvent {
    /// Whether we understand the entity method
    pub fn is_unknown(&self) -> bool {
        matches!(self.method, EntityMethod::NotImplemented { .. })
    }
}

impl From<EntityMethodEvent> for EntityMethod {
    fn from(val: EntityMethodEvent) -> Self {
        val.method
    }
}

/// Enumerates all possible entity method calls (hopefully).
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum EntityMethod {
    NotImplemented {
        entity_type: EntityType,
        method_id:   usize,
    },
    Vehicle(VehicleMethods),
    Avatar(AvatarMethods),
}

impl EntityMethod {
    pub fn new(
        entity_type: EntityType, input: &[u8], context: &Context, method_id: usize,
    ) -> std::result::Result<Self, PacketError> {
        use EntityType::*;

        match entity_type {
            Vehicle => VehicleMethods::parse(input, method_id, context),
            Avatar => AvatarMethods::parse(input, method_id, context),
            _ => Ok(EntityMethod::NotImplemented {
                entity_type,
                method_id,
            }),
        }
    }
}


pub trait MethodParser {
    fn parse(input: &[u8], method_id: usize, context: &Context) -> Result<super::EntityMethod, PacketError>
    where
        Self: Sized;
}
