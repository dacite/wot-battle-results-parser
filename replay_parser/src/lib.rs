use std::io::{Cursor, Seek, SeekFrom};

use anyhow::anyhow;
use byteorder::{LittleEndian, ReadBytesExt};
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::BlockDecryptor;
use miniz_oxide::inflate::decompress_to_vec_zlib;
mod error;
pub mod packet_parser;
pub use packet_parser::events;
pub use packet_parser::PacketStream;

mod battle_context;
pub mod xml_handler;
pub use battle_context::{get_replay_time, BattleContext};
pub use error::Error;
pub use error::Result;
/// A tuple of :
/// 1. `JSON Values`
/// 2. `Binary buffer` (contains data used to play replay)
type ReplayParseResult = (Vec<serde_json::Value>, Vec<u8>);

/// Parse both the JSON portion and the binary portion
pub fn parse(input: &[u8]) -> anyhow::Result<ReplayParseResult> {
    let mut seekable = Cursor::new(input);

    // Parse JSON Dump
    seekable.seek(SeekFrom::Start(4))?;
    let json_dumps = separate_json(&mut seekable)?;

    // Parse Binary Dump
    seekable.seek(SeekFrom::Current(8))?;
    let binary_dump = separate_binary(&mut seekable)?;

    Ok((json_dumps, binary_dump))
}

/// Parse the JSON portion only
pub fn parse_json(input: &[u8]) -> anyhow::Result<Vec<serde_json::Value>> {
    let mut seekable = Cursor::new(input);

    // Parse JSON Dump
    seekable.seek(SeekFrom::Start(4))?;
    separate_json(&mut seekable)
}


/// Separate the JSON dumps
fn separate_json(seekable: &mut Cursor<&[u8]>) -> anyhow::Result<Vec<serde_json::Value>> {
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

fn separate_binary(seekable: &mut Cursor<&[u8]>) -> anyhow::Result<Vec<u8>> {
    let mut decrypted = decrypt_remaining_slice(seekable)?;
    xor_decrypted(&mut decrypted);

    let decompressed = decompress_to_vec_zlib(&decrypted).map_err(|_| anyhow!("decompression failure"))?;

    Ok(decompressed)
}

fn decrypt_remaining_slice(seekable: &mut Cursor<&[u8]>) -> anyhow::Result<Vec<u8>> {
    // Grab encrypted buffer
    let current_seek_pos = seekable.position() as usize;
    let encrypted_buffer = &seekable.get_ref()[current_seek_pos..];

    decrypt(encrypted_buffer)
}

fn decrypt(input_blocks: &[u8]) -> anyhow::Result<Vec<u8>> {
    // Init blowfish cipher
    let mut wot_blowfish_key = [0; 16];
    hex::decode_to_slice("DE72BEA0DE04BEB1DEFEBEEFDEADBEEF", &mut wot_blowfish_key)?;
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
