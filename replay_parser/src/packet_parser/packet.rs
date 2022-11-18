use byteorder::{ReadBytesExt, LE};

use crate::ReplayError;

pub const METADATA_SIZE: usize = 12;

/// A packet is simply a wrapper around a slice that represents that packet. We can also access its type,
/// timestamp and payload size
#[derive(Clone)]
pub struct Packet<'a> {
    inner: &'a [u8],
    id:    i32,
}

impl<'pkt> Packet<'pkt> {
    pub fn new(id: i32, data: &'pkt [u8]) -> Self {
        Self { id, inner: data }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn packet_type(&'pkt self) -> u32 {
        let mut chunk = &self.inner()[4..8];
        chunk.read_u32::<LE>().unwrap()
    }

    pub fn time(&'pkt self) -> f32 {
        let mut chunk = &self.inner()[8..];
        chunk.read_f32::<LE>().unwrap()
    }

    /// Size is only the size of the payload
    /// The size of the entire packet is `(payload_size + metadata_size)`
    pub fn size(&'pkt self) -> u32 {
        let mut chunk = &self.inner()[..4];
        chunk.read_u32::<LE>().unwrap()
    }

    pub fn payload(&'pkt self) -> &'pkt [u8] {
        &self.inner()[METADATA_SIZE..]
    }

    pub fn inner(&'pkt self) -> &'pkt [u8] {
        self.inner
    }
}

/// A `PacketStream` is a wrapper around the binary data produced after decompressing the `.wotreplay` file.
/// It implements the `Iterator` trait which allows to retrieve each `Packet` as we iterate through the binary
/// stream.
#[derive(Clone)]
pub struct PacketStream<'a> {
    inner:    &'a [u8],
    position: usize,
    count:    i32,
}

impl<'a> PacketStream<'a> {
    pub fn new(inner: &'a [u8]) -> Self {
        Self {
            inner,
            position: 0,
            count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}

impl<'a> Iterator for PacketStream<'a> {
    type Item = Result<Packet<'a>, ReplayError>;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;

        if (position + 4) > self.inner.len() {
            if position != self.inner.len() {
                return Some(Err(ReplayError::PacketStreamError));
            }
            return None;
        }

        let payload_size = self.inner[position..(position + 4)].try_into().unwrap();
        let payload_size = u32::from_le_bytes(payload_size);

        let packet_size = METADATA_SIZE as usize + payload_size as usize;
        let packet_range = position..(position + packet_size);

        if (position + packet_size) > self.inner.len() {
            return Some(Err(ReplayError::PacketStreamError));
        }

        self.position += packet_size;

        let packet_id = self.count;
        self.count += 1;

        Some(Ok(Packet::new(packet_id, &self.inner[packet_range])))
    }
}

impl<'a> std::fmt::Debug for Packet<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_spaced = false;
        let chunk = if let Some(width) = f.width() {
            if width == 0 {
                1
            } else {
                is_spaced = true;
                width
            }
        } else {
            1
        };

        let _packet_as_hex = (0..self.inner.len())
            .step_by(chunk)
            .fold(String::new(), |acc, i| {
                let len = if i + chunk > self.inner.len() {
                    self.inner.len()
                } else {
                    i + chunk
                };
                if is_spaced {
                    format!("{} {}", acc, hex::encode_upper(&self.inner[i..len]))
                } else {
                    format!("{}{}", acc, hex::encode_upper(&self.inner[i..len]))
                }
            });

        let payload_as_hex = (0..self.payload().len())
            .step_by(chunk)
            .fold(String::new(), |acc, i| {
                let len = if i + chunk > self.payload().len() {
                    self.payload().len()
                } else {
                    i + chunk
                };

                if is_spaced {
                    format!("{} {}", acc, hex::encode_upper(&self.payload()[i..len]))
                } else {
                    format!("{}{}", acc, hex::encode_upper(&self.payload()[i..len]))
                }
            });

        let packet_name = format!("0x{:02X}", &self.packet_type());
        let time = format!("{:3.3}", &self.time());
        let size = format!("{}", &self.size());

        if f.sign_plus() {
            f.debug_struct(&packet_name)
                .field("data", &payload_as_hex)
                .finish()
        } else {
            f.debug_struct(&packet_name)
                .field("time", &time)
                .field("size", &size)
                .finish()
        }
    }
}
