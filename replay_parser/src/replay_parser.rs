use core::result::Result as StdResult;

use crypto::{blowfish::Blowfish, symmetriccipher::BlockDecryptor};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use nom::{
    bytes::complete::take,
    multi::{length_count, length_data},
    number::complete::le_u32,
};

use crate::{json_parser::JsonParser, BattleContext, Error, EventStream, PacketStream, Result};
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

    /// JSON portion of the `.wotreplay` file.
    json: Vec<serde_json::Value>,

    /// Packet portion of the `.wotreplay` file. Lazily evaluated.
    packets_buffer: Option<Vec<u8>>,
}

impl ReplayParser {
    #[tracing::instrument]
    pub fn parse_file(path: &str) -> Result<Self> {
        let input = std::fs::read(path)?;

        Self::parse(input)
    }

    /// Parse JSON from replay file and convert the binary portion into packets iterator (lazy)
    pub fn parse(input: Vec<u8>) -> Result<Self> {
        let (json_slices, binary_slice) = split_replay_data(&input)?;
        let packets_buffer = decrypt_and_decompress_binary(binary_slice)?;

        // Raw JSON buffer to JSONValue from serde_json
        let json: StdResult<Vec<_>, serde_json::Error> =
            json_slices.into_iter().map(serde_json::from_slice).collect();
        let json = json?;

        Ok(ReplayParser {
            data: input,
            version: get_replay_version(&json),
            json,
            packets_buffer: Some(packets_buffer),
        })
    }

    /// Only parse the JSON portion of the replay. Use this if the packets portion is not needed because
    /// parsing the binary into packets is quite expensive.
    pub fn parse_json(input: Vec<u8>) -> Result<Self> {
        let (json_slices, _binary_slice) = split_replay_data(&input)?;

        // Raw JSON buffer to JSONValue from serde_json
        let json: StdResult<Vec<_>, serde_json::Error> =
            json_slices.into_iter().map(serde_json::from_slice).collect();
        let json = json?;

        Ok(ReplayParser {
            data: input,
            version: get_replay_version(&json),
            json,
            packets_buffer: None,
        })
    }

    /// Call this if `ReplayParser` was instantiated with [parse_json](ReplayParser::parse_json) and the
    /// packets portion is later required
    pub fn load_packets(&mut self) -> Result<()> {
        if self.packets_buffer.is_none() {
            let packets_buffer = parse_binary(&self.data)?;

            self.packets_buffer = Some(packets_buffer);
        }

        Ok(())
    }

    /// Get JSON values of this wotreplay
    pub fn get_json(&self) -> &[serde_json::Value] {
        &self.json
    }

    /// Interact with the packets buffer directly. Usually you will only need
    /// [packet_stream](ReplayParser::packet_stream)
    pub fn get_packets_buffer(&self) -> Result<&[u8]> {
        if let Some(buffer) = &self.packets_buffer {
            Ok(buffer)
        } else {
            Err(Error::Other(
                "binary portion is not parsed. call load_packets to parse".to_string(),
            ))
        }
    }

    /// Very similar to [get_replay_version](get_replay_version)
    pub fn get_version(&self) -> Result<[u16; 4]> {
        if let Some(version) = self.version {
            Ok(version)
        } else {
            Err(Error::Other("failed to get replay version".to_string()))
        }
    }

    /// An iterator over the packets in the replay. Must call `load_packets` if its not already loaded. The
    /// separation allows this method to take a shared reference
    pub fn packet_stream(&self) -> Result<PacketStream> {
        if self.packets_buffer.is_none() {
            return Err(Error::Other(
                "binary portion is not parsed. call load_packets to parse".to_string(),
            ));
        }

        Ok(PacketStream::new(self.packets_buffer.as_ref().unwrap()))
    }

    /// An iterator over the events in the replay. This is a layer of abstraction over `PacketStream`. Each
    /// packet is converted into the event it represents.
    // #[tracing::instrument(name = "event_stream", skip_all)]
    pub fn event_stream(&self) -> Result<EventStream> {
        let packet_stream = self.packet_stream()?;
        let version = self.get_version()?;

        EventStream::new(packet_stream, version)
    }

    pub fn battle_context(&self) -> BattleContext {
        BattleContext::from(&self.json, self.packets_buffer.as_ref().unwrap())
    }

    /// Bi-directional map from avatar_id to db_id
    pub fn gen_id_bimap(&self) -> Result<bimap::BiHashMap<i32, i64>> {
        let json = self
            .get_json()
            .get(1)
            .ok_or_else(|| Error::Other(String::from("Cannot generate bimap for incomplete replay")))?;

        // TODO: Enfore DRY
        let vehicles = json
            .pointer("/0/vehicles")
            .ok_or_else(|| Error::Other("JSON access error".into()))?
            .as_object()
            .ok_or_else(|| Error::Other("Unexpected JSON type".into()))?;

        let mut map = bimap::BiHashMap::new();

        for (avatar_id, vehicle) in vehicles {
            let avatar_id = avatar_id.parse().unwrap();
            let db_id = vehicle
                .pointer("/0/accountDBID")
                .ok_or_else(|| Error::Other("JSON access error".into()))?
                .as_i64()
                .ok_or_else(|| Error::Other("Unexpected JSON type".into()))?;

            map.insert(avatar_id, db_id);
        }

        Ok(map)
    }
}

impl JsonParser for ReplayParser {
    fn get_json(&self) -> &[serde_json::Value] {
        self.get_json()
    }
}

pub fn parse_binary(input: &[u8]) -> Result<Vec<u8>> {
    let (_json_slices, binary_slice) = split_replay_data(input)?;

    let decrypted = decrypt(binary_slice)?;
    let decrypted = xor_decrypted(decrypted);

    let packets_buffer =
        decompress_to_vec_zlib(&decrypted).map_err(|_| Error::Other("decompression error".to_string()))?;

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
pub fn get_replay_version(json: &[serde_json::Value]) -> Option<[u16; 4]> {
    let json = json.iter().next()?;

    // World\u{a0}of\u{a0}Tanks v.1.9.1.1 #378
    let version = json.pointer("/clientVersionFromXml")?.as_str()?;

    // "1.9.1.1 #378"
    let version = version.replace("World\u{a0}of\u{a0}Tanks v.", "");

    // ["1.9.1.1", "#378"]
    let version: Vec<_> = version.split(' ').collect();

    // "1.9.1.1"
    let version = version.get(0)?;

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
fn split_replay_data(input: &[u8]) -> Result<(Vec<&[u8]>, &[u8])> {
    let (remaining, _magic_num) = take(4_usize)(input)?;

    // Take JSON Slices
    let take_json_slice = length_data(le_u32);
    let (remaining, json_slices) = length_count(le_u32, take_json_slice)(remaining)?;

    // Take Binary Slice (the part that contains the packets)
    let (binary_slice, _magic_num) = take(8_usize)(remaining)?;

    Ok((json_slices, binary_slice))
}

fn decrypt_and_decompress_binary(binary: &[u8]) -> Result<Vec<u8>> {
    let decrypted = decrypt(binary)?;
    let decrypted = xor_decrypted(decrypted);

    let decompressed =
        decompress_to_vec_zlib(&decrypted).map_err(|_| Error::Other("decompression error".to_string()))?;

    Ok(decompressed)
}

fn decrypt(input_blocks: &[u8]) -> Result<Vec<u8>> {
    // Init blowfish cipher
    let mut wot_blowfish_key = [0; 16];
    hex::decode_to_slice("DE72BEA0DE04BEB1DEFEBEEFDEADBEEF", &mut wot_blowfish_key).unwrap();
    let bf = Blowfish::new(&wot_blowfish_key);

    // Init output buffer (output buffer needs to be a multiple of 8)
    let input_blocks_len = input_blocks.len();
    let mut output_buffer = vec![0; input_blocks_len + (input_blocks_len % 8)];

    // Fill output buffer with decrypted values
    for i in (0..input_blocks_len).step_by(8) {
        if (i + 8) > input_blocks_len {
            let padded_block = get_padded_block(&input_blocks[i..input_blocks_len]);

            bf.decrypt_block(&padded_block, &mut output_buffer[i..(i + 8)]);
        } else {
            bf.decrypt_block(&input_blocks[i..(i + 8)], &mut output_buffer[i..(i + 8)]);
        }
    }

    Ok(output_buffer)
}

#[inline]
fn xor_decrypted(mut decrypted: Vec<u8>) -> Vec<u8> {
    for i in 8..decrypted.len() {
        decrypted[i] ^= decrypted[i - 8];
    }

    decrypted
}

#[inline]
fn get_padded_block(source_block: &[u8]) -> [u8; 8] {
    let mut padded_block = [0x00; 8];
    let block_size = source_block.len();
    padded_block[..block_size].clone_from_slice(source_block);

    padded_block
}
