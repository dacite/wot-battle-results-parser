/// This can be considered the main child module of the events module. It specifies all the different traits
/// all types of events have to implement. `BattleEvent` can mean any event (desribed by the other child
/// modules) in the battle
mod battle_event;
pub use battle_event::BattleEvent;
pub use battle_event::EventPrinter;
pub use battle_event::PacketParser;
pub use battle_event::Version;
pub use battle_event::VersionInfo;

mod event_stream;
pub use event_stream::parse;
pub use event_stream::EventStream;
//////////////////////////////////////////////////////////////////////////////////////
/// Modules for different events
//////////////////////////////////////////////////////////////////////////////////////

/// `entity_method` describe multiple events because there can be many different types of method calls
mod entity_method;
pub use entity_method::EntityMethod;
pub use entity_method::EntityMethodEvent;
pub use entity_method::ShowDamageFromShot;
pub use entity_method::ShowShooting;

mod game_version;
pub use game_version::GameVersion;

mod avatar_create;
pub use avatar_create::AvatarCreate;
