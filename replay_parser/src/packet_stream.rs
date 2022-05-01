


use byteorder::{ReadBytesExt, LE};
use macros::PacketMetadata;

pub const METADATA_SIZE: usize = 12;


#[derive(PacketMetadata)]
pub struct Packet<'a> {
    inner: &'a [u8],
}

impl<'a> Packet<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { inner: &data }
    }
}

pub trait PacketMetadata<'pkt> {
    fn get_inner(&'pkt self) -> &'pkt [u8];

    fn get_type(&'pkt self) -> u32 {
        let mut chunk = &self.get_inner()[4..8];
        chunk.read_u32::<LE>().unwrap()
    }

    fn get_time(&'pkt self) -> f32 {
        let mut chunk = &self.get_inner()[8..];
        chunk.read_f32::<LE>().unwrap()
    }

    /// Size is only the size of the payload
    /// The size of the entire packet is `(payload_size + metadata_size)`
    fn get_size(&'pkt self) -> u32 {
        let mut chunk = &self.get_inner()[..4];
        chunk.read_u32::<LE>().unwrap()
    }

    fn get_payload(&'pkt self) -> &'pkt [u8] {
        &self.get_inner()[METADATA_SIZE..]
    }
}

pub struct PacketStream<'a> {
    inner:    &'a [u8],
    position: usize,
}

impl<'a> PacketStream<'a> {
    pub fn new(inner: &'a [u8]) -> Self {
        Self { inner, position: 0 }
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }
}

impl<'a> Iterator for PacketStream<'a> {
    type Item = Packet<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;

        if (position + 4) > self.inner.len() {
            // TODO: Remove this panic or switch to the previous cursor impl since this
            // impl is not faster and more likely to have errors
            if position != self.inner.len() {
                panic!("UNEXPECTED POSITION");
            }
            return None;
        }

        let payload_size = self.inner[position..(position + 4)].try_into().unwrap();
        let payload_size = u32::from_le_bytes(payload_size);

        let packet_size = METADATA_SIZE as usize + payload_size as usize;
        let packet_range = position..(position + packet_size);

        self.position += packet_size;
        Some(Packet::new(&self.inner[packet_range]))
    }
}

impl<'a> std::fmt::Debug for Packet<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let packet_name = format!("0x{:02X}", &self.get_type());

        let displayed_payload = if self.get_size() <= 100 {
            self.get_payload()
        } else {
            &self.get_payload()[..100]
        };

        let mut payload_string = String::from("");
        let mut temp_store = Vec::new();
        for (i, byte) in displayed_payload.into_iter().enumerate() {
            temp_store.push(byte);
            if (i + 1) % 4 == 0 {
                temp_store.iter().rev().for_each(|x| {
                    payload_string.push_str(&format!("{:02X?}", x));
                });
                temp_store.retain(|_| false);
                payload_string.push_str(&" ");
            }
        }
        if temp_store.len() != 0 {
            temp_store.iter().rev().for_each(|x| {
                payload_string.push_str(&format!("{:02X?}", x));
            });
            temp_store.retain(|_| false);
        }
        let time = format!("{:3.3}", &self.get_time());
        let size = format!("{}", &self.get_size());
        if f.sign_plus() {
            f.debug_struct(&packet_name)
                .field("time", &time)
                .field("size", &size)
                .field("data", &payload_string)
                .finish()
        } else {
            f.debug_struct(&packet_name)
                .field("time", &time)
                .field("size", &size)
                .finish()
        }
    }
}
