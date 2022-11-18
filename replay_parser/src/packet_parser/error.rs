use std::{num::ParseIntError, str::Utf8Error, string::FromUtf8Error};

use utils::DataError;
#[derive(Debug, thiserror::Error, Clone)]
pub enum PacketError {
    #[error("{0}: incomplete input")]
    IncompleteInput(String),

    #[error("packet should be fully consumed after deserializing")]
    UnconsumedInput,

    #[error("nom error: {0}")]
    NomError(String),

    #[error("{0}")]
    StringUtf8Error(String),

    #[error("temporary error to catch incorrect usage of serde_packet deserializer")]
    IncorrectUsage,

    #[error("{0}")]
    ParseIntError(ParseIntError),

    #[error("{0}")]
    ConversionError(String),

    #[error("{0}")]
    DataError(String),

    #[error(
        "entity method {method_name}[ID: {method_id}] parse failed: {root_cause}. given data: {method_data}"
    )]
    EntityMethodError {
        method_data: String,
        method_id:   i32,
        method_name: String,
        root_cause:  String,
    },

    #[error("Expected variant: {0}")]
    WrongEnumVariant(String),

    #[error("Pickle error: {0}")]
    PickleError(String),
}


impl serde::de::Error for PacketError {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::IncorrectUsage
    }
}

impl From<nom::Err<PacketError>> for PacketError {
    fn from(err: nom::Err<PacketError>) -> Self {
        match err {
            nom::Err::Incomplete(_) => PacketError::IncompleteInput("nom".into()),
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        }
    }
}

impl<T> nom::error::ParseError<T> for PacketError {
    fn from_error_kind(_: T, kind: nom::error::ErrorKind) -> Self {
        match kind {
            nom::error::ErrorKind::Eof => PacketError::IncompleteInput("nom".into()),
            _ => PacketError::NomError(kind.description().to_string()),
        }
    }

    fn append(_: T, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<Utf8Error> for PacketError {
    fn from(err: Utf8Error) -> Self {
        PacketError::StringUtf8Error(err.to_string())
    }
}


impl From<DataError> for PacketError {
    fn from(err: DataError) -> Self {
        PacketError::DataError(err.to_string())
    }
}

impl From<FromUtf8Error> for PacketError {
    fn from(err: FromUtf8Error) -> Self {
        PacketError::StringUtf8Error(err.to_string())
    }
}

impl From<ParseIntError> for PacketError {
    fn from(err: ParseIntError) -> Self {
        PacketError::ParseIntError(err)
    }
}

impl From<serde_pickle::Error> for PacketError {
    fn from(err: serde_pickle::Error) -> Self {
        PacketError::PickleError(err.to_string())
    }
}
