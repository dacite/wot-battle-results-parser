pub(crate) mod avatar_methods;
pub(crate) mod vehicle_methods;
// mod vehicle_misc_status;

pub use avatar_methods::update_arena::*;
use nom::number::complete::le_i32;
pub use vehicle_methods::*;

use crate::packet_parser::prelude::*;

/// Represents all packets of type `0x08`. `0x08` packet seems to describe a method call on an entity.
/// Refers to multiple types of events (depending on which method was called on the entity).
#[derive(Debug, Clone, EventPrinter, Serialize)]
pub struct EntityMethodEvent {
    /// ID of the entity who called this method
    #[event_debug(as_player)]
    pub entity_id: i32,

    /// Total size of the method arguments. i.e the size of the actual payload `(packet_payload =
    /// (payload_info_size + actual_payload_size))`
    pub size: i32,

    /// Method ID associated with this method. This ID could be different for the same method if the replay
    /// versions are different
    pub method_id: i32,

    /// What we think the method is for right now..
    pub method_name: Option<String>,

    /// Houses details about the actual entity method that was called
    #[event_debug(custom_debug)]
    pub event: EntityMethod,
}

impl PacketParser for EntityMethodEvent {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let data = packet.payload();
        let (remaining, entity_id) = le_i32(data)?;
        let (remaining, method_id) = le_i32(remaining)?;
        let (method_data, size) = le_i32(remaining)?;

        if let Some(method_name) = context.find_method(entity_id, method_id) {
            let entity_method_event = EntityMethodEvent {
                entity_id,
                size,
                method_id,
                method_name: Some(method_name.into()),
                event: EntityMethod::new(method_name, method_data, context.get_version()).map_err(
                    |root_cause| PacketError::EntityMethodError {
                        method_data: hex::encode_upper(data),
                        method_name: method_name.into(),
                        method_id,
                        root_cause: root_cause.to_string(),
                    },
                )?,
            };

            Ok(BattleEvent::EntityMethod(entity_method_event))
        } else {
            let entity_method_event = EntityMethodEvent {
                entity_id,
                size,
                method_id,
                method_name: None,
                event: EntityMethod::Unknown(method_id),
            };

            Ok(BattleEvent::EntityMethod(entity_method_event))
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
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub enum EntityMethod {
    DamageFromShot(ShowDamageFromShot),
    ShotFired(ShowShooting),
    HealthChanged(OnHealthChanged),
    Explosion(ShowDamageFromExplosion),
    StaticCollision(OnStaticCollision),
    Tracer(ShowTracer),
    Arena(UpdateArena),
    NotImplemented(String),
    Unknown(i32),
}

impl EntityMethod {
    pub fn new(name: &str, data: &[u8], version: [u16; 4]) -> std::result::Result<Self, PacketError> {
        use EntityMethod::*;
        match name {
            "showShooting" => Ok(ShotFired(from_slice(data, version)?)),
            "onHealthChanged" => Ok(HealthChanged(from_slice(data, version)?)),
            "showDamageFromExplosion" => Ok(Explosion(from_slice(data, version)?)),
            "onStaticCollision" => Ok(StaticCollision(from_slice(data, version)?)),
            "showDamageFromShot" => Ok(DamageFromShot(from_slice(data, version)?)),
            "showTracer" => Ok(Tracer(from_slice(data, version)?)),
            "updateArena" => Ok(Arena(UpdateArena::from(data, version)?)),
            _ => {
                // eprintln!("{name}");

                Ok(NotImplemented(name.into()))
            }
        }
    }
}

impl EventPrinter for EntityMethod {
    fn to_debug_string(&self, context: &Context) -> String
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
            EntityMethod::NotImplemented(method) => format!("Not implemented: {method}"),
        }
    }
}
