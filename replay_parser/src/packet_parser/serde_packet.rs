use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};
use serde::Deserialize;

use crate::events::{Version, VersionInfo};
use crate::Error;

pub struct Deserializer<'de> {
    input: &'de [u8],

    /// The version the `Deserializer` expect the data format to be
    de_version: [u16; 4],

    /// Versions of each field. (only used when deserialzing to a struct)
    version_info: VersionInfo,

    /// Whether to skip deserialzing current item. This flag is set by `VersionedSeqAccess`.
    /// When set, the current item is deserialized to `None` and the flag will be unset
    skip: bool,

    /// Name of struct we are deserialzing into. We use this to make sure we call the correct
    /// visitor for children of this struct who are also structs
    name: &'static str,
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(
        input: &'de [u8], de_version: [u16; 4], version_info: VersionInfo, name: &'static str,
    ) -> Self {
        Deserializer {
            input,
            de_version,
            version_info,
            name,
            skip: false,
        }
    }
}

pub fn from_slice<'a, T>(input: &'a [u8], de_version: [u16; 4]) -> Result<T, Error>
where
    T: Deserialize<'a> + Version,
{
    let mut deserializer = Deserializer::from_slice(input, de_version, T::version(), T::name());
    let t = T::deserialize(&mut deserializer)?;

    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::InvalidPacket)
    }
}


impl<'de, 'a, 'v> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::SerdePacketError(
            "Deserializer went to unsupported type".to_string(),
        ))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u8;
        let (remaining, result) = le_u8::<_, Error>(self.input)?;
        self.input = remaining;
        let result = match result {
            0 => false,
            1 => true,
            _ => true,
        };

        visitor.visit_bool(result)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i8;

        let (remaining, result) = le_i8::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_i8(result)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i16;

        let (remaining, result) = le_i16::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_i16(result)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i32;

        let (remaining, result) = le_i32::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_i32(result)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i64;

        let (remaining, result) = le_i64::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_i64(result)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u8;

        let (remaining, result) = le_u8::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_u8(result)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u16;

        let (remaining, result) = le_u16::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_u16(result)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u32;

        let (remaining, result) = le_u32::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_u32(result)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u64;

        let (remaining, result) = le_u64::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_u64(result)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_f32;

        let (remaining, result) = le_f32::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_f32(result)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_f64;

        let (remaining, result) = le_f64::<_, Error>(self.input)?;
        self.input = remaining;
        visitor.visit_f64(result)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.skip == true {
            self.skip = false;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u8;

        let (remaining, len) = le_u8::<_, Error>(self.input)?;
        self.input = remaining;

        visitor.visit_seq(SequenceAccess::new(self, len as usize))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self, _name: &'static str, _len: usize, _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self, name: &'static str, fields: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if name == self.name {
            if let VersionInfo::Struct(version_info) = self.version_info {
                assert!(version_info.len() == fields.len());
                visitor.visit_seq(VersionedSeqAccess::new(self, fields.len(), &version_info))
            } else {
                panic!("Struct must always have version info of `Struct` variant")
            }
        } else {
            // This is for children structs of the main struct. We do not support versioning for those
            visitor.visit_seq(SequenceAccess::new(self, fields.len()))
        }
    }

    fn deserialize_enum<V>(
        self, _name: &'static str, _variants: &'static [&'static str], _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct SequenceAccess<'a, 'de: 'a> {
    de:   &'a mut Deserializer<'de>,
    len:  usize,
    curr: usize,
}

impl<'a, 'de> SequenceAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: usize) -> Self {
        SequenceAccess { de, len, curr: 0 }
    }
}

impl<'de, 'a> SeqAccess<'de> for SequenceAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.curr == self.len {
            Ok(None)
        } else {
            self.curr += 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}
struct VersionedSeqAccess<'a, 'de: 'a> {
    de:           &'a mut Deserializer<'de>,
    version_info: &'static [VersionInfo],
    len:          usize,
    curr:         usize,
}

impl<'a, 'de> VersionedSeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: usize, version_info: &'static [VersionInfo]) -> Self {
        VersionedSeqAccess {
            de,
            len,
            version_info,
            curr: 0,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for VersionedSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.curr == self.len {
            Ok(None)
        } else {
            // Version Check
            let version = &self.version_info[self.curr as usize];
            self.de.version_info = version.clone();

            if !is_correct_version(&self.de.de_version, &version) {
                self.de.skip = true;
            }

            self.curr += 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

fn is_correct_version(de_version: &[u16; 4], item_version: &VersionInfo) -> bool {
    match item_version {
        VersionInfo::Version(version) => {
            if de_version == &[0, 0, 0, 0] {
                return true;
            }

            if de_version < version {
                false
            } else {
                true
            }
        }
        _ => true,
    }
}
