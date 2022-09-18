/// This can be considered the main child module of the events module. It specifies all the different traits
/// all types of events have to implement. `BattleEvent` can mean any event (desribed by the other child
/// modules) in the battle
mod battle_event;
pub use battle_event::parse;
pub use battle_event::BattleEvent;
pub use battle_event::Event;
pub use battle_event::EventPrinter;
pub use battle_event::PacketParser;
pub use battle_event::Version;
pub use battle_event::VersionInfo;

mod event_stream;
pub use event_stream::EventStream;

mod method_defs;

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

mod chat;
pub use chat::Chat;

#[derive(thiserror::Error, Debug, Clone)]
pub enum PacketDeserializeError {
    #[error("packet should be fully consumed after deserializing")]
    UnconsumedInput,

    #[error("packet data ended unexpectedly")]
    IncompleteInput,

    #[error("nom parser error: {0}")]
    NomParserError(String),

    #[error("temporary error to catch incorrect usage of serde_packet deserializer")]
    IncorrectUsage,

    #[error("nom error")]
    NomError,

    #[error("error deserializing string from packet")]
    StringDeError,
}

impl serde::de::Error for PacketDeserializeError {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::IncorrectUsage
    }
}


impl From<nom::Err<PacketDeserializeError>> for PacketDeserializeError {
    fn from(err: nom::Err<PacketDeserializeError>) -> Self {
        match err {
            nom::Err::Incomplete(_) => PacketDeserializeError::IncorrectUsage,
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        }
    }
}

impl<T> nom::error::ParseError<T> for PacketDeserializeError {
    fn from_error_kind(_: T, kind: nom::error::ErrorKind) -> Self {
        match kind {
            nom::error::ErrorKind::Eof => PacketDeserializeError::IncompleteInput,
            _ => PacketDeserializeError::NomParserError(kind.description().to_string()),
        }
    }

    fn append(_: T, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
