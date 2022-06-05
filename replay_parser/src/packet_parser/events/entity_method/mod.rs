mod update_arena;
mod vehicle_methods;


use macros::EventPrinter;
use nom::number::complete::le_i32;
use serde::{Deserialize, Serialize};
pub use vehicle_methods::*;

use self::update_arena::UpdateArena;
use super::{event_stream::Context, BattleEvent, EventPrinter, PacketParser, Version};
use crate::{
    packet_parser::{serde_packet, Packet},
    BattleContext, Error, Result,
};

/// Represents all packets of type `0x08`. `0x08` packet seems to describe a method call on an entity.
/// Refers to multiple types of events (depending on which method was called on the entity).
#[derive(Debug, Clone, EventPrinter)]
pub struct EntityMethodEvent {
    /// ID of the entity who called this method
    #[event_debug(as_player)]
    entity_id: i32,

    /// Total size of the method arguments. i.e the size of the actual payload `(packet_payload =
    /// (payload_info_size + actual_payload_size))`
    size: i32,

    /// Method ID associated with this method. This ID could be different for the same method if the replay
    /// versions are different
    method: i32,

    /// Houses details about the actual entity method that was called
    #[event_debug(custom_debug)]
    event: EntityMethod,
}

impl PacketParser for EntityMethodEvent {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent> {
        let data = packet.get_payload();
        let (remaining, entity_id) = le_i32(data)?;
        let (remaining, method_id) = le_i32(remaining)?;
        let (method_data, size) = le_i32(remaining)?;

        if let Some(method_name) = context.find_method(entity_id, method_id as usize) {
            let entity_method_event = EntityMethodEvent {
                entity_id,
                size,
                method: method_id,
                event: EntityMethod::new(method_name, method_data, context.get_version())?,
            };

            Ok(BattleEvent::EntityMethod(entity_method_event))
        } else {
            let entity_method_event = EntityMethodEvent {
                entity_id,
                size,
                method: method_id,
                event: EntityMethod::Unknown(method_id),
            };

            Ok(BattleEvent::EntityMethod(entity_method_event))
        }
    }
}


impl EntityMethodEvent {
    /// Whether we understand the entity method
    pub fn is_unknown(&self) -> bool {
        if let EntityMethod::Unknown(_) = self.event {
            true
        } else {
            false
        }
    }
}

impl Into<EntityMethod> for EntityMethodEvent {
    fn into(self) -> EntityMethod {
        self.event
    }
}

/// Enumerates all possible entity method calls (hopefully).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EntityMethod {
    DamageFromShot(ShowDamageFromShot),
    ShotFired(ShowShooting),
    HealthChanged(OnHealthChanged),
    Explosion(ShowDamageFromExplosion),
    StaticCollision(OnStaticCollision),
    Tracer(ShowTracer),
    Arena(UpdateArena),
    Unknown(i32),
}

impl EntityMethod {
    /// TODO: This is where the parsing gets difficult. For now, we keep match statement this way. However,
    /// the values are different depending on the replay version. To make this more general we will need
    /// to parse definition files or come up with another solution.
    pub fn new(name: &str, data: &[u8], version: [u16; 4]) -> Result<Self> {
        use EntityMethod::*;
        match name {
            "showShooting" => Ok(ShotFired(EntityMethod::parse_method(data, version)?)),
            "onHealthChanged" => Ok(HealthChanged(EntityMethod::parse_method(data, version)?)),
            "showDamageFromExplosion" => Ok(Explosion(EntityMethod::parse_method(data, version)?)),
            "onStaticCollision" => Ok(StaticCollision(EntityMethod::parse_method(data, version)?)),
            "showDamageFromShot" => Ok(DamageFromShot(EntityMethod::parse_method(data, version)?)),
            "showTracer" => Ok(Tracer(EntityMethod::parse_method(data, version)?)),
            "updateArena" => Ok(Arena(UpdateArena::from(data, version)?)),
            _ => Ok(Unknown(-1)),
        }
    }

    /// We move the parsing logic needed to create `EntityMethod` to its own function because with `map_err`
    /// it gets messy. We only really need `data` to parse the method; the rest of the args are used for
    /// decorating the error from `from_slice` with method information
    fn parse_method<'de, T: Deserialize<'de> + Version>(data: &'de [u8], version: [u16; 4]) -> Result<T> {
        serde_packet::from_slice(data, version)
            .map_err(|err| Error::new_entity_method_err(data, T::name(), err))
    }
}

impl EventPrinter for EntityMethod {
    fn to_debug_string(&self, context: &BattleContext) -> String
    where
        Self: std::fmt::Debug,
    {
        match self {
            EntityMethod::DamageFromShot(event) => format!("{:?}", event),
            EntityMethod::ShotFired(event) => format!("{:?}", event),
            EntityMethod::HealthChanged(event) => format!("{:?}", event.to_debug_string(context)),
            EntityMethod::Explosion(event) => format!("{:?}", event.to_debug_string(context)),
            EntityMethod::Tracer(event) => format!("{:?}", event.to_debug_string(context)),
            EntityMethod::StaticCollision(event) => format!("{:?}", event),
            EntityMethod::Arena(event) => format!("{:?}", event),
            EntityMethod::Unknown(method_id) => format!("Unknown method: {}", method_id),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3 {
    x: f32,
    z: f32,
    y: f32,
}
