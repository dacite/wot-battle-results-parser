use std::{num::ParseIntError, str::Utf8Error, string::FromUtf8Error};

use snafu::prelude::*;

use crate::entity_defs::EntityType;
#[derive(Debug, Snafu, Clone)]
pub enum PacketError {
    #[snafu(display("{err}: incomplete input"))]
    IncompleteInput { err: String },

    #[snafu(display("packet should be fully consumed after deserializing"))]
    UnconsumedInput,

    #[snafu(display("nom error: {err}"))]
    NomError { err: String },

    #[snafu(display("{err}"))]
    StringUtf8Error { err: String },

    #[snafu(display("temporary error to catch incorrect usage of serde_packet deserializer"))]
    IncorrectUsage,

    #[snafu(display("{err}"))]
    ParseIntError { err: ParseIntError },

    #[snafu(display("{err}"))]
    ConversionError { err: String },

    #[snafu(display("{err}"))]
    DataError { err: String },

    #[snafu(display("entity_type={entity_type}, method={method} root_cause={root_cause}"))]
    EntityMethodError {
        entity_type: EntityType,
        method:      &'static str,
        root_cause:  String,
    },

    #[snafu(display("entity_type={entity_type}, property={property} root_cause={root_cause}"))]
    EntityPropertyError {
        entity_type: EntityType,
        property:    &'static str,
        root_cause:  String,
    },

    #[snafu(display("Expected variant: {err}"))]
    WrongEnumVariant { err: String },

    #[snafu(display("Pickle error: {err}"))]
    PickleError { err: String },

    #[snafu(display("Deserialize error: {err}"))]
    DeserializeError { err: String },

    #[snafu(display("Not found: {err}"))]
    NotFoundError { err: String },

    #[snafu(display("size marker says {expected} bytes but {actual} bytes remaining"))]
    IncorrectSizeMarker { expected: usize, actual: usize },
}

impl PacketError {
    pub fn incorrect_size(expected: usize, actual: usize) -> Self {
        Self::IncorrectSizeMarker { expected, actual }
    }

    pub fn entity_prop_err(entity_type: EntityType, property: &'static str, root_cause: String) -> Self {
        Self::EntityPropertyError {
            entity_type,
            property,
            root_cause,
        }
    }
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
            nom::Err::Incomplete(_) => PacketError::IncompleteInput { err: "nom".into() },
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        }
    }
}

impl<T> nom::error::ParseError<T> for PacketError {
    fn from_error_kind(_: T, kind: nom::error::ErrorKind) -> Self {
        match kind {
            nom::error::ErrorKind::Eof => PacketError::IncompleteInput { err: "nom".into() },
            _ => PacketError::NomError {
                err: kind.description().to_string(),
            },
        }
    }

    fn append(_: T, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<Utf8Error> for PacketError {
    fn from(err: Utf8Error) -> Self {
        PacketError::StringUtf8Error { err: err.to_string() }
    }
}


impl From<FromUtf8Error> for PacketError {
    fn from(err: FromUtf8Error) -> Self {
        PacketError::StringUtf8Error { err: err.to_string() }
    }
}

impl From<ParseIntError> for PacketError {
    fn from(err: ParseIntError) -> Self {
        PacketError::ParseIntError { err }
    }
}

impl From<serde_pickle::Error> for PacketError {
    fn from(err: serde_pickle::Error) -> Self {
        PacketError::PickleError { err: err.to_string() }
    }
}
