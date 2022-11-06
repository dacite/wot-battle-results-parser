use core::result::Result as StdResult;

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

use crate::{BattleContext, EventStream, PacketStream, ReplayError};
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

    /// WoT version of the `.wotreplay` file. Parsed from the JSON portion
    version: Option<[u16; 4]>,

    /// Battle type of the replay. Analogous to arena bonus type for the most part
    battle_type: ArenaBonusType,

    /// JSON portion of the `.wotreplay` file.
    json: Vec<JsonVal>,

    /// Packet portion of the `.wotreplay` file. Lazily evaluated.
    packets_buffer: Option<Vec<u8>>,
}

impl std::fmt::Display for ReplayParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Replay {{ ")?;
        if let Some(version) = self.version() {
            write!(
                f,
                "version: {}.{}.{}.{}, ",
                version[0], version[1], version[2], version[3]
            )?;
        } else {
            write!(f, "version: None")?;
        }
        write!(
            f,
            "battle_type: {}({}) }}",
            self.battle_type, self.battle_type as u8
        )
    }
}


impl ReplayParser {
    #[tracing::instrument]
    pub fn parse_file(path: &str) -> Result<Self, ReplayError> {
        let input = std::fs::read(path)?;

        Self::parse(input)
    }

    /// Parse JSON from replay file and convert the binary portion into packets iterator (lazy)
    #[tracing::instrument(err, ret(Display), skip_all)]
    pub fn parse(input: Vec<u8>) -> Result<Self, ReplayError> {
        let (json_slices, binary_slice) = split_replay_data(&input)?;
        let packets_buffer = decrypt_and_decompress_binary(binary_slice)?;

        // Raw JSON buffer to JSONValue from serde_json
        let json: StdResult<Vec<_>, serde_json::Error> =
            json_slices.into_iter().map(serde_json::from_slice).collect();
        let json = json?;

        Ok(ReplayParser {
            data: input,
            version: get_replay_version(&json),
            battle_type: load_arena_bonus_type(&json)?,
            json,
            packets_buffer: Some(packets_buffer),
        })
    }

    /// Only parse the JSON portion of the replay. Use this if the packets portion is not needed because
    /// parsing the binary into packets is quite expensive.
    pub fn parse_json(input: Vec<u8>) -> Result<Self, ReplayError> {
        let (json_slices, _binary_slice) = split_replay_data(&input)?;

        // Raw JSON buffer to JSONValue from serde_json
        let json: StdResult<Vec<_>, serde_json::Error> =
            json_slices.into_iter().map(serde_json::from_slice).collect();
        let json = json?;

        Ok(ReplayParser {
            data: input,
            version: get_replay_version(&json),
            battle_type: load_arena_bonus_type(&json)?,
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

    /// Get Battle type of this wotreplay
    pub fn battle_type(&self) -> ArenaBonusType {
        self.battle_type
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

    /// Very similar to [get_replay_version](get_replay_version)
    pub fn version(&self) -> Option<[u16; 4]> {
        self.version
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
    /// packet is converted into the event it represents.
    // #[tracing::instrument(name = "event_stream", skip_all)]
    pub fn event_stream(&self) -> Result<EventStream, ReplayError> {
        let packet_stream = self.packet_stream();
        let version = self
            .version()
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
}

pub fn parse_binary(input: &[u8]) -> Result<Vec<u8>, ReplayError> {
    let (_json_slices, binary_slice) = split_replay_data(input)?;

    let decrypted = decrypt(binary_slice)?;
    let decrypted = xor_decrypted(decrypted);

    let packets_buffer = decompress_to_vec_zlib(&decrypted)
        .map_err(|_| ReplayError::Other("decompression error".to_string()))?;

    Ok(packets_buffer)
}

/// Retrieve version information of a particular replay file. If the replay is from WoT v.1.16.1, we get `[1,
/// 16, 1, 0]`.
/// ## Example
/// ```
/// # use wot_replay_parser::*;
/// let replay_file = std::fs::read("input_files/example.wotreplay").unwrap();
/// let json_values = parse_json_value(&replay_file).unwrap();
///
/// let replay_version = get_replay_version(&json_values).unwrap();
/// assert_eq!(replay_version, [1, 16, 1, 0]);
/// ```
pub fn get_replay_version(json: &[JsonVal]) -> Option<[u16; 4]> {
    let json = json.iter().next()?;

    // World\u{a0}of\u{a0}Tanks v.1.9.1.1 #378
    let version = json.pointer("/clientVersionFromXml")?.as_str()?;

    // "1.9.1.1 #378"
    let version = version.replace("World\u{a0}of\u{a0}Tanks v.", "");

    // ["1.9.1.1", "#378"]
    let version: Vec<_> = version.split(' ').collect();

    // "1.9.1.1"
    let version = version.first()?;

    let version = version.replace(", ", "."); // Some replays have ", " as delimiter

    let mut version_array = [0u16; 4];
    for (i, substr) in version.split('.').enumerate() {
        if i >= 4 {
            break;
        }

        version_array[i] = substr.parse().ok()?;
    }

    Some(version_array)
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

fn load_arena_bonus_type(json: &[JsonVal]) -> Result<ArenaBonusType, ReplayError> {
    let replay_start_json = json.first().ok_or_else(|| {
        ReplayError::ReplayJsonFormatError(
            "missing initial json object that is present in all replays".into(),
        )
    })?;
    let bonus_type = crate::utils::as_i64("/battleType", replay_start_json)?;

    ArenaBonusType::from(bonus_type as i32).ok_or_else(|| {
        ReplayError::ReplayJsonFormatError(format!("arena bonus type of {bonus_type} is invalid"))
    })
}
