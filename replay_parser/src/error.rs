use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("given packet's size did not match its expected size")]
    InvalidPacket,

    #[error("nom: {nom_error} error for packet of type: {packet_type}")]
    PacketParseError { nom_error: String, packet_type: u32 },

    #[error("serde packet error: {0}")]
    SerdePacketError(String),

    #[error("other error: {0}")]
    Other(String),
}

impl<E: std::fmt::Display> From<nom::Err<E>> for Error {
    fn from(err: nom::Err<E>) -> Self {
        match err {
            nom::Err::Incomplete(_) => Error::PacketParseError {
                nom_error:   "nom incomplete error".to_string(),
                packet_type: 0xFFFFFFFF,
            },
            nom::Err::Error(e) => Error::PacketParseError {
                nom_error:   e.to_string(),
                packet_type: 0xFFFFFFFF,
            },
            nom::Err::Failure(e) => Error::PacketParseError {
                nom_error:   e.to_string(),
                packet_type: 0xFFFFFFFF,
            },
        }
    }
}

impl<T> nom::error::ParseError<T> for Error {
    fn from_error_kind(_: T, kind: nom::error::ErrorKind) -> Self {
        Error::PacketParseError {
            nom_error:   format!("nom: {} error", kind.description()),
            packet_type: 0xFFFFFFFF,
        }
    }

    fn append(_: T, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerdePacketError(msg.to_string())
    }
}
