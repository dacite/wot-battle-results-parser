use nom::{
    bytes::complete::{take, take_till, take_while},
    character::is_digit,
};

use crate::packet_parser::prelude::*;

#[derive(Debug, Clone, EventPrinter)]
pub struct GameVersion {
    /// Version of the game as described in the replay file
    /// sometimes `"0.0.0.0"` for some weird reason
    pub version: [u16; 4],
}

impl PacketParser for GameVersion {
    fn parse(packet: &Packet, _context: &Context) -> Result<Event, PacketError> {
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
            return Err(PacketError::UnconsumedInput);
        }

        Ok(Event::GameVersion(Self {
            version: [major, minor, patch, extra],
        }))
    }
}

/// Match char that corresponds to a digit. Then take all char that are not digits. This second step is done
/// to ensure the next call to match_digit with the same buffer will start on a digit char
fn match_digit(s: &[u8]) -> Result<(&[u8], u16), PacketError> {
    let (remaining, digit) = take_while(is_digit)(s)?;
    let (remaining, _) = take_till(is_digit)(remaining)?;

    let digit = std::str::from_utf8(digit)?;
    let digit: u16 = digit.parse()?;

    Ok((remaining, digit))
}
