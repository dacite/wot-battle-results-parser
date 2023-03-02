use crate::packet_parser::prelude::*;

// Credit: https://github.com/Monstrofil/replays_unpack/blob/master/replay_unpack/core/packets/Position.py
#[derive(Debug, Clone, Deserialize, Version, EventPrinter, Serialize)]
pub struct Position {
    pub entity_id: i32,

    #[version(range([0, 9, 7, 0], [0, 9, 14, 0]))]
    pub space_id: Option<i32>,

    pub vehicle_id:     i32,
    pub position:       Vector3,
    pub position_error: Vector3,
    pub yaw:            f32,
    pub pitch:          f32,
    pub roll:           f32,
    pub is_volatile:    u8,
}

impl PacketParser for Position {
    fn parse(packet: &Packet, context: &Context) -> Result<BattleEvent, PacketError> {
        let position = from_slice(packet.payload(), context.get_version())?;

        Ok(BattleEvent::Position(position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::replay_iterator;
    pub use crate::{Packet, ReplayParser};

    #[test]
    fn parses_position() {
        for file in replay_iterator(&std::env::var("REPLAY_DIR").unwrap()) {
            println!("{} ", file.display());
            let parser = ReplayParser::parse_file(file).unwrap();
            let mut context = parser.context().unwrap();

            // We currently do not have the def files for versions less than 0.9.13.0 so we skip them
            if parser.parse_replay_version().unwrap() < [0, 9, 13, 0] {
                return;
            }

            let packet_stream = parser
                .packet_stream()
                .filter(|packet| packet.as_ref().unwrap().packet_type() == 0x0A);

            for position_packet in packet_stream {
                let position_packet = position_packet.unwrap();
                let position = BattleEvent::parse(&position_packet, &mut context);

                match position.unwrap() {
                    BattleEvent::Position(pos) => {
                        // TODO: Add more checks
                        assert!(matches!(pos.is_volatile, 0..=1));
                    }
                    _ => panic!("Incorrect event. expected position"),
                }
            }
        }
    }
}
