use std::{
    fs::{self, DirEntry},
    path::Path,
};

use anyhow::{Context, Result};

/// A macro that tries to destructure an Enum to the given variant,
/// wrapped in a `Result`. Used to avoid using if let everywhere and have the
/// entire code shift right. Once if let chains stablize, this is probably not
/// needed.
#[macro_export]
macro_rules! try_variant {
    ($target: expr, $pattern: path) => {{
        if let $pattern(value) = $target {
            Ok(value)
        } else {
            Err(anyhow::anyhow!("Wrong variant. Expected: {}", stringify!($pattern)))
        }
    }};
}


/// Get files in directory, given directory path (only direct childs of the directory)
pub fn parse_dir(path: &Path) -> Result<Vec<DirEntry>> {
    let file_paths = fs::read_dir(path).with_context(|| format!("failed to read dir"))?;

    Ok(file_paths
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .collect())
}

// TODO: Make its own file
#[derive(thiserror::Error, Debug)]
pub struct NomErrorWrapper {
    pub kind:  nom::error::ErrorKind,
    backtrace: Vec<nom::error::ErrorKind>,
    _input:    Vec<u8>,
}

impl nom::error::ParseError<&[u8]> for NomErrorWrapper {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        Self {
            kind:      kind,
            backtrace: Vec::new(),
            _input:    input.to_vec(),
        }
    }

    fn append(input: &[u8], kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.backtrace.push(Self::from_error_kind(input, kind).kind);

        other
    }
}

impl std::fmt::Display for NomErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nom error: {} error", self.kind.description())
    }
}
