use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};
use serde::Deserialize;

use crate::Error;

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_slice(s);
    let t = T::deserialize(&mut deserializer)?;

    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::InvalidPacket)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_none()
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

        let (remaining, result) = le_i16::<_, utils::NomErrorWrapper>(self.input)?;
        self.input = remaining;
        visitor.visit_i16(result)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i32;

        let (remaining, result) = le_i32::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_i32(result)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_i64;

        let (remaining, result) = le_i64::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_i64(result)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u8;

        let (remaining, result) = le_u8::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_u8(result)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u16;

        let (remaining, result) = le_u16::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_u16(result)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u32;

        let (remaining, result) = le_u32::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_u32(result)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_u64;

        let (remaining, result) = le_u64::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_u64(result)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_f32;

        let (remaining, result) = le_f32::<_, utils::NomErrorWrapper>(self.input).unwrap();
        self.input = remaining;
        visitor.visit_f32(result)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use nom::number::complete::le_f64;

        let (remaining, result) = le_f64::<_, utils::NomErrorWrapper>(self.input).unwrap();
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

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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

        visitor.visit_seq(SequenceAccess::new(self, len as u16))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
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
        self, _name: &'static str, fields: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SequenceAccess::new(self, fields.len() as u16))
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
    len:  u16,
    curr: u16,
}

impl<'a, 'de> SequenceAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: u16) -> Self {
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
