/// An incomplete implementation of serde Deserializer for some packets found in World of Tanks Replays.
///
/// All it achieves is instead of writing code like this:
/// ```ignore
/// let x = input.read_f32::<LE>().unwrap();
/// let z = input.read_f32::<LE>().unwrap();
/// let y = input.read_f32::<LE>().unwrap();
/// ```
///
/// We can write this:
/// ```ignore
/// struct Position {
///     x: f32, z: f32, y: f32
/// }
///
/// let position_packet: Position = serde_packet::from_slice(input).unwrap();
/// ```
/// This way `serde` takes care of the tedious byte to byte parsing (powered by `nom`).
///
/// Deserializer assumes that the input will match the data format. This means that a packet's payload must be
/// truncated to match the data.
///
/// A replay packet contains the following :
///     - Metadata - (packet type, payload size, timestamp)
///     - Payload
///
/// We can further divide the Payload depending on the packet type.
///
///
/// For example if we have the following packet (type is `0x08`) payload:
/// `008F2F48 00000000 00000002 0001` the input is only actually `0001` because it has the following format:
///     - `008F2F48` Object/Entity ID (Always part of the 0x08)
///     - `00000000` For packet `0x08`, this is known as the `messageID` inside BigWorld (the game engine WoT
///       for stuff that does not include graphics). For us, it tell us which method of the entity is being
///       called.
///     In this particular case, the `messageID` is 0 and refers to `showShooting()` method on the entity (the
/// entity is     `Vehicle` for this ex.)
///     - `00000002` tells us how many bytes of information is left. In this case this is `2` bytes
///     - `0001` is the interesting part: the input for serde. In this particular case it is simply two values
///       of u8s
mod serde_packet;
pub use serde_packet::from_slice;

/// Contains code for all the different types of events. `battle_event` module is like entry point as it
/// contains the main parsing code. For each packet, we have an event. An event can be considered the human
/// readable abstraction over a packet.
pub mod events;
pub use events::parse;

/// Contains `Packet`, and `PacketStream`. A light zero-copy wrapper for the binary data from
/// `.wotreplay`. These are then used by the `events` module to parse into events that we can understand.
mod packet;
pub use packet::Packet;
pub use packet::PacketStream;
