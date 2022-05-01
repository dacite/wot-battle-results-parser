use anyhow::{ensure, Result};
use getset::Getters;
use macros::PacketMetadata;
use nom::{
    bytes::complete::{take, take_till, take_while},
    character::is_digit,
    IResult,
};
use utils::NomErrorWrapper;

use crate::{
    event::{battle_event::BattleEvent, PacketParser},
    packet_stream::{Packet, PacketMetadata},
};

#[derive(derivative::Derivative, Getters, PacketMetadata, Clone)]
#[derivative(Debug)]
pub struct GameVersion<'pkt> {
    #[derivative(Debug = "ignore")]
    inner: &'pkt [u8],

    /// Version of the game as described in the replay file
    /// sometimes `"0.0.0.0"` for some weird reason
    version: [&'pkt str; 4],
}

/// Match char that corresponds to a digit. Then take all char that are not digits. This second step is done to ensure
/// the next call to match_digit with the same buffer will start on a digit char
fn match_digit(s: &[u8]) -> IResult<&[u8], &str> {
    let (remaining, digit) = take_while(is_digit)(s)?;
    let (remaining, _) = take_till(is_digit)(remaining)?;

    let digit = std::str::from_utf8(digit).unwrap();
    Ok((remaining, digit))
}

impl<'pkt> PacketParser<'pkt> for GameVersion<'pkt> {
    fn parse(packet: &'pkt Packet) -> Result<BattleEvent<'pkt>> {
        let inner = packet.get_payload();

        // First, we skip an le_u32 value which tells us the size of the rest of the packet
        // We dont really need it because we try to match 4 digits in input regardless of size.
        // We also ensure the packet was completely parsed with `ensure!`
        let (remaining, _) = take::<_, _, NomErrorWrapper>(4 as usize)(inner).unwrap();

        // We could simply the parse the `remaining` as one str but the problem is the delimiter can be different for
        // different replay files so getting that problem out of the way at this stage seems reasonable.
        // For example version str could be "1.16.1.0" or "0, 9, 16, 0"
        // This will parse those to ["1", "16", "1", "0"] and ["0", "9", "16", "0"]
        let (remaining, major) = match_digit(remaining).unwrap();
        let (remaining, minor) = match_digit(remaining).unwrap();
        let (remaining, patch) = match_digit(remaining).unwrap();
        let (remaining, extra) = match_digit(remaining).unwrap();

        ensure!(
            remaining.is_empty(),
            "Game Version (0x18) not empty after finishing parsing"
        );

        Ok(BattleEvent::GameVersion(Self {
            inner,
            version: [major, minor, patch, extra],
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_correct_game_version_given_packet_data() {
        let packet_data = hex::decode("0B000000180000000000000007000000312E392E312E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = BattleEvent::new(&game_version_packet);
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, ["1", "9", "1", "0"]);

        let packet_data = hex::decode("0C000000180000000000000008000000312E31362E312E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = BattleEvent::new(&game_version_packet);
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();

        assert_eq!(game_version.version, ["1", "16", "1", "0"]);

        let packet_data = hex::decode("0C000000180000000000000008000000312E31352E302E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = BattleEvent::new(&game_version_packet);
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, ["1", "15", "0", "0"]);

        let packet_data = hex::decode("0B000000180000000000000007000000312E392E302E30").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = BattleEvent::new(&game_version_packet);
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, ["1", "9", "0", "0"]);

        let packet_data = hex::decode("0F00000018000000000000000B000000302C20392C2031352C2030").unwrap();
        let game_version_packet = Packet::new(&packet_data);
        let game_version = BattleEvent::new(&game_version_packet);
        let game_version = utils::try_variant!(game_version.unwrap(), BattleEvent::GameVersion).unwrap();
        assert_eq!(game_version.version, ["0", "9", "15", "0"]);
    }
}
