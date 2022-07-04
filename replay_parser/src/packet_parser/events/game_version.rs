use macros::EventPrinter;
use nom::{
    bytes::complete::{take, take_till, take_while},
    character::is_digit,
};

use super::{event_stream::Context, BattleEvent, EventPrinter, PacketParser};
use crate::Result;
use crate::{packet_parser::Packet, Error};

#[derive(Debug, Clone, EventPrinter)]
pub struct GameVersion {
    /// Version of the game as described in the replay file
    /// sometimes `"0.0.0.0"` for some weird reason
    pub version: [u16; 4],
}

impl PacketParser for GameVersion {
    fn parse(packet: &Packet, _context: &Context) -> Result<BattleEvent> {
        // First, we skip an le_u32 value which tells us the size of the rest of the packet
        // We don't really need it because we try to match 4 digits in input regardless of size.
        let (remaining, _) = take(4_usize)(packet.get_payload())?;

        // We could simply the parse the `remaining` as one str but the problem is the delimiter can be
        // different for different replay files so getting that problem out of the way at this stage
        // seems reasonable. For example version str could be "1.16.1.0" or "0, 9, 16, 0"
        // This will parse those to [1, 16, 1, 0] and [0, 9, 16, 0]
        let (remaining, major) = match_digit(remaining)?;
        let (remaining, minor) = match_digit(remaining)?;
        let (remaining, patch) = match_digit(remaining)?;
        let (remaining, extra) = match_digit(remaining)?;

        if !remaining.is_empty() {
            return Err(Error::InvalidPacket);
        }

        Ok(BattleEvent::GameVersion(Self {
            version: [major, minor, patch, extra],
        }))
    }
}

/// Match char that corresponds to a digit. Then take all char that are not digits. This second step is done
/// to ensure the next call to match_digit with the same buffer will start on a digit char
fn match_digit(s: &[u8]) -> Result<(&[u8], u16)> {
    let (remaining, digit) = take_while(is_digit)(s)?;
    let (remaining, _) = take_till(is_digit)(remaining)?;

    let digit = std::str::from_utf8(digit).map_err(|err| Error::Other(err.to_string()))?;
    let digit = digit
        .parse()
        .map_err(|_| Error::Other("parse integer error".to_string()))?;
    Ok((remaining, digit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_correct_game_version_given_packet_data() {
        let packet_data = hex::decode("0B000000180000000000000007000000312E392E312E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = crate::packet_parser::parse(&game_version_packet, &Context::default());
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, [1, 9, 1, 0]);

        let packet_data = hex::decode("0C000000180000000000000008000000312E31362E312E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = crate::packet_parser::parse(&game_version_packet, &Context::default());
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();

        assert_eq!(game_version.version, [1, 16, 1, 0]);

        let packet_data = hex::decode("0C000000180000000000000008000000312E31352E302E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = crate::packet_parser::parse(&game_version_packet, &Context::default());
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, [1, 15, 0, 0]);

        let packet_data = hex::decode("0B000000180000000000000007000000312E392E302E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = crate::packet_parser::parse(&game_version_packet, &Context::default());
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, [1, 9, 0, 0]);

        let packet_data = hex::decode("0F00000018000000000000000B000000302C20392C2031352C2030").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = crate::packet_parser::parse(&game_version_packet, &Context::default());
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, [0, 9, 15, 0]);
    }
}
