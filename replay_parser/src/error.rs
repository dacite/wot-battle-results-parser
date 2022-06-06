use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("given packet's size did not match its expected size")]
    InvalidPacket,

    #[error("cannot read replay file")]
    ReplayFileError,

    #[error("error reading replay json: {0}")]
    ReplayJsonError(String),

    #[error("cannot find definitions: {0}")]
    DefinitionFileError(String),

    /// Error when parsing with nom. Only holds information about what type of nom error.
    /// More info is added and this error will turn to one of the other variants and this error gets bubbled
    /// up
    #[error("nom: {0}")]
    NomParseError(String),

    #[error("nom error with serde: {nom_error} for {info}")]
    NomSerdeParseError { nom_error: String, info: String },

    #[error("serde packet error: {0}")]
    SerdePacketError(String),

    #[error("entity method {method_name} parse failed: {root_cause}. given data: {method_data}")]
    EntityMethodError {
        method_data: String,
        method_name: String,
        root_cause:  String,
    },

    #[error("other error: {0}")]
    Other(String),

    #[error("json error: {0}")]
    JsonKeyError(&'static str),

    #[error("i/o error: {0}")]
    IoError(String),
}

impl Error {
    /// Construct an `EnityMethodError` given:
    /// - `data` data of the method
    /// - `id` expected id of the method
    /// - `name` name of the method
    /// - `root_cause` the underlying factor that caused the parsing to fail
    pub fn new_entity_method_err(data: &[u8], name: &str, root_cause: Error) -> Self {
        Self::EntityMethodError {
            method_data: hex::encode_upper(data),
            method_name: name.to_string(),
            root_cause:  root_cause.to_string(),
        }
    }
}

impl From<nom::Err<Error>> for Error {
    fn from(err: nom::Err<Error>) -> Self {
        match err {
            nom::Err::Incomplete(_) => Error::NomParseError("nom incomplete error".to_string()),
            nom::Err::Error(error) => error,
            nom::Err::Failure(error) => error,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ReplayJsonError(err.to_string())
    }
}

impl<T> nom::error::ParseError<T> for Error {
    fn from_error_kind(_: T, kind: nom::error::ErrorKind) -> Self {
        Error::NomParseError(kind.description().to_string())
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
