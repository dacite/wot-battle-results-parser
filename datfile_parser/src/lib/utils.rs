use std::{
    fs::{self, DirEntry},
    path::Path,
};

use anyhow::{Context, Result};

/// A macro that tries to destructure an Enum to the given variant,
/// wrapped in a `Result`. Used to avoid using if let everywhere and have the
/// entire code shift right. Once if let chains stablize, this is probably not
/// needed.
macro_rules! try_variant {
    ($target: expr, $pattern: path) => {{
        if let $pattern(value) = $target {
            Ok(value)
        } else {
            Err(anyhow::anyhow!("Wrong variant. Expected: {}", stringify!($pattern)))
        }
    }};
}

pub(crate) use try_variant;

/// Parse a directory of .dat files (only direct childs of the directory)
pub fn parse_dir(path: &Path) -> Result<Vec<DirEntry>> {
    let file_paths = fs::read_dir(path).with_context(|| format!("failed to read dir"))?;

    Ok(file_paths
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .collect())
}
