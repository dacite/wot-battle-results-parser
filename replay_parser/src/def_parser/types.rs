use std::{collections::HashMap, path::Path};

use roxmltree::Node as XMLNode;
use wot_replay_parser::Result;

/// Represent an Aliased type found in the `alias.xml` file. These types are later used in the entity
/// definition files. For ex. there is an aliased type with `type_name = SHOT_ID` and `ty = I32`(int32)
#[derive(Debug, Clone)]
pub struct TypeAlias {
    type_name: String,
    ty:        WotType,
}


/// Represents the actual type value of an aliased type.
#[derive(Debug, Clone)]
pub enum WotType {
    OpaqueType(OpaqueType),
    Array(Box<WotType>),
    Dict(HashMap<String, WotType>),
}


/// These types are `Opaque` in the sense that it's the only info we represent. In contrast, `WotType`
/// represent dictionaries where it also tells us the types of the values in the dictionary. This is not
/// called primitive types because the variant `Alias` can represnt non-primitive types.
#[derive(Debug, Clone)]
pub enum OpaqueType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    String,
    Vector2,
    Vector3,
    Pickle,
    MailBox,
    Alias(String),
}


/// Load type aliases found in `alias.xml` file of a particular version of the game
pub fn load_type_aliases(version: [u16; 4]) -> Result<HashMap<String, TypeAlias>>{
    let path = format!(
        "replay_parser/definitions/{}/alias.xml",
        version_as_string(version)
    );

    let xml_string = read_xml(path).unwrap();
    let document = roxmltree::Document::parse(&xml_string).unwrap();

    let root = document.root().first_child().unwrap();
    let mut alias_dict = HashMap::new();
    for node in root.children().filter(XMLNode::is_element) {
        let alias = parse_type_alias(&node).unwrap();

        if let Some(_) = alias_dict.insert(alias.type_name.clone(), alias) {
            panic!("Overwrote type alias");
        }
    }

    Ok(alias_dict)
}

fn parse_type_alias(node: &XMLNode) -> Result<TypeAlias> {
    let type_name = node.tag_name().name().to_string();
    let ty = node.text().unwrap();

    if ty.contains("FIXED_DICT") {
        let dict = parse_dict_type(&node);

        Ok(TypeAlias {
            type_name,
            ty: WotType::Dict(dict),
        })
    } else {
        Ok(TypeAlias {
            type_name,
            ty: parse_type(node)?,
        })
    }
}

fn parse_dict_type(node: &XMLNode) -> HashMap<String, WotType> {
    let properties = select_child("Properties", node).unwrap();
    let dict = parse_properties(&properties).unwrap();

    dict
}

///
fn parse_properties(node: &XMLNode) -> Result<HashMap<String, WotType>> {
    let mut dict = HashMap::new();

    for child in node.children().filter(XMLNode::is_element) {
        let name = child.tag_name().name().to_string();
        let ty = select_child("Type", &child).unwrap();

        dict.insert(name, parse_type(&ty)?);
    }

    Ok(dict)
}

/// Parse nodes like `<Type>	ARRAY	  <of>	INT32	</of> </Type>`
fn parse_type(node: &XMLNode) -> Result<WotType> {
    let ty = node.text().unwrap().trim();

    match ty {
        "ARRAY" | "TUPLE" => {
            let child_type = select_child("of", node).unwrap();

            return Ok(WotType::Array(Box::new(parse_type(&child_type)?)));
        }
        _ => Ok(WotType::OpaqueType(type_from_str(node.text().unwrap())?)),
    }
}

/// Convert a str representation of a type to the Type enum.
fn type_from_str(s: &str) -> Result<OpaqueType> {
    use OpaqueType::*;

    let s = s.trim();
    match s {
        "UINT8" => Ok(U8),
        "INT8" => Ok(I8),
        "UINT16" => Ok(U16),
        "INT16" => Ok(I16),
        "UINT32" => Ok(U32),
        "INT32" => Ok(I32),
        "UINT64" => Ok(U64),
        "INT64" => Ok(I64),
        "FLOAT32" => Ok(F32),
        "FLOAT64" => Ok(F64),
        "FLOAT" => Ok(F32), // Order is important for floats,
        "STRING" => Ok(String),
        "VECTOR2" => Ok(Vector2),
        "VECTOR3" => Ok(Vector3),
        "PYTHON" => Ok(Pickle),
        "MAILBOX" => Ok(MailBox),
        _ => Ok(Alias(s.to_string())),
    }
}


/// Often times, we expect a parent to have a particular child.
fn select_child<'a, 'b>(tag_name: &'static str, parent: &'a XMLNode<'a, 'b>) -> Option<XMLNode<'a, 'b>> {
    for child in parent.children().filter(XMLNode::is_element) {
        if child.tag_name().name() == tag_name {
            return Some(child);
        }
    }

    None
}

fn read_xml<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

/// `[0, 9, 15, 0]` => `"0_9_15_0"`
fn version_as_string(version: [u16; 4]) -> String {
    version.map(|x| x.to_string()).join("_")
}
