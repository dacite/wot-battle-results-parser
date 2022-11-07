use blowfish::cipher::KeyInit;
use blowfish::{cipher::BlockDecrypt, Blowfish};
use byteorder::{ReadBytesExt, BE, LE};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use nom::{
    bytes::complete::take,
    multi::{length_count, length_data},
    number::complete::le_u32,
};
use serde_json::Value as JsonVal;
use wot_types::ArenaBonusType;

use crate::replay_errors;
use crate::utils::{as_i64};
use crate::{BattleContext, Event, EventStream, PacketStream, ReplayError};
/// Parse a wotreplay from file. Only deals with that wotreplay. If you need to parse multiple replays, create
/// multiple instances of `ReplayParser`.
/// ## Example 1 - Print Replay Events
/// ```
/// # use wot_replay_parser::*;
/// let replay_file = std::fs::File::open("input_files/example.wotreplay").unwrap();
/// let replay = ReplayParser::parse(replay_file).unwrap();
///
/// for event in replay.event_stream().unwrap() {
///     let event = event.unwrap();
///
///     if !event.is_unknown() { // skip printing events that arent support yet
///         println!("{:?}", event.into_battle_event());
///     }
/// }
/// ```
pub struct ReplayParser {
    /// raw binary data of the entire `.wotreplay` file
    data: Vec<u8>,

    /// JSON portion of the `.wotreplay` file.
    json: Vec<JsonVal>,

    /// Packet portion of the `.wotreplay` file. Lazily evaluated.
    packets_buffer: Option<Vec<u8>>,
}

// impl std::fmt::Display for ReplayParser {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Replay {{ ")?;
//         if let Some(version) = self.parse_replay_version() {
//             write!(
//                 f,
//                 "version: {}.{}.{}.{}, ",
//                 version[0], version[1], version[2], version[3]
//             )?;
//         } else {
//             write!(f, "version: None")?;
//         }
//         write!(
//             f,
//             "battle_type: {}({}) }}",
//             self.battle_type, self.battle_type as u8
//         )
//     }
// }

impl ReplayParser {
    #[tracing::instrument]
    pub fn parse_file(path: &str) -> Result<Self, ReplayError> {
        let input = std::fs::read(path)?;

        Self::parse(input)
    }

    /// Parse JSON from replay file and convert the binary portion into packets iterator (lazy)
    // #[tracing::instrument(err, ret(Display), skip_all)]
    pub fn parse(input: Vec<u8>) -> Result<Self, ReplayError> {
        let (json_slices, binary_slice) = split_replay_data(&input)?;
        let packets_buffer = decrypt_and_decompress_binary(binary_slice)?;

        let json = parse_json_portion(json_slices)?;

        Ok(ReplayParser {
            data: input,
            json,
            packets_buffer: Some(packets_buffer),
        })
    }

    /// Only parse the JSON portion of the replay. Use this if the packets portion is not needed because
    /// parsing the binary into packets is quite expensive.
    pub fn parse_json(input: Vec<u8>) -> Result<Self, ReplayError> {
        let (json_slices, _binary_slice) = split_replay_data(&input)?;

        let json = parse_json_portion(json_slices)?;

        Ok(ReplayParser {
            data: input,
            json,
            packets_buffer: None,
        })
    }

    /// Call this if `ReplayParser` was instantiated with [parse_json](ReplayParser::parse_json) and the
    /// packets portion is later required
    pub fn load_packets(&mut self) -> Result<(), ReplayError> {
        if self.packets_buffer.is_none() {
            let packets_buffer = parse_binary(&self.data)?;

            self.packets_buffer = Some(packets_buffer);
        }

        Ok(())
    }

    /// Get JSON values of this wotreplay
    pub fn replay_json(&self) -> &[JsonVal] {
        &self.json
    }

    pub fn replay_json_start(&self) -> Result<&JsonVal, ReplayError> {
        self.replay_json().get(0).ok_or_else(|| {
            ReplayError::ReplayJsonFormatError(
                "missing initial json object that is present in all replays".into(),
            )
        })
    }

    pub fn replay_json_end(&self) -> Option<&JsonVal> {
        self.replay_json().get(1)
    }

    /// Interact with the packets buffer directly. Usually you will only need
    /// [packet_stream](ReplayParser::packet_stream)
    pub fn packets_buffer(&self) -> Result<&[u8], ReplayError> {
        if let Some(buffer) = &self.packets_buffer {
            Ok(buffer)
        } else {
            Err(ReplayError::Other(
                "binary portion is not parsed. call load_packets to parse".to_string(),
            ))
        }
    }

    pub fn battle_start_time(&self) -> f32 {
        for packet in self.packet_stream() {
            let packet = packet.unwrap();
            if packet.get_type() == 0x16 && packet.get_payload().read_u32::<LE>().unwrap() == 3 {
                return packet.get_time();
            }
        }

        -1.0
    }

    /// An iterator over the packets in the replay. Must call `load_packets` if its not already loaded. The
    /// separation allows this method to take a shared reference
    pub fn packet_stream(&self) -> PacketStream {
        PacketStream::new(
            self.packets_buffer
                .as_ref()
                .expect("call load_packets before using packet stream"),
        )
    }

    /// An iterator over the events in the replay. This is a layer of abstraction over `PacketStream`. Each
    /// packet is converted into the event it represents. It is important for
    pub fn event_stream(&self) -> Result<EventStream, ReplayError> {
        let packet_stream = self.packet_stream();
        let version = self
            .parse_replay_version()
            .ok_or_else(|| ReplayError::ReplayJsonFormatError("failed to parse replay version".into()))?;

        Ok(EventStream::new(packet_stream, version)?)
    }

    pub fn battle_context(&self) -> BattleContext {
        BattleContext::from(&self.json, self.packets_buffer.as_ref().unwrap())
    }

    /// Bi-directional map from avatar_id to db_id
    pub fn gen_id_bimap(&self) -> Result<bimap::BiHashMap<i32, i64>, ReplayError> {
        let json = self
            .replay_json_end()
            .ok_or_else(|| ReplayError::Other(String::from("Cannot generate bimap for incomplete replay")))?;

        // TODO: Enfore DRY
        let vehicles = crate::utils::as_map("/0/vehicles", json)?;

        let mut map = bimap::BiHashMap::new();

        for (avatar_id, vehicle) in vehicles {
            let avatar_id = avatar_id.parse().unwrap();
            let db_id = crate::utils::as_i64("/0/accountDBID", vehicle)?;

            map.insert(avatar_id, db_id);
        }

        Ok(map)
    }

    /// Parse the replay version. Return `None` if parsing fails
    pub fn parse_replay_version(&self) -> Option<[u16; 4]> {
        // TODO: MAKE FASTER!
        let json = self.replay_json_start().ok()?;

        // World\u{a0}of\u{a0}Tanks v.1.9.1.1 #378
        // "坦克世界 v.0.9.10 #77"
        let version = json.pointer("/clientVersionFromXml")?.as_str()?;

        // ["World\u{a0}of\u{a0}Tanks", "v.1.9.1.1", "#378"]
        // ["坦克世界", "v.0.9.10", "#77"]
        let version: Vec<_> = version.split(' ').collect();

        // "v.0.9.10"
        // "v.1.9.1.1"
        let version = version.get(1)?;

        let mut version_iter = version.split('.');

        version_iter.next(); // skip "v"

        let major: u16 = version_iter.next()?.parse().ok()?;
        let minor: u16 = version_iter.next()?.parse().ok()?;
        let patch: u16 = version_iter.next()?.parse().ok()?;
        let extra: u16 = version_iter.next().unwrap_or("0").parse().ok()?;

        let version_array = [major, minor, patch, extra];

        Some(version_array)
    }

    /// Parse the Arena Unique ID of the battle in the replay. 
    /// 
    /// For complete replays, this information
    /// is present in the JSON portion of the replay and is therefore not very expensive to parse. 
    /// 
    /// For incomplete replays, Arena Unique ID is present in one of the packets and is therefore
    /// more expensive to parse
    pub fn parse_arena_unique_id(&self) -> Result<i64, ReplayError> {
        if let Some(replay_end) = self.replay_json_end() {
            as_i64("/0/arenaUniqueID", replay_end)
        } else {
            let stream = self.event_stream()?;
    
            for event in stream.flatten() {
                if let Event::AvatarCreate(avatar_create) = event {
                    return Ok(avatar_create.arena_unique_id);
                }
            }
    
            Err(ReplayError::MissingArenaUniqueId)
        }
    }

    /// Parse the type of the replay. For ex. Regular, Clan Wars, Frontlines etc.
    pub fn parse_arena_bonus_type(json: &[JsonVal]) -> Result<ArenaBonusType, ReplayError> {
        let replay_start_json = json.first().ok_or_else(|| {
            ReplayError::ReplayJsonFormatError(
                "missing initial json object that is present in all replays".into(),
            )
        })?;
        let bonus_type = crate::utils::as_i64("/battleType", replay_start_json)?;
    
        ArenaBonusType::try_from(bonus_type as i32).map_err(|_| {
            ReplayError::ReplayJsonFormatError(format!("arena bonus type of {bonus_type} is invalid"))
        })
    }
}

fn parse_binary(input: &[u8]) -> Result<Vec<u8>, ReplayError> {
    let (_json_slices, binary_slice) = split_replay_data(input)?;

    let decrypted = decrypt(binary_slice)?;
    let decrypted = xor_decrypted(decrypted);

    let packets_buffer = decompress_to_vec_zlib(&decrypted)
        .map_err(|_| ReplayError::Other("decompression error".to_string()))?;

    Ok(packets_buffer)
}

fn parse_json_portion(json_slices: Vec<&[u8]>) -> Result<Vec<serde_json::Value>, ReplayError> {
    let mut json_values = Vec::new();
    for slice in json_slices {
        let json_value = serde_json::from_slice(slice).or_else(|_| {
            let slice_as_string = String::from_utf8_lossy(slice);
            let fixed = replay_errors::fix_json_bugs(slice_as_string);

            serde_json::from_str(&fixed)
        })?;

        json_values.push(json_value);
    }

    Ok(json_values)
}



/// Return the JSON part and Binary part of the `.wotreplay` file as a tuple
fn split_replay_data(input: &[u8]) -> Result<(Vec<&[u8]>, &[u8]), ReplayError> {
    let (remaining, _magic_num) = take(4_usize)(input)?;

    // Take JSON Slices
    let take_json_slice = length_data(le_u32);
    let (remaining, json_slices) = length_count(le_u32, take_json_slice)(remaining)?;

    // Take Binary Slice (the part that contains the packets)
    let (binary_slice, _magic_num) = take(8_usize)(remaining)?;

    Ok((json_slices, binary_slice))
}

fn decrypt_and_decompress_binary(binary: &[u8]) -> Result<Vec<u8>, ReplayError> {
    let decrypted = decrypt(binary)?;
    let decrypted = xor_decrypted(decrypted);

    let decompressed = decompress_to_vec_zlib(&decrypted)
        .map_err(|_| ReplayError::Other("decompression error".to_string()))?;

    Ok(decompressed)
}

fn decrypt(input_blocks: &[u8]) -> Result<Vec<u8>, ReplayError> {
    // Init blowfish cipher
    let mut wot_blowfish_key = [0; 16];
    hex::decode_to_slice("DE72BEA0DE04BEB1DEFEBEEFDEADBEEF", &mut wot_blowfish_key).unwrap();
    let bf: Blowfish<BE> = Blowfish::new_from_slice(&wot_blowfish_key).unwrap();

    let output_len = input_blocks.len() + (input_blocks.len() % 8);
    let mut output_blocks = input_blocks.to_vec();
    output_blocks.resize(output_len, 0);

    for i in (0..output_blocks.len()).step_by(8) {
        let block = &mut output_blocks[i..(i + 8)];
        bf.decrypt_block(block.into());
    }

    Ok(output_blocks)
}

#[inline]
fn xor_decrypted(mut decrypted: Vec<u8>) -> Vec<u8> {
    for i in 8..decrypted.len() {
        decrypted[i] ^= decrypted[i - 8];
    }

    decrypted
}
