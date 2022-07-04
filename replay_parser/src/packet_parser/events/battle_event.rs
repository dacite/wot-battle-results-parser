use super::event_stream::{Context, UpdateContext}; // Import all event types
use super::*;
use crate::packet_parser::Packet;
use crate::{BattleContext, Result};

#[derive(Debug)]
pub struct Event<'pkt> {
    packet: Packet<'pkt>,
    event:  BattleEvent,
}

impl<'pkt> Event<'pkt> {
    pub fn new(packet: Packet<'pkt>, event: BattleEvent) -> Self {
        Event { packet, event }
    }

    pub fn into_battle_event(self) -> BattleEvent {
        self.event
    }

    pub fn event(&self) -> &BattleEvent {
        &self.event
    }

    pub fn packet(&self) -> &Packet {
        &self.packet
    }

    pub fn is_unknown(&self) -> bool {
        self.event.is_unknown()
    }
}

/// This enum aims to represent all possible events that can occur in a battle. It's variant should map to
/// each packet type and is expected to always be that type. For ex., a `GameVersion` packet has type `0x18`
/// and is a variant of this enum. It is always be expected to be this type across all replays. Note that some
/// packet types like `0x08` may have children of its own. See `EntityMethodEvent` for more details.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum BattleEvent {
    Unimplemented,
    GameVersion(GameVersion),
    AvatarCreate(AvatarCreate),
    EntityMethod(EntityMethodEvent),
}

impl BattleEvent {
    pub fn is_unknown(&self) -> bool {
        matches!(self, BattleEvent::Unimplemented)
    }
}

impl EventPrinter for BattleEvent {
    fn to_debug_string(&self, context: &BattleContext) -> String
    where
        Self: std::fmt::Debug,
    {
        use BattleEvent::*;
        match self {
            Unimplemented => "Unimplemented".to_string(),
            AvatarCreate(x) => x.to_debug_string(context),
            GameVersion(x) => x.to_debug_string(context),
            EntityMethod(x) => x.to_debug_string(context),
        }
    }
}

impl UpdateContext for BattleEvent {
    fn update_context(&self, context: &mut Context) {
        match self {
            BattleEvent::Unimplemented => {}
            BattleEvent::GameVersion(_) => {}
            BattleEvent::AvatarCreate(x) => x.update_context(context),
            BattleEvent::EntityMethod(_) => {}
        }
    }
}
/// This trait is implemented by all events so that they can parse a packet to a BattleEvent
pub trait PacketParser {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent>;
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

#[derive(Debug, Clone)]
pub enum VersionInfo {
    /// Present in all versions
    All,

    /// Present in this version
    Version([u16; 4]),

    /// Represent Versions of structs
    Struct(&'static [VersionInfo]),
}
