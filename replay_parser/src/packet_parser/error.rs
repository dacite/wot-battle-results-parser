#[derive(Debug, thiserror::Error, Clone)]
pub enum PacketError {
    #[error("Packet stream is corrupted")]
    PacketStreamError,

    #[error("{0}")]
    PacketDeserializeError(PacketDeserializeError),
}

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