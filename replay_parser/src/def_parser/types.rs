use std::collections::HashMap;

use roxmltree::{Document, Node as XMLNode};

use super::utils::{self, get_definitions_root};
use crate::Result;

/// Type information for types found in the alias.xml and .def files.
#[derive(Debug, Clone)]
pub enum WotType {
    OpaqueType(OpaqueType),
    Array(Box<WotType>),
    FixedDict {
        is_nullable: bool,
        dict:        HashMap<String, WotType>,
    },
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
    Vector4, // is this actually present in def files?
    Pickle,
    MailBox,
    UserType,
    Alias(Box<WotType>),
}

#[derive(Debug, Default)]
pub struct TypeAliasLookup {
    dict: HashMap<String, WotType>,
}


impl TypeAliasLookup {
    pub fn load(version: [u16; 4]) -> Result<Self> {
        let def_root = get_definitions_root();
        let path = format!("{def_root}/{}/alias.xml", utils::version_as_string(version));

        let xml_string = utils::read_xml(path)?;
        let document = Document::parse(&xml_string).unwrap();
        let root = document.root().first_child().unwrap();

        let mut type_alias_dict = TypeAliasLookup { dict: HashMap::new() };
        for node in root.children().filter(XMLNode::is_element) {
            type_alias_dict.parse_type_alias(&node).unwrap();
        }

        Ok(type_alias_dict)
    }

    fn parse_type_alias(&mut self, node: &XMLNode) -> Result<()> {
        let type_name = node.tag_name().name().to_string();
        let ty = node.text().unwrap();

        if ty.contains("FIXED_DICT") {
            let dict = self.parse_dict_type(node);

            let mut is_nullable = false;
            if let Some(allow_none) = utils::select_child("AllowNone", node) {
                if let Some(text) = allow_none.text() {
                    is_nullable = text.contains("true");
                }
            }
            if self
                .dict
                .insert(type_name, WotType::FixedDict { is_nullable, dict })
                .is_some()
            {
                panic!("Overwrote type alias");
            }
        } else if self.dict.insert(type_name, self.parse_type(node)?).is_some() {
            panic!("Overwrote type alias");
        }

        Ok(())
    }

    fn parse_dict_type(&mut self, node: &XMLNode) -> HashMap<String, WotType> {
        let properties = utils::select_child("Properties", node).unwrap();

        self.parse_properties(&properties).unwrap()
    }

    fn parse_properties(&mut self, node: &XMLNode) -> Result<HashMap<String, WotType>> {
        let mut dict = HashMap::new();

        for child in node.children().filter(XMLNode::is_element) {
            let name = child.tag_name().name().to_string();
            let ty = utils::select_child("Type", &child).unwrap();

            dict.insert(name, self.parse_type(&ty)?);
        }

        Ok(dict)
    }

    /// Parse nodes like `<Type>    ARRAY      <of> INT32    </of> </Type>`
    pub fn parse_type(&self, node: &XMLNode) -> Result<WotType> {
        let ty = node.text().unwrap().trim();

        match ty {
            "ARRAY" | "TUPLE" => {
                let child_type = utils::select_child("of", node).unwrap();
                let child_type = self.parse_type(&child_type)?;

                Ok(WotType::Array(Box::new(child_type)))
            }
            _ => {
                let type_as_text = node.text().unwrap();
                let ty = type_from_str(type_as_text, self)?;

                Ok(WotType::OpaqueType(ty))
            }
        }
    }
}

/// Convert a str representation of a type to the Type enum. Utilizes a lookup table for getting the type
/// information for aliased types
fn type_from_str(s: &str, type_lookup: &TypeAliasLookup) -> Result<OpaqueType> {
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
        "USER_TYPE" => Ok(UserType),
        _ => {
            if let Some(alias) = type_lookup.dict.get(s) {
                Ok(Alias(Box::new(alias.clone())))
            } else {
                panic!("cannot find alias type: {}", s)
            }
        }
    }
}
