use crate::packet_parser::PacketError;

#[derive(thiserror::Error, Debug)]
pub enum ReplayError {
    #[error("cannot read replay file")]
    ReplayFileError,

    #[error("error deserializing json: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("cannot find definitions: {0}")]
    XmlFileError(String),

    #[error("replay has unexpected json format: {0}")]
    ReplayJsonFormatError(String),

    /// Error when parsing with nom. Only holds information about what type of nom error.
    /// More info is added and this error will turn to one of the other variants and this error gets bubbled
    /// up
    #[error("nom: {0}")]
    NomParseError(String),

    #[error("serde packet error: {0}")]
    SerdePacketError(String),

    #[error("other error: {0}")]
    Other(String),

    #[error("json path error: {0}")]
    JsonPathError(&'static str),

    #[error("json type error: {0}")]
    JsonTypeError(String),

    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unable to find the arena unique id")]
    MissingArenaUniqueId,

    #[error("Packet stream is corrupted")]
    PacketStreamError,

    #[error("packet parse error: packet id: {packet_id} type: {packet_type} error: {error}")]
    PacketParseError {
        packet_id:   i32,
        packet_type: u32,
        error:       PacketError,
    },
}

impl From<nom::Err<ReplayError>> for ReplayError {
    fn from(err: nom::Err<ReplayError>) -> Self {
        match err {
            nom::Err::Incomplete(_) => ReplayError::NomParseError("nom incomplete error".to_string()),
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        }
    }
}

impl<T> nom::error::ParseError<T> for ReplayError {
    fn from_error_kind(_: T, kind: nom::error::ErrorKind) -> Self {
        ReplayError::NomParseError(kind.description().to_string())
    }

    fn append(_: T, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl serde::de::Error for ReplayError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerdePacketError(msg.to_string())
    }
}
