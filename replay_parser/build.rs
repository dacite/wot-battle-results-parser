use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::rc::Rc;

use definition_parser::utils::{self, version_string_as_arr};
use definition_parser::{Entity, TypeAliasLookup};

fn main() {
    load_definitions();
}

pub fn load_definitions() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("method_map_codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = phf_codegen::Map::new();
    let versions = get_available_versions();

    for version in versions {
        load_version(version, &mut map);
    }

    writeln!(
        &mut file,
        "pub static METHOD_MAP: phf::Map<&'static str, &'static str> = \n{};\n",
        map.build()
    )
    .unwrap();
}

fn get_available_versions() -> Vec<[u16; 4]>{
    let dir = std::fs::read_dir("../definition_parser/definitions").unwrap();

    let mut vec = Vec::new();
    for dir_entry in dir.flatten() {
        let dir_name = dir_entry.file_name().to_string_lossy().to_string();
        let version = version_string_as_arr(dir_name).unwrap();

        vec.push(version);
    }

    vec
}
/// Load type aliases for this version and also the necessary entities
fn load_version(version: [u16; 4], map: &mut phf_codegen::Map<String>) {
    let type_alias = Rc::new(TypeAliasLookup::load(version).unwrap());

    add_entity_to_map("Avatar", version, type_alias.clone(), map);
    add_entity_to_map("Vehicle", version, type_alias, map);
}

pub fn add_entity_to_map(
    name: &str, version: [u16; 4], type_aliases: Rc<TypeAliasLookup>, map: &mut phf_codegen::Map<String>,
) {
    let entity = Entity::new(name, version, type_aliases).unwrap();

    for (method_id, method) in entity.client_methods.iter().enumerate() {
        let version = utils::version_as_string(version);
        let key = format!("{name} {version} {method_id}");
        let value = method.get_name();

        map.entry(key, &format!("\"{value}\""));
    }
}
