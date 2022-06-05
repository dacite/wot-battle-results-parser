use std::path::Path;

use roxmltree::Node as XMLNode;

use crate::{Error, Result};


pub fn read_xml<'a, 'input, P: AsRef<Path>>(path: P) -> Result<String> {
    match std::fs::read_to_string(&path) {
        Ok(file) => Ok(file),
        Err(err) => Err(Error::DefinitionFileError(format!(
            "error reading {:?}. {}",
            path.as_ref(),
            err.to_string()
        ))),
    }
}

/// Often times, we expect a parent to have a particular child.
pub fn select_child<'a, 'b>(tag_name: &'static str, parent: &'a XMLNode<'a, 'b>) -> Option<XMLNode<'a, 'b>> {
    for child in parent.children().filter(XMLNode::is_element) {
        if child.tag_name().name() == tag_name {
            return Some(child);
        }
    }

    None
}

/// `[0, 9, 15, 0]` => `"0_9_15_0"`
pub fn version_as_string(version: [u16; 4]) -> String {
    version.map(|x| x.to_string()).join("_")
}

pub fn version_string_as_arr(version: String) -> Option<[u16; 4]> {
    let vec: Option<Vec<u16>> = version.split("_").map(|v| v.parse().ok()).collect();

    if let Some(vec) = vec {
        if vec.len() == 4 {
            Some([vec[0], vec[1], vec[2], vec[3]])
        } else {
            None
        }
    } else {
        None
    }
}

/// Validate this version by checking if we have definition files for this version. If not return version
/// closest to the input version
pub fn validate_version(version: [u16; 4]) -> Result<[u16; 4]> {
    let def_dir = get_definitions_root();
    let file_paths = std::fs::read_dir(def_dir).map_err(|e| Error::DefinitionFileError(e.to_string()))?;

    let dir_names: Vec<_> = file_paths
        .filter_map(|entry| {
            entry
                .ok()
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
        })
        .collect();

    let knwn_versions: Vec<_> = dir_names
        .into_iter()
        .map(|v| version_string_as_arr(v))
        .flatten()
        .collect();

    let mut smallest_diff = [u16::MAX, u16::MAX, u16::MAX, u16::MAX];
    let mut best_candidate = version;
    for curr_version in knwn_versions {
        let diff = [
            version[0].abs_diff(curr_version[0]),
            version[1].abs_diff(curr_version[1]),
            version[2].abs_diff(curr_version[2]),
            version[3].abs_diff(curr_version[3]),
        ];
        if smallest_diff > diff {
            best_candidate = curr_version;
            smallest_diff = diff;
        }
    }

    Ok(best_candidate)
}

pub fn get_definitions_root() -> String {
    std::env::var("DEF_DIR").unwrap_or("definitions".to_string())
}
