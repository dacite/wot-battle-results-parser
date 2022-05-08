use enum_as_inner::EnumAsInner;
use enum_dispatch::enum_dispatch;

use super::*; // Import all event types
use crate::packet_parser::Packet;
use crate::{BattleContext, Result};


/// This enum aims to represent all possible events that can occur in a battle. It's variant should map to
/// each packet type and is expected to always be that type. For ex., a `GameVersion` packet has type `0x18`
/// and is a variant of this enum. It is always be expected to be this type across all replays. Note that some
/// packet types like `0x08` may have children of its own. See `EntityMethodEvent` for more details.
#[derive(Debug, EnumAsInner, Clone)]
#[enum_dispatch(AsPacket, EventPrinter)]
#[non_exhaustive]
pub enum BattleEvent<'pkt> {
    Unimplemented(Unknown<'pkt>),
    GameVersion(GameVersion<'pkt>),
    EntityMethod(EntityMethodEvent<'pkt>),
}

/// Parse packet to a Battle event.
pub fn parse<'pkt>(packet: &'pkt Packet) -> Result<BattleEvent<'pkt>> {
    match packet.get_type() {
        0x18 => GameVersion::parse(packet),
        0x08 => EntityMethodEvent::parse(packet),
        _ => Unknown::parse(packet),
    }
}

/// This trait is implemented by all events so that they can parse a packet to a BattleEvent
#[enum_dispatch]
pub trait PacketParser<'a> {
    fn parse(packet: &'a Packet) -> Result<BattleEvent<'a>>;
}

/// Get the underlying packet representation of an event. Used to get the event's time, overall size or its
/// packet type
#[enum_dispatch]
pub trait AsPacket {
    fn as_packet(&self) -> &Packet;
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
#[enum_dispatch]
pub trait EventPrinter {
    fn to_debug_string(&self, context: &BattleContext) -> String
    where
        Self: std::fmt::Debug;
}
