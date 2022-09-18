include!(concat!(env!("OUT_DIR"), "/method_map_codegen.rs"));

pub fn find_method(entity_name: &str, version_str: &str, method_id: i32) -> Option<&'static str> {
    let key = format!("{entity_name} {version_str} {method_id}");

    METHOD_MAP.get(&key).copied()
}
