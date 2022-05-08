/// This can be considered the main child module of the events module. It specifies all the different traits
/// all types of events have to implement. `BattleEvent` can mean any event (desribed by the other child
/// modules) in the battle
mod battle_event;
pub use battle_event::parse;
pub use battle_event::AsPacket;
pub use battle_event::BattleEvent;
pub use battle_event::EventPrinter;
pub use battle_event::PacketParser;

//////////////////////////////////////////////////////////////////////////////////////
/// Modules for different events
//////////////////////////////////////////////////////////////////////////////////////

/// `entity_method` describe multiple events because there can be many different types of method calls
mod entity_method;
pub use entity_method::EntityMethodEvent;
pub use entity_method::ShowDamageFromShot;
pub use entity_method::ShowShooting;

mod game_version;
pub use game_version::GameVersion;

mod unknown;
pub use unknown::Unknown;
