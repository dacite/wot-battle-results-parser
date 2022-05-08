use macros::EventPrinter;
use nom::number::complete::le_i32;
use serde::{Deserialize, Serialize};

use super::{AsPacket, BattleEvent, EventPrinter, PacketParser};
use crate::{
    packet_parser::{serde_packet, Packet},
    BattleContext, Error, Result,
};

#[derive(Debug, Clone, EventPrinter)]
/// Represents all packets of type `0x08`. `0x08` packet seems to describe a method call on an entity.
/// Refers to multiple types of events (depending on which method was called on the entity).
pub struct EntityMethodEvent<'pkt> {
    /// ID of the entity who called this method
    #[event_debug(as_player)]
    entity_id: i32,

    /// The packet representation of the `EntityMethod`
    #[event_debug(ignore)]
    inner: &'pkt Packet<'pkt>,

    /// Total size of the method arguments. i.e the size of the actual payload `(packet_payload =
    /// (payload_info_size + actual_payload_size))`
    size: i32,

    /// Method ID associated with this method. This ID could be different for the same method if the replay
    /// versions are different
    method: i32,

    /// Houses details about the actual entity method that was called
    #[event_debug(custom_debug)]
    event: EntityMethod,
}

impl<'pkt> PacketParser<'pkt> for EntityMethodEvent<'pkt> {
    fn parse(packet: &'pkt Packet) -> Result<BattleEvent<'pkt>> {
        let data = packet.get_payload();
        let (remaining, entity_id) = le_i32::<_, crate::Error>(data)?;
        let (remaining, method_id) = le_i32::<_, crate::Error>(remaining)?;
        let (method_data, size) = le_i32::<_, crate::Error>(remaining)?;

        let entity_method_event = EntityMethodEvent {
            entity_id,
            inner: packet,
            size,
            method: method_id,
            event: EntityMethod::new(method_id, method_data)?,
        };

        Ok(BattleEvent::EntityMethod(entity_method_event))
    }
}

impl<'pkt> AsPacket for EntityMethodEvent<'pkt> {
    fn as_packet(&self) -> &Packet {
        self.inner
    }
}

impl EntityMethodEvent<'_> {
    /// Whether we understand the entity method
    pub fn is_unknown(&self) -> bool {
        if let EntityMethod::Unknown = self.event {
            true
        } else {
            false
        }
    }
}

/// Enumerates all possible entity method calls (hopefully).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EntityMethod {
    DamageFromShot(ShowDamageFromShot),
    ShotFired(ShowShooting),
    HealthChanged(OnHealthChanged),
    Unknown,
}

impl EntityMethod {
    /// TODO: This is where the parsing gets difficult. For now, we keep match statement this way. However,
    /// the values are different depending on the replay version. To make this more general we will need
    /// to parse definition files or come up with another solution.
    pub fn new(id: i32, data: &[u8]) -> Result<Self> {
        use EntityMethod::*;
        match id {
            0 => Ok(ShotFired(EntityMethod::parse_method("ShotFired", id, data)?)),
            2 => Ok(HealthChanged(EntityMethod::parse_method("HealthChanged", id, data)?)),
            _ => Ok(Unknown),
        }
    }

    /// We move the parsing logic needed to create `EntityMethod` to its own function because with `map_err`
    /// it gets messy. We only really need `data` to parse the method; the rest of the args are used for
    /// decorating the error from `from_slice` with method information
    fn parse_method<'de, T: Deserialize<'de>>(name: &str, id: i32, data: &'de [u8]) -> Result<T> {
        serde_packet::from_slice(data).map_err(|err| Error::new_entity_method_err(data, id, name, err))
    }
}

impl EventPrinter for EntityMethod {
    fn to_debug_string(&self, context: &BattleContext) -> String
    where
        Self: std::fmt::Debug,
    {
        match self {
            EntityMethod::DamageFromShot(event) => format!("{:?}", event),
            EntityMethod::ShotFired(event) => format!("{:?}", event),
            EntityMethod::HealthChanged(event) => format!("{:?}", event.to_debug_string(context)),
            EntityMethod::Unknown => format!("Unknown"),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////
// Structs below describe a specific method call
///////////////////////////////////////////////////////////////////////////////////////////////////////////


/// Ex: Your teammate hits an enemy. Another ex: You get shot by artillery.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowDamageFromShot {
    pub entity_id:            u32,
    pub points:               Vec<u64>,
    pub effects_index:        u8,
    pub damage_factor:        u8,
    pub last_material_shield: bool,
}

/// A vehicle fires a shot
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowShooting {
    /// This value seems to be `1` most times. Perhaps, with a double-barrel tank it could different.
    burst_count: u8,

    gun_index: u8, // TODO: This field is not present in older replays.
}

/// TODO:
#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter)]
pub struct OnHealthChanged {
    new_health: i16,
    old_health: i16,

    #[event_debug(as_player)]
    attacker_id: i32,

    attack_reason: u8,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::events::BattleEvent;

    // #[test]
    // fn parses_show_damage_from_shot() {
    //     let packet_data = hex::decode("352F8F0002000100B9BC1DB7AE030103B9BA39B5A1130000").unwrap();

    //     let show_damage_from_shot: ShowDamageFromShot =
    //         crate::packet_parser::from_slice(packet_data.as_slice()).unwrap();

    //     let expected = ShowDamageFromShot {
    //         entity_id:            9383733,
    //         points:               vec![12589563979732353280, 11652283085021708547],
    //         effects_index:        19,
    //         damage_factor:        0,
    //         last_material_shield: false,
    //     };
    //     assert_eq!(format!("{:?}", show_damage_from_shot), format!("{:?}", expected));
    // }

    #[test]
    fn parses_show_shooting() {
        let packet_data = hex::decode("0E00000008000000AEE7C743302F8F0000000000020000000100").unwrap();
        let packet = Packet::new(&packet_data);

        let event = EntityMethodEvent::parse(&packet).unwrap();

        let expected = BattleEvent::EntityMethod(EntityMethodEvent {
            entity_id: 9383728,
            inner:     &packet,
            size:      2,
            method:    0,
            event:     EntityMethod::ShotFired(ShowShooting {
                burst_count: 1,
                gun_index:   0,
            }),
        });

        assert_eq!(format!("{:?}", event), format!("{:?}", expected));
    }

    #[test]
    fn parses_on_health_changed() {
        let packet_data = hex::decode("150000000800000048C19C433B2F8F000200000009000000680583083F2F8F0000").unwrap();
        let packet = Packet::new(&packet_data);

        let event = EntityMethodEvent::parse(&packet).unwrap();

        let expected = BattleEvent::EntityMethod(EntityMethodEvent {
            entity_id: 9383739,
            inner:     &packet,
            size:      9,
            method:    2,
            event:     EntityMethod::HealthChanged(OnHealthChanged {
                new_health:    1384,
                old_health:    2179,
                attacker_id:   9383743,
                attack_reason: 0,
            }),
        });

        assert_eq!(format!("{:?}", event), format!("{:?}", expected));
    }
}
