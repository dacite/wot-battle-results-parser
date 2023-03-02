use crate::AnyErr;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unknown Checksum for {0}: {1}")]
    UnknownChecksum(&'static str, i64),

    #[error("AccountCompDescrError: {0}")]
    AccountCompDescrError(#[source] AnyErr),

    #[error("ValueReplayError: {0}")]
    ValueReplayError(#[source] AnyErr),

    #[error("ManualParserError: {0}")]
    ManualParserError(#[source] AnyErr),

    #[error("Error during decompression")]
    DecompressionError,

    #[error("PickleError: {0}")]
    PickleError(#[from] serde_pickle::Error),

    #[error("OtherError: {0}")]
    OtherError(&'static str),

    #[error("Unexpected pickle format")]
    PickleFormatError,
}
