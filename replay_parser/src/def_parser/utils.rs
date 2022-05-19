use std::path::Path;

use roxmltree::Node as XMLNode;
use wot_replay_parser::Result;


pub fn read_xml<'a, 'input, P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
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
