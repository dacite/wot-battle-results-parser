use std::rc::Rc;

use roxmltree::Document;
use roxmltree::Node as XMLNode;

use super::types::WotType;
use super::utils::{self, select_child};
use super::{Size, TypeAliasLookup};
use crate::Error;
use crate::Result;

#[derive(Debug)]
pub struct Entity {
    _name:        String,
    version:      [u16; 4],
    type_aliases: Rc<TypeAliasLookup>,

    _volatiles: Vec<Property>,
    properties: Vec<Property>,

    client_methods: Vec<Method>,
    cell_methods:   Vec<Method>,
    base_methods:   Vec<Method>,
}

#[derive(Debug)]
pub struct Property {
    _name: String,
    _ty:   WotType,
    _flag: String,
}

#[derive(Debug, Clone)]
pub struct Method {
    name:                 String,
    params:               Vec<WotType>,
    variable_header_size: Option<u8>,
}

impl Method {
    pub fn get_params(&self) -> &[WotType] {
        &self.params
    }

    pub fn get_variable_header_size(&self) -> Option<u8> {
        self.variable_header_size
    }
}

impl Entity {
    pub fn new(name: &str, version: [u16; 4], type_aliases: Rc<TypeAliasLookup>) -> Result<Self> {
        let mut entity = Entity {
            _name: name.to_string(),
            version,
            type_aliases,
            _volatiles: Vec::new(),
            properties: Vec::new(),
            client_methods: Vec::new(),
            cell_methods: Vec::new(),
            base_methods: Vec::new(),
        };
        entity.from_def_file(get_def_file_path(version, name, false))?;
        entity
            .client_methods
            .sort_by(|a, b| a.get_size().cmp(&b.get_size()));

        Ok(entity)
    }

    pub fn find_method(&self, method_id: usize) -> Option<&str> {
        let method = self.client_methods.get(method_id)?;

        Some(&method.name)
    }

    fn from_def_file(&mut self, path: String) -> Result<()> {
        let xml_string = utils::read_xml(path).map_err(|e| Error::DefinitionFileError(e.to_string()))?;
        let document = Document::parse(&xml_string).unwrap();
        let root = document.root().first_child().unwrap();

        if let Some(implements) = utils::select_child("Implements", &root) {
            parse_interfaces(self, implements)?;
        }

        // let volatiles = utils::select_child("Volatile", &root).unwrap();
        if let Some(properties) = utils::select_child("Properties", &root) {
            parse_properties(self, properties)?;
        }

        if let Some(client_methods) = utils::select_child("ClientMethods", &root) {
            let mut client_methods = parse_methods(client_methods, self.type_aliases.as_ref())?;
            self.client_methods.append(&mut client_methods);
        }

        if let Some(cell_methods) = utils::select_child("CellMethods", &root) {
            let mut cell_methods = parse_methods(cell_methods, self.type_aliases.as_ref())?;

            self.cell_methods.append(&mut cell_methods);
        }

        if let Some(base_methods) = utils::select_child("BaseMethods", &root) {
            let mut base_methods = parse_methods(base_methods, self.type_aliases.as_ref())?;

            self.base_methods.append(&mut base_methods);
        }

        Ok(())
    }
}

fn parse_interfaces(entity: &mut Entity, node: XMLNode) -> Result<()> {
    for child in node.children().filter(XMLNode::is_element) {
        if is_interface(&child) {
            let interface_path = get_def_file_path(entity.version, get_interface_name(&child), true);

            entity.from_def_file(interface_path)?;
        } else {
            panic!("<Implements> may only contai <Interfaces>");
        }
    }

    Ok(())
}

fn parse_properties(entity: &mut Entity, node: XMLNode) -> Result<()> {
    for child in node.children().filter(XMLNode::is_element) {
        let name = child.tag_name().name().to_string();
        let flag = select_child("Flags", &child).unwrap();
        let ty = select_child("Type", &child).unwrap();

        let property = Property {
            _name: name,
            _ty:   entity.type_aliases.parse_type(&ty)?,
            _flag: flag.text().unwrap().trim().to_string(),
        };

        entity.properties.push(property)
    }

    Ok(())
}

fn parse_methods(node: XMLNode, type_alias_dict: &TypeAliasLookup) -> Result<Vec<Method>> {
    let mut methods = Vec::new();
    for child in node.children().filter(XMLNode::is_element) {
        methods.push(parse_method(child, type_alias_dict)?)
    }

    Ok(methods)
}

fn parse_method(node: XMLNode, type_alias_dict: &TypeAliasLookup) -> Result<Method> {
    let name = node.tag_name().name().to_string();

    let mut params = Vec::new();
    let mut variable_header_size = None;
    for child in node.children().filter(XMLNode::is_element) {
        if is_arg(&child) {
            let ty = type_alias_dict.parse_type(&child)?;

            params.push(ty);
            continue;
        }
        if is_variable_header_size(&child) {
            variable_header_size = Some(parse_variable_header_size(&child)?);

            continue;
        }
    }

    Ok(Method {
        name,
        params,
        variable_header_size,
    })
}

fn is_interface(node: &XMLNode) -> bool {
    let tag_name = node.tag_name().name();

    tag_name == "Interface"
}

fn is_variable_header_size(node: &XMLNode) -> bool {
    let tag_name = node.tag_name().name();

    tag_name == "VariableLengthHeaderSize"
}

fn parse_variable_header_size(node: &XMLNode) -> Result<u8> {
    let header_size = node.text().unwrap().trim();
    let header_size: u8 = header_size
        .parse()
        .map_err(|_| Error::Other("header size parse error".to_string()))?;

    Ok(header_size)
}

/// `<Interface>	TeamBase_Arena	</Interface>` => `TeamBase_Arena`
fn get_interface_name<'a>(node: &'a XMLNode) -> &'a str {
    node.text().unwrap().trim()
}

fn is_arg(node: &XMLNode) -> bool {
    let tag_name = node.tag_name().name();

    tag_name == "Arg"
}

/// Get file path of `.def` for a particular version of an entity / interface.
/// We force a rule that all paths be lowercase because the casing was not consistent in originial WoT files.
///
/// For ex. in WoT `v0.9.15`, everything was lowercase (ex: `avatar.def`) then in later versions it was
/// `Avatar.def`
fn get_def_file_path(version: [u16; 4], name: &str, is_interface: bool) -> String {
    let game_version = utils::version_as_string(version);
    let def_dir = utils::get_definitions_root();

    if is_interface {
        format!("{def_dir}/{game_version}/interfaces/{name}.def").to_lowercase()
    } else {
        format!("{def_dir}/{game_version}/{name}.def").to_lowercase()
    }
}
