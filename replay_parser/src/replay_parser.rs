use core::result::Result as StdResult;
use std::io::{Cursor, Read};

use crypto::{blowfish::Blowfish, symmetriccipher::BlockDecryptor};
use miniz_oxide::inflate::decompress_to_vec_zlib;
use nom::{
    bytes::complete::take,
    multi::{length_count, length_data},
    number::complete::le_u32,
};

use crate::{BattleContext, Error, EventStream, PacketStream, ReplayParseResult, Result};
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
    /// Parse JSON from replay file and convert the binary portion into packets
    pub fn parse<T: Read>(mut input: T) -> Result<Self> {
        let mut data = Vec::new();
        input.read_to_end(&mut data).map_err(|_| Error::ReplayFileError)?;

        let (json, packets_buffer) = parse(&data)?;

        // Raw JSON buffer to JSONValue from serde_json
        let json: StdResult<Vec<_>, serde_json::Error> = json
            .into_iter()
            .map(|json_buf| serde_json::from_slice(json_buf))
            .collect();
        let json = json?;

        Ok(ReplayParser {
            data,
            version: get_replay_version(&json),
            json,
            packets_buffer: Some(packets_buffer),
        })
    }

    /// Only parse the JSON portion of the replay. Use this if the packets portion is not needed because
    /// parsing the binary into packets is quite expensive.
    pub fn parse_json<T: Read>(mut input: T) -> Result<Self> {
        let mut data = Vec::new();
        input
            .read_to_end(&mut data)
            .map_err(|err| Error::Other(format!("failed to read replay file: {}", err.to_string())))?;

        let json = parse_json_value(&data)?;

        Ok(ReplayParser {
            data,
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

        Ok(PacketStream::new(&self.packets_buffer.as_ref().unwrap()))
    }

    /// An iterator over the events in the replay. This is a layer of abstraction over `PacketStream`. Each
    /// packet is converted into the event it represents.
    pub fn event_stream(&self) -> Result<EventStream> {
        let packet_stream = self.packet_stream()?;
        let version = self.get_version()?;

        EventStream::new(packet_stream, version)
    }

    pub fn battle_context(&self) -> BattleContext {
        BattleContext::from(&self.json, &self.packets_buffer.as_ref().unwrap())
    }
}

/// Parse replay and return the JSON portion and the binary portion (after decrypting)
/// ## Example
/// ```
/// # use wot_replay_parser::parse_json;
/// let replay_file = std::fs::read("input_files/example.wotreplay").unwrap();
/// let (json_values, binary_portion) = parse(&replay_file).unwrap();
///
/// for json_buf in json_buffers {
///     println!("{}", std::str::from_utf8(json_buf).unwrap());
/// }
/// ```
pub fn parse(input: &[u8]) -> Result<ReplayParseResult> {
    let (json_slices, binary_slice) = split_replay_data(input)?;

    let packets_buffer = separate_binary(&mut Cursor::new(binary_slice))?;

    Ok((json_slices, packets_buffer))
}

/// Parse and return the raw JSON data from the replay. Each element in the vector refers to a JSON object
/// ## Example - Print JSON info of a replay
/// ```
/// # use wot_replay_parser::parse_json;
/// let replay_file = std::fs::read("input_files/example.wotreplay").unwrap();
/// let json_buffers = parse_json(&replay_file).unwrap();
///
/// for json_buf in json_buffers {
///     println!("{}", std::str::from_utf8(json_buf).unwrap());
/// }
/// ```
pub fn parse_json(input: &[u8]) -> Result<Vec<&[u8]>> {
    let (json_slices, _binary_slice) = split_replay_data(input)?;

    Ok(json_slices)
}

/// Same as [parse_json](fn.parse_json.html) but converts the raw json buffers to `serde_json::Value`
pub fn parse_json_value(input: &[u8]) -> Result<Vec<serde_json::Value>> {
    let (json_slices, _binary_slice) = split_replay_data(input)?;

    let mut json = Vec::new();
    for slice in json_slices {
        json.push(serde_json::from_slice(slice)?);
    }

    Ok(json)
}

pub fn parse_binary(input: &[u8]) -> Result<Vec<u8>> {
    let (_json_slices, binary_slice) = split_replay_data(input)?;

    let packets_buffer = separate_binary(&mut Cursor::new(binary_slice))?;

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
    let version: Vec<_> = version.split(" ").collect();

    // "1.9.1.1"
    let version = version.get(0)?;

    let version = version.replace(", ", "."); // Some replays have ", " as delimiter

    let mut version_array = [0u16; 4];
    for (i, substr) in version.split(".").enumerate() {
        if i >= 4 {
            break;
        }

        version_array[i] = substr.parse().ok()?;
    }

    Some(version_array)
}

/// Return the JSON part and Binary part of the `.wotreplay` file as a tuple
fn split_replay_data(input: &[u8]) -> Result<(Vec<&[u8]>, &[u8])> {
    let (remaining, _magic_num) = take(4 as usize)(input)?;

    // Take JSON Slices
    let take_json_slice = length_data(le_u32);
    let (remaining, json_slices) = length_count(le_u32, take_json_slice)(remaining)?;

    // Take Binary Slice (the part that contains the packets)
    let (binary_slice, _magic_num) = take(8 as usize)(remaining)?;

    Ok((json_slices, binary_slice))
}

fn separate_binary(seekable: &mut Cursor<&[u8]>) -> Result<Vec<u8>> {
    let mut decrypted = decrypt_remaining_slice(seekable)?;
    xor_decrypted(&mut decrypted);

    let decompressed =
        decompress_to_vec_zlib(&decrypted).map_err(|_| Error::Other("decompression error".to_string()))?;

    Ok(decompressed)
}

/// We have a seperate function for this because cursor remaining is unstable feature.
fn decrypt_remaining_slice(seekable: &mut Cursor<&[u8]>) -> Result<Vec<u8>> {
    // Grab encrypted buffer
    let current_seek_pos = seekable.position() as usize;
    let encrypted_buffer = &seekable.get_ref()[current_seek_pos..];

    decrypt(encrypted_buffer)
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

fn xor_decrypted(decrypted: &mut [u8]) {
    for i in 8..decrypted.len() {
        decrypted[i] ^= decrypted[i - 8];
    }
}

fn get_padded_block(source_block: &[u8]) -> [u8; 8] {
    let mut padded_block = [0x00; 8];
    let block_size = source_block.len();
    padded_block[..block_size].clone_from_slice(source_block);

    padded_block
}
