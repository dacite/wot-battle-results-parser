use roxmltree::Node as XMLNode;

/// Often times, we expect a parent to have a particular child.
pub fn select_child<'a, 'b>(tag_name: &'static str, parent: &'a XMLNode<'a, 'b>) -> Option<XMLNode<'a, 'b>> {
    parent
        .children()
        .filter(XMLNode::is_element)
        .find(|child| child.tag_name().name() == tag_name)
}

pub fn get_definitions_root() -> String {
    std::env::var("DEF_DIR").unwrap_or_else(|_| "../definition_parser/definitions".to_string())
}
