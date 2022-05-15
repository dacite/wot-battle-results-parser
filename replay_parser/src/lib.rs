use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::BlockDecryptor;
use miniz_oxide::inflate::decompress_to_vec_zlib;
mod error;
pub mod packet_parser;
use nom::bytes::complete::take;
use nom::multi::length_count;
use nom::multi::length_data;
use nom::number::complete::le_u32;
pub use packet_parser::events;
pub use packet_parser::PacketStream;

mod battle_context;
pub use battle_context::{get_replay_time, BattleContext};
pub use error::Error;
pub use error::Result;
/// A tuple of :
/// 1. `JSON Values`
/// 2. `Binary buffer` (contains data used to play replay)
type ReplayParseResult = (Vec<serde_json::Value>, Vec<u8>);


/// Parse a wotreplay from file. Only deals with that wotreplay. If you need to parse multiple replays, create
/// multiple instances of `ReplayParser`.
pub struct ReplayParser {
    /// raw binary data of the entire `.wotreplay` file
    data: Vec<u8>,

    /// WoT version of the `.wotreplay` file. Parsed from the JSON portion
    version: Option<[u16; 4]>,

    /// JSON portion of the `.wotreplay` file. Lazily evaluated.
    json: Vec<serde_json::Value>,

    /// Packet portion of the `.wotreplay` file. Lazily evaluated.
    packets_buffer: Option<Vec<u8>>,
}

impl ReplayParser {
    pub fn parse<T: Read>(mut input: T) -> Result<Self> {
        let mut data = Vec::new();
        input
            .read_to_end(&mut data)
            .map_err(|err| Error::Other(format!("failed to read replay file: {}", err.to_string())))?;

        let (json, packets_buffer) = parse_nom(&data)?;

        Ok(ReplayParser {
            data,
            version: get_replay_version(&json),
            json,
            packets_buffer: Some(packets_buffer),
        })
    }

    pub fn parse_json<T: Read>(mut input: T) -> Result<Self> {
        let mut data = Vec::new();
        input
            .read_to_end(&mut data)
            .map_err(|err| Error::Other(format!("failed to read replay file: {}", err.to_string())))?;

        let json = parse_json_nom(&data)?;

        Ok(ReplayParser {
            data,
            version: get_replay_version(&json),
            json,
            packets_buffer: None,
        })
    }

    pub fn load_packets(&mut self) -> Result<()> {
        if self.packets_buffer.is_none() {
            let packets_buffer = parse_binary_nom(&self.data)?;

            self.packets_buffer = Some(packets_buffer);
        }

        Ok(())
    }

    pub fn get_json(&self) -> &[serde_json::Value] {
        &self.json
    }

    pub fn get_version(&self) -> Result<[u16; 4]> {
        if let Some(version) = self.version {
            println!("{:?}", version);
            Ok(version)
        } else {
            Err(Error::Other("failed to get replay version".to_string()))
        }
    }

    pub fn packet_stream(&self) -> Result<PacketStream> {
        if self.packets_buffer.is_none() {
            return Err(Error::Other(
                "binary portion is not parsed. call load_packets to parse".to_string(),
            ));
        }

        Ok(PacketStream::new(&self.packets_buffer.as_ref().unwrap()))
    }

    pub fn event_stream(&self) -> Result<events::EventStream> {
        let packet_stream = self.packet_stream()?;
        let version = self.get_version()?;

        Ok(events::EventStream::new(packet_stream, version))
    }

    pub fn battle_context(&self) -> BattleContext {
        BattleContext::from(&self.json, &self.packets_buffer.as_ref().unwrap())
    }
}

/// Parse both the JSON portion and the binary portion
pub fn parse(input: &[u8]) -> Result<ReplayParseResult> {
    let mut seekable = Cursor::new(input);

    // Parse JSON Dump
    seekable.seek(SeekFrom::Start(4))?;
    let json_dumps = separate_json(&mut seekable)?;

    // Parse Binary Dump
    seekable.seek(SeekFrom::Current(8))?;
    let binary_dump = separate_binary(&mut seekable)?;

    Ok((json_dumps, binary_dump))
}

/// Return the JSON part and Binary part of the `.wotreplay` file as a tuple
pub fn separate_nom(input: &[u8]) -> Result<(Vec<&[u8]>, &[u8])> {
    let (remaining, _magic_num) = take(4 as usize)(input)?;

    // Take JSON Slices
    let take_json_slice = length_data(le_u32);
    let (remaining, json_slices) = length_count(le_u32, take_json_slice)(remaining)?;

    // Take Binary Slice (the part that contains the packets)
    let (binary_slice, _magic_num) = take(8 as usize)(remaining)?;

    Ok((json_slices, binary_slice))
}

pub fn parse_nom(input: &[u8]) -> Result<ReplayParseResult> {
    let (json_slices, binary_slice) = separate_nom(input)?;

    let mut json = Vec::new();
    for slice in json_slices {
        json.push(serde_json::from_slice(slice)?);
    }

    let packets_buffer = separate_binary(&mut Cursor::new(binary_slice))?;

    Ok((json, packets_buffer))
}

pub fn parse_json_nom(input: &[u8]) -> Result<Vec<serde_json::Value>> {
    let (json_slices, _binary_slice) = separate_nom(input)?;

    let mut json = Vec::new();
    for slice in json_slices {
        json.push(serde_json::from_slice(slice)?);
    }

    Ok(json)
}

pub fn parse_binary_nom(input: &[u8]) -> Result<Vec<u8>> {
    let (_json_slices, binary_slice) = separate_nom(input)?;

    let packets_buffer = separate_binary(&mut Cursor::new(binary_slice))?;

    Ok(packets_buffer)
}

/// Parse the JSON portion only
pub fn parse_json(input: &[u8]) -> Result<Vec<serde_json::Value>> {
    let mut seekable = Cursor::new(input);

    // Parse JSON Dump
    seekable.seek(SeekFrom::Start(4))?;
    separate_json(&mut seekable)
}

pub fn get_replay_version(json: &[serde_json::Value]) -> Option<[u16; 4]> {
    let json = json.iter().next()?;

    let version = json.pointer("/clientVersionFromExe")?.as_str()?;
    let version = version.replace(", ", "."); // Some replays have ", " as delimiter

    let mut version_array = [0u16; 4];
    for (i, substr) in version.splitn(4, ".").enumerate() {
        version_array[i] = substr.parse().ok()?;
    }

    Some(version_array)
}
/// Separate the JSON dumps
fn separate_json(seekable: &mut Cursor<&[u8]>) -> Result<Vec<serde_json::Value>> {
    let mut parsed_json = Vec::new();

    let blocks_count = seekable.read_u32::<LittleEndian>()?;
    for _ in 0..blocks_count {
        let block_size = seekable.read_u32::<LittleEndian>()? as usize;
        let block_start: usize = seekable.position() as usize;
        let block_end = block_start + block_size;

        let json_u8 = &seekable.get_ref()[block_start..block_end];
        parsed_json.push(serde_json::from_slice(json_u8)?);

        seekable.seek(SeekFrom::Current(block_size as i64))?;
    }

    Ok(parsed_json)
}

fn separate_binary(seekable: &mut Cursor<&[u8]>) -> Result<Vec<u8>> {
    let mut decrypted = decrypt_remaining_slice(seekable)?;
    xor_decrypted(&mut decrypted);

    let decompressed =
        decompress_to_vec_zlib(&decrypted).map_err(|_| Error::Other("decompression error".to_string()))?;

    Ok(decompressed)
}

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
