use crate::events::*;
use crate::packet_parser::{Context, Packet, PacketError};
use crate::BattleContext;


/// This enum aims to represent all possible events that can occur in a battle. It's variant should map to
/// each packet type and is expected to always be that type. For ex., a `GameVersion` packet has type `0x18`
/// and is a variant of this enum. It is always be expected to be this type across all replays. Note that some
/// packet types like `0x08` may have children of its own. See `EntityMethodEvent` for more details.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    Unimplemented,
    GameVersion(GameVersion),
    AvatarCreate(AvatarCreate),
    EntityMethod(EntityMethodEvent),
    Position(Position),
    Chat(Chat),
}

impl Event {
    /// Parse packet to a Battle event. Optional context is provided to aid in parsing some particular
    /// packets.
    pub fn parse(packet: &Packet, context: &Context) -> Result<Event, PacketError> {
        match packet.get_type() {
            0x00 => AvatarCreate::parse(packet, context),
            0x0A => Position::parse(packet, &Context::default()),
            0x18 => GameVersion::parse(packet, &Context::default()),
            0x08 => EntityMethodEvent::parse(packet, context),
            0x23 => Chat::parse(packet, context),
            _ => Ok(Event::Unimplemented),
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Event::Unimplemented)
    }
}

/// This trait is implemented by all events so that they can parse a packet to a BattleEvent
pub trait PacketParser {
    fn parse(packet: &Packet, context: &Context) -> Result<Event, PacketError>;
}

/// Used for debugging purposes. Instead of the `Debug` trait (we don't have to choose. It is available as
/// well) because its useful for us to transform some values based on the `BattleContext`. For example, an
/// event may have an `attacker_id` attribute. We can transform that id to the actual player.
/// Right now, the following options are used:
///  - `#[event_debug(ignore)]` - ignore that field when printing the event out
///  - `#[event_debug(as_player)]` - transform this field's value to player if possible
///  - `#[event_debug(call_debug_string)]` - This field has its own implementation of `EventPrinter` so call
///    that
///
/// If no options, then `std::fmt::Debug` is called on that field
pub trait EventPrinter {
    fn to_debug_string(&self, context: &BattleContext) -> String
    where
        Self: std::fmt::Debug;
}

pub trait Version {
    fn name() -> &'static str;
    fn version() -> VersionInfo;
}

pub trait UpdateContext {
    fn update_context(&self, context: &mut Context);
}

impl EventPrinter for Event {
    fn to_debug_string(&self, context: &BattleContext) -> String
    where
        Self: std::fmt::Debug,
    {
        use Event::*;
        match self {
            Unimplemented => "Unimplemented".to_string(),
            AvatarCreate(x) => x.to_debug_string(context),
            GameVersion(x) => x.to_debug_string(context),
            EntityMethod(x) => x.to_debug_string(context),
            Position(x) => x.to_debug_string(context),
            Chat(x) => x.to_debug_string(context),
        }
    }
}

impl UpdateContext for Event {
    fn update_context(&self, context: &mut Context) {
        match self {
            Event::AvatarCreate(x) => x.update_context(context),
            Event::Unimplemented => {}
            Event::GameVersion(_) => {}
            Event::EntityMethod(_) => {}
            Event::Position(_) => {}
            Event::Chat(_) => {}
        }
    }
}


#[derive(Debug, Clone)]
pub enum VersionInfo {
    /// Present in all versions
    All,

    /// Present in this version
    Version([u16; 4]),

    /// Represent Versions of structs
    Struct(&'static [VersionInfo]),
}


///////////////////////////////////////////////////////////////////////////////////////////////////
/// All code related to the event stream
///////////////////////////////////////////////////////////////////////////////////////////////////
use super::PacketStream;
use crate::utils::validate_version;

pub struct EventStream<'pkt> {
    packet_stream: PacketStream<'pkt>,
    context:       Context,
}

impl<'pkt> EventStream<'pkt> {
    pub fn new(packet_stream: PacketStream<'pkt>, version: [u16; 4]) -> Result<Self, PacketError> {
        let version_validated = validate_version(version);
        let context = Context::new(version_validated)?;

        Ok(EventStream {
            packet_stream,
            context,
        })
    }
}

impl<'pkt> Iterator for EventStream<'pkt> {
    type Item = Result<Event, PacketError>;

    fn next(&mut self) -> Option<Self::Item> {
        let packet = self.packet_stream.next()?;
        match packet {
            Ok(packet) => {
                let packet_id = packet.id();
                let event = Event::parse(&packet, &self.context).map(|battle_event| {
                    battle_event.update_context(&mut self.context);

                    battle_event
                });

                log_if_error(packet_id, &event);
                Some(event)
            }
            Err(err) => Some(Err(err)),
        }
    }
}

fn log_if_error(packet_id: i32, event: &Result<Event, PacketError>) {
    match event.as_ref() {
        Ok(_) => {}
        Err(err) => {
            tracing::error!(packet_id, error = ?err)
        }
    }
}
