mod update_arena;
pub(crate) mod vehicle_methods;
// mod vehicle_misc_status;

use nom::number::complete::le_i32;
use update_arena::UpdateArena;
pub use vehicle_methods::*;

use crate::{packet_parser::prelude::*, BattleContext};

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
    method_id: i32,

    /// Houses details about the actual entity method that was called
    #[event_debug(custom_debug)]
    event: EntityMethod,
}

impl PacketParser for EntityMethodEvent {
    fn parse(packet: &Packet, context: &Context) -> Result<Event, PacketError> {
        let data = packet.payload();
        let (remaining, entity_id) = le_i32(data)?;
        let (remaining, method_id) = le_i32(remaining)?;
        let (method_data, size) = le_i32(remaining)?;

        if let Some(method_name) = context.find_method(entity_id, method_id) {
            let entity_method_event = EntityMethodEvent {
                entity_id,
                size,
                method_id,
                event: EntityMethod::new(method_name, method_data, context.get_version()).map_err(
                    |root_cause| PacketError::EntityMethodError {
                        method_data: hex::encode_upper(data),
                        method_name: method_name.into(),
                        method_id,
                        root_cause,
                    },
                )?,
            };

            Ok(Event::EntityMethod(entity_method_event))
        } else {
            let entity_method_event = EntityMethodEvent {
                entity_id,
                size,
                method_id,
                event: EntityMethod::Unknown(method_id),
            };

            Ok(Event::EntityMethod(entity_method_event))
        }
    }
}

impl EntityMethodEvent {
    /// Whether we understand the entity method
    pub fn is_unknown(&self) -> bool {
        matches!(self.event, EntityMethod::Unknown(_))
    }
}

impl From<EntityMethodEvent> for EntityMethod {
    fn from(val: EntityMethodEvent) -> Self {
        val.event
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
    pub fn new(name: &str, data: &[u8], version: [u16; 4]) -> std::result::Result<Self, String> {
        use EntityMethod::*;
        match name {
            "showShooting" => Ok(ShotFired(EntityMethod::parse_method(data, version)?)),
            "onHealthChanged" => Ok(HealthChanged(EntityMethod::parse_method(data, version)?)),
            "showDamageFromExplosion" => Ok(Explosion(EntityMethod::parse_method(data, version)?)),
            "onStaticCollision" => Ok(StaticCollision(EntityMethod::parse_method(data, version)?)),
            "showDamageFromShot" => Ok(DamageFromShot(EntityMethod::parse_method(data, version)?)),
            "showTracer" => Ok(Tracer(EntityMethod::parse_method(data, version)?)),
            // "updateArena" => Ok(Arena(UpdateArena::from(data, version).unwrap())),
            _ => {
                // eprintln!("{name}");

                Ok(Unknown(-1))
            }
        }
    }

    /// We move the parsing logic needed to create `EntityMethod` to its own function because with `map_err`
    /// it gets messy.
    fn parse_method<'de, T: Deserialize<'de> + Version>(
        data: &'de [u8], version: [u16; 4],
    ) -> std::result::Result<T, String> {
        from_slice(data, version).map_err(|err| err.to_string())
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
