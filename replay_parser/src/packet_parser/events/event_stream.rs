use std::{collections::HashMap, rc::Rc};

use super::battle_event::Event;
use super::*;
use crate::def_parser::{entity::Entity, TypeAliasLookup};
use crate::packet_parser::Packet;
use crate::{PacketStream, Result};

pub struct Context {
    entities:     HashMap<i32, Entity>,
    type_aliases: Rc<TypeAliasLookup>,
    version:      [u16; 4],
}

impl Context {
    pub fn new(version: [u16; 4]) -> Result<Self> {
        let type_aliases = TypeAliasLookup::load(version)?;
        let type_aliases = Rc::new(type_aliases);

        Ok(Context {
            entities: HashMap::new(),
            type_aliases,
            version,
        })
    }

    pub fn get_version(&self) -> [u16; 4] {
        self.version
    }

    pub fn add_entity(&mut self, entity_id: i32, entity_name: &str) {
        let entity = Entity::new(entity_name, self.version, self.type_aliases.clone()).unwrap();

        self.entities.insert(entity_id, entity);
    }

    pub fn find_method(&self, entity_id: i32, method_id: usize) -> Option<&str> {
        let entity = self.entities.get(&entity_id)?;

        entity.find_method(method_id)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            entities:     HashMap::new(),
            version:      [0, 0, 0, 0],
            type_aliases: Default::default(),
        }
    }
}

pub trait UpdateContext {
    fn update(&self, context: &mut Context) {
        // println!("BEFORE: {:?}", context.entities.len());
        self.update_context(context);
        // println!("AFTER: {:?}", context.entities.len());
    }
    fn update_context(&self, context: &mut Context);
}


pub struct EventStream<'pkt> {
    packet_stream: PacketStream<'pkt>,
    _vehicle:      Entity,
    context:       Context,
}

impl<'pkt> EventStream<'pkt> {
    pub fn new(packet_stream: PacketStream<'pkt>, version: [u16; 4]) -> Result<Self> {
        let version = crate::def_parser::utils::validate_version(version)?;
        let context = Context::new(version)?;
        let vehicle = Entity::new("Vehicle", version, context.type_aliases.clone()).unwrap();

        Ok(EventStream {
            _vehicle: vehicle,
            packet_stream,
            context,
        })
    }
}

/// Parse packet to a Battle event. Optional context is provided to aid in parsing some particular packets.
pub fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent> {
    match packet.get_type() {
        0x00 => AvatarCreate::parse(packet, context),
        0x18 => GameVersion::parse(packet, &Context::default()),
        0x08 => EntityMethodEvent::parse(packet, context),
        _ => Ok(BattleEvent::Unimplemented),
    }
}


impl<'pkt> Iterator for EventStream<'pkt> {
    type Item = Result<Event<'pkt>>;

    fn next(&mut self) -> Option<Self::Item> {
        let packet = self.packet_stream.next()?;
        match packet {
            Ok(packet) => {
                let event = parse(&packet, &self.context).map(|battle_event| {
                    battle_event.update(&mut self.context);

                    Event::new(packet, battle_event)
                });

                Some(event)
            }
            Err(err) => Some(Err(err)),
        }
    }
}
