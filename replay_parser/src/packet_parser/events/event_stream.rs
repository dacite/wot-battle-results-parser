use std::collections::HashMap;

use definition_parser::utils::validate_version;

use super::battle_event::Event;
use super::*;
use crate::{PacketStream, Result};

#[derive(Default)]
pub struct Context {
    entities: HashMap<i32, String>,
    version:  [u16; 4],
}

impl Context {
    pub fn new(version: [u16; 4]) -> Result<Self> {
        Ok(Context {
            entities: HashMap::new(),
            version,
        })
    }

    pub fn get_version(&self) -> [u16; 4] {
        self.version
    }

    pub fn add_entity(&mut self, entity_id: i32, entity_name: &str) {
        // let entity = Entity::new(entity_name, self.version, self.type_aliases.clone()).unwrap();

        self.entities.insert(entity_id, entity_name.to_string());
    }

    pub fn find_method(&self, entity_id: i32, method_id: i32) -> Option<&str> {
        let entity_name = if let Some(name) = self.entities.get(&entity_id) {
            name
        } else {
            "Vehicle"
        };

        let version_str = crate::utils::version_as_string(self.version);

        method_defs::find_method(entity_name, &version_str, method_id)
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
    context:       Context,
}

impl<'pkt> EventStream<'pkt> {
    pub fn new(packet_stream: PacketStream<'pkt>, version: [u16; 4]) -> Result<Self> {
        let version = validate_version(version);
        let context = Context::new(version)?;

        Ok(EventStream {
            packet_stream,
            context,
        })
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

                log_if_error(&event);
                Some(event)
            }
            Err(err) => Some(Err(err.into())),
        }
    }
}

fn log_if_error(event: &Result<Event>) {
    match event.as_ref() {
        Ok(_) => {}
        Err(err) => {
            tracing::error!(error = ?err)
        }
    }
}
