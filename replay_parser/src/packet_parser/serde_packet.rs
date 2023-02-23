use nom::bytes::complete::take;
use nom::number::complete::{le_u24, le_u8};
use serde::de::{self, DeserializeSeed, EnumAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

use super::event::{TrackVersion, VersionInfo};
use crate::packet_parser::PacketError;


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

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

pub fn from_slice<'a, T>(input: &'a [u8], de_version: [u16; 4]) -> Result<T, PacketError>
where
    T: Deserialize<'a> + TrackVersion,
{
    let mut deserializer = Deserializer::from_slice(input, de_version, T::version(), T::name());
    let t = T::deserialize(&mut deserializer)?;

    if !deserializer.is_empty() {
        return Err(PacketError::UnconsumedInput);
    }

    Ok(t)
}

pub fn from_slice_prim<'a, T>(input: &'a [u8], de_version: [u16; 4]) -> Result<T, PacketError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_slice(input, de_version, VersionInfo::All, "");
    let t = T::deserialize(&mut deserializer)?;

    if !deserializer.is_empty() {
        return Err(PacketError::UnconsumedInput);
    }

    Ok(t)
}

pub fn _from_slice_prim_unchecked<'a, T>(
    input: &'a [u8], de_version: [u16; 4],
) -> Result<(&'a [u8], T), PacketError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_slice(input, de_version, VersionInfo::All, "");
    let t = T::deserialize(&mut deserializer)?;

    Ok((deserializer.input, t))
}

/// Does not check if the input was fully consumed
pub fn from_slice_unchecked<'a, T>(
    input: &'a [u8], de_version: [u16; 4],
) -> Result<(&'a [u8], T), PacketError>
where
    T: Deserialize<'a> + TrackVersion,
{
    let mut deserializer = Deserializer::from_slice(input, de_version, T::version(), T::name());
    let t = T::deserialize(&mut deserializer)?;

    Ok((deserializer.input, t))
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = PacketError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(PacketError::IncorrectUsage)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remaining, result) = le_u8(self.input)?;
        self.input = remaining;
        let result = !matches!(result, 0);

        visitor.visit_bool(result)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i8;

        let (remaining, result) = le_i8(self.input)?;
        self.input = remaining;
        visitor.visit_i8(result)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i16;

        let (remaining, result) = le_i16(self.input)?;
        self.input = remaining;
        visitor.visit_i16(result)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i32;

        let (remaining, result) = le_i32(self.input)?;
        self.input = remaining;
        visitor.visit_i32(result)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i64;

        let (remaining, result) = le_i64(self.input)?;
        self.input = remaining;
        visitor.visit_i64(result)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remaining, result) = le_u8(self.input)?;
        self.input = remaining;
        visitor.visit_u8(result)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u16;

        let (remaining, result) = le_u16(self.input)?;
        self.input = remaining;
        visitor.visit_u16(result)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u32;

        let (remaining, result) = le_u32(self.input)?;
        self.input = remaining;
        visitor.visit_u32(result)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u64;

        let (remaining, result) = le_u64(self.input)?;
        self.input = remaining;
        visitor.visit_u64(result)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_f32;

        let (remaining, result) = le_f32(self.input)?;
        self.input = remaining;
        visitor.visit_f32(result)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_f64;

        let (remaining, result) = le_f64(self.input)?;
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

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remaining, len) = le_u8(self.input)?;

        if (len as usize) > remaining.len() {
            return Err(PacketError::IncompleteInput("string length is too large".into()));
        }

        let str_vec = &remaining[..(len as usize)];

        let str = std::str::from_utf8(str_vec)?;
        self.input = &remaining[(len as usize)..];
        visitor.visit_string(str.into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (remaining, bytes_array) = parse_byte_array(self.input)?;

        self.input = remaining;

        visitor.visit_borrowed_bytes(bytes_array)
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
        if self.skip {
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
        let (remaining, len) = le_u8(self.input)?;
        if len == u8::MAX {
            // This is a packed int spanning 3 bytes
            let (remaining, len) = le_u24(remaining)?;

            self.input = remaining;
            visitor.visit_seq(SequenceAccess::new(self, len as usize))
        } else {
            self.input = remaining;
            visitor.visit_seq(SequenceAccess::new(self, len as usize))
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SequenceAccess::new(self, len))
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
                visitor.visit_seq(VersionedSeqAccess::new(self, fields.len(), version_info))
            } else {
                panic!("Struct must always have version info of `Struct` variant")
            }
        } else {
            // This is for children structs of the main struct. We do not support versioning for those
            visitor.visit_seq(SequenceAccess::new(self, fields.len()))
        }
    }

    fn deserialize_enum<V>(
        self, name: &'static str, variants: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // if name != self.name {
        //     dbg!(name, self.name);
        //     panic!("Nested enums not supported")
        // };
        if let VersionInfo::Struct(version_info) = self.version_info {
            assert!(version_info.len() == variants.len());
            visitor.visit_enum(VersionedEnumAccess::new(self, variants.len(), version_info))
        } else {
            panic!("Enum must always have version info of `Struct` variant")
        }
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
    type Error = PacketError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
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
struct VersionedEnumAccess<'a, 'de: 'a> {
    de:           &'a mut Deserializer<'de>,
    version_info: &'static [VersionInfo],
    len:          usize,
    curr:         usize,
}

impl<'a, 'de> VersionedEnumAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: usize, version_info: &'static [VersionInfo]) -> Self {
        VersionedEnumAccess {
            de,
            len,
            version_info,
            curr: 0,
        }
    }
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
    type Error = PacketError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.curr == self.len {
            Ok(None)
        } else {
            // Version Check
            let version = &self.version_info[self.curr as usize];
            self.de.version_info = version.clone();

            if !is_correct_version(&self.de.de_version, version) {
                self.de.skip = true;
            }

            self.curr += 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

impl<'de, 'a> EnumAccess<'de> for VersionedEnumAccess<'a, 'de> {
    type Error = PacketError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for VersionedEnumAccess<'a, 'de> {
    type Error = PacketError;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<(), Self::Error> {
        panic!("Uncharted territory")
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let version = &self.version_info[self.curr as usize];
        if !is_correct_version(&self.de.de_version, version) {
            seed.deserialize(self.de)
        } else {
            self.de.skip = true;
            self.curr += 1;

            Err(PacketError::UnconsumedInput)
        }
    }

    // // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Uncharted territory")
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(self, _fields: &'static [&'static str], _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        panic!("Uncharted territory")
    }
}


fn is_correct_version(de_version: &[u16; 4], item_version: &VersionInfo) -> bool {
    if de_version == &[0, 0, 0, 0] {
        return true;
    }

    match item_version {
        VersionInfo::Version(version) => de_version >= version,
        VersionInfo::VersionRange((range_begin, range_end)) => {
            de_version >= range_begin && de_version < range_end
        }
        VersionInfo::VersionRangeList(list) => {
            for version_list in list.iter() {
                match version_list {
                    super::event::VersionList::Range((range_begin, range_end)) => {
                        if de_version >= range_begin && de_version < range_end {
                            return true;
                        }
                    }
                    super::event::VersionList::From(version) => {
                        if de_version >= version {
                            return true;
                        }
                    }
                }
            }
            false
        }
        _ => true,
    }
}

/// Return the remaining input and the byte_array that was parsed
pub fn parse_byte_array(input: &[u8]) -> Result<(&[u8], &[u8]), PacketError> {
    let (remaining, len) = le_u8(input)?;

    if len == u8::MAX {
        // This is a packed int spanning 3 bytes Ex: 0xFF080100
        let (remaining, len) = le_u24(remaining)?;
        let (remaining, bytes_array) = take(len)(remaining)?;

        Ok((remaining, bytes_array))
    } else {
        let (remaining, bytes_array) = take(len)(remaining)?;

        Ok((remaining, bytes_array))
    }
}
