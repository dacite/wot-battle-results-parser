use macros::{EventPrinter, Version};
use serde::{Deserialize, Serialize};

use super::Vector3;
use crate::events::{EventPrinter, Version, VersionInfo};

/// Ex: Your teammate hits an enemy. Another ex: You get shot by artillery.
#[derive(Serialize, Deserialize, Debug, Clone, Version)]
pub struct ShowDamageFromShot {
    pub entity_id:     u32,
    pub points:        Vec<u64>,
    pub effects_index: u8,
    pub damage_factor: u8,

    #[version([1, 13, 0, 0])]
    pub last_material_shield: bool,
}

/// A vehicle fires a shot
#[derive(Serialize, Deserialize, Debug, Clone, Version)]
pub struct ShowShooting {
    /// This value seems to be `1` most times. Perhaps, with a double-barrel tank it could different.
    burst_count: u8,

    #[version([1, 6, 1, 0])]
    gun_index: Option<u8>,
}

/// Ex: A vehicle takes a shot and loses hp
#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
pub struct OnHealthChanged {
    new_health: i16,
    old_health: i16,

    #[event_debug(as_player)]
    attacker_id: i32,

    attack_reason: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
pub struct ShowDamageFromExplosion {
    #[event_debug(as_player)]
    attacker_id: i32,

    center:        Vector3,
    effects_idx:   u8,
    damage_factor: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Version, EventPrinter)]
pub struct ShowTracer {
    #[event_debug(as_player)]
    shooter_id: i32,

    shot_id:         i32,
    is_ricochet:     bool,
    effects_idx:     u8,
    ref_start_point: Vector3,
    velocity:        Vector3,
    gravity:         f32,
    max_shot_dist:   f32,
    gun_idx:         u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, EventPrinter, Version)]
pub struct OnStaticCollision {
    energy:     f32,
    point:      Vector3,
    normal:     Vector3,
    misc_flags: u8,

    #[version([0, 9, 16, 0])]
    damage: Option<f32>,

    #[version([0, 9, 17, 0])]
    destr_effects_idx: Option<i8>,

    #[version([0, 9, 23, 0])]
    destr_max_health: Option<u16>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        events::{entity_method::EntityMethod, event_stream::Context, *},
        packet_parser::Packet,
    };

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

        let event = EntityMethodEvent::parse(&packet, &Context::default()).unwrap();

        let expected = BattleEvent::EntityMethod(EntityMethodEvent {
            entity_id: 9383728,
            size:      2,
            method_id: 0,
            event:     EntityMethod::ShotFired(ShowShooting {
                burst_count: 1,
                gun_index:   Some(0),
            }),
        });

        assert_eq!(format!("{:?}", event), format!("{:?}", expected));
    }

    #[test]
    fn parses_on_health_changed() {
        let packet_data =
            hex::decode("150000000800000048C19C433B2F8F000200000009000000680583083F2F8F0000").unwrap();
        let packet = Packet::new(&packet_data);

        let event = EntityMethodEvent::parse(&packet, &Context::default()).unwrap();

        let expected = BattleEvent::EntityMethod(EntityMethodEvent {
            entity_id: 9383739,
            size:      9,
            method_id: 2,
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
