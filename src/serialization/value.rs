//! Note: This module is loosely adapted from
//! <https://github.com/dtolnay/typetag/blob/master/src/content.rs>

use serde::de::{
    self, value, Deserialize, DeserializeSeed, Deserializer, EnumAccess, Expected,
    IntoDeserializer, MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;
use std::fmt;
use std::marker::PhantomData;

// Used from generated code to buffer the contents of the Deserializer when
// deserializing internally and adjacently tagged trait objects.
#[derive(Debug)]
pub enum Value<'de> {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    F32(f32),
    F64(f64),

    Char(char),
    String(String),
    Str(&'de str),
    ByteBuf(Vec<u8>),
    Bytes(&'de [u8]),

    None,
    Some(Box<Value<'de>>),

    Unit,
    Newtype(Box<Value<'de>>),
    Seq(Vec<Value<'de>>),
    Map(Vec<(Value<'de>, Value<'de>)>),
}

impl<'de> Value<'de> {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Value::Bool(b) => Unexpected::Bool(b),
            Value::U8(n) => Unexpected::Unsigned(u64::from(n)),
            Value::U16(n) => Unexpected::Unsigned(u64::from(n)),
            Value::U32(n) => Unexpected::Unsigned(u64::from(n)),
            Value::U64(n) => Unexpected::Unsigned(n),
            Value::I8(n) => Unexpected::Signed(i64::from(n)),
            Value::I16(n) => Unexpected::Signed(i64::from(n)),
            Value::I32(n) => Unexpected::Signed(i64::from(n)),
            Value::I64(n) => Unexpected::Signed(n),
            Value::F32(f) => Unexpected::Float(f64::from(f)),
            Value::F64(f) => Unexpected::Float(f),
            Value::Char(c) => Unexpected::Char(c),
            Value::String(ref s) => Unexpected::Str(s),
            Value::Str(s) => Unexpected::Str(s),
            Value::ByteBuf(ref b) => Unexpected::Bytes(b),
            Value::Bytes(b) => Unexpected::Bytes(b),
            Value::None | Value::Some(_) => Unexpected::Option,
            Value::Unit => Unexpected::Unit,
            Value::Newtype(_) => Unexpected::NewtypeStruct,
            Value::Seq(_) => Unexpected::Seq,
            Value::Map(_) => Unexpected::Map,
        }
    }
}

impl<'de> Deserialize<'de> for Value<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Untagged and internally tagged enums are only supported in
        // self-describing formats.
        let visitor = ValueVisitor { value: PhantomData };
        deserializer.deserialize_any(visitor)
    }
}

struct ValueVisitor<'de> {
    value: PhantomData<Value<'de>>,
}

impl<'de> Visitor<'de> for ValueVisitor<'de> {
    type Value = Value<'de>;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("any value")
    }

    fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::Bool(value))
    }

    fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::I8(value))
    }

    fn visit_i16<F>(self, value: i16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::I16(value))
    }

    fn visit_i32<F>(self, value: i32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::I32(value))
    }

    fn visit_i64<F>(self, value: i64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::I64(value))
    }

    fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::U8(value))
    }

    fn visit_u16<F>(self, value: u16) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::U16(value))
    }

    fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::U32(value))
    }

    fn visit_u64<F>(self, value: u64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::U64(value))
    }

    fn visit_f32<F>(self, value: f32) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::F32(value))
    }

    fn visit_f64<F>(self, value: f64) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::F64(value))
    }

    fn visit_char<F>(self, value: char) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::Char(value))
    }

    fn visit_str<F>(self, value: &str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::String(value.into()))
    }

    fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::Str(value))
    }

    fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::String(value))
    }

    fn visit_bytes<F>(self, value: &[u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::ByteBuf(value.into()))
    }

    fn visit_borrowed_bytes<F>(self, value: &'de [u8]) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::Bytes(value))
    }

    fn visit_byte_buf<F>(self, value: Vec<u8>) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::ByteBuf(value))
    }

    fn visit_unit<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::Unit)
    }

    fn visit_none<F>(self) -> Result<Self::Value, F>
    where
        F: de::Error,
    {
        Ok(Value::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v| Value::Some(Box::new(v)))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|v| Value::Newtype(Box::new(v)))
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(e) = visitor.next_element()? {
            vec.push(e);
        }
        Ok(Value::Seq(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(kv) = visitor.next_entry()? {
            vec.push(kv);
        }
        Ok(Value::Map(vec))
    }

    fn visit_enum<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
    where
        V: EnumAccess<'de>,
    {
        Err(de::Error::custom(
            "untagged and internally tagged enums do not support enum input",
        ))
    }
}

pub struct ValueDeserializer<'de, E> {
    value: Value<'de>,
    err: PhantomData<E>,
}

impl<'de, E> IntoDeserializer<'de, E> for Value<'de>
where
    E: de::Error,
{
    type Deserializer = ValueDeserializer<'de, E>;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializer::new(self)
    }
}

impl<'de, E> IntoDeserializer<'de, E> for ValueDeserializer<'de, E>
where
    E: de::Error,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de, E> ValueDeserializer<'de, E>
where
    E: de::Error,
{
    fn invalid_type(self, exp: &dyn Expected) -> E {
        de::Error::invalid_type(self.value.unexpected(), exp)
    }

    fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, E>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::U8(v) => visitor.visit_u8(v),
            Value::U16(v) => visitor.visit_u16(v),
            Value::U32(v) => visitor.visit_u32(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::I8(v) => visitor.visit_i8(v),
            Value::I16(v) => visitor.visit_i16(v),
            Value::I32(v) => visitor.visit_i32(v),
            Value::I64(v) => visitor.visit_i64(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

fn visit_value_seq<'de, V, E>(value: Vec<Value<'de>>, visitor: V) -> Result<V::Value, E>
where
    V: Visitor<'de>,
    E: de::Error,
{
    let seq = value.into_iter().map(ValueDeserializer::new);
    let mut seq_visitor = value::SeqDeserializer::new(seq);
    let value = visitor.visit_seq(&mut seq_visitor)?;
    seq_visitor.end()?;
    Ok(value)
}

fn visit_value_map<'de, V, E>(
    value: Vec<(Value<'de>, Value<'de>)>,
    visitor: V,
) -> Result<V::Value, E>
where
    V: Visitor<'de>,
    E: de::Error,
{
    let map = value
        .into_iter()
        .map(|(k, v)| (ValueDeserializer::new(k), ValueDeserializer::new(v)));
    let mut map_visitor = value::MapDeserializer::new(map);
    let value = visitor.visit_map(&mut map_visitor)?;
    map_visitor.end()?;
    Ok(value)
}

impl<'de, E> Deserializer<'de> for ValueDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(v) => visitor.visit_bool(v),
            Value::U8(v) => visitor.visit_u8(v),
            Value::U16(v) => visitor.visit_u16(v),
            Value::U32(v) => visitor.visit_u32(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::I8(v) => visitor.visit_i8(v),
            Value::I16(v) => visitor.visit_i16(v),
            Value::I32(v) => visitor.visit_i32(v),
            Value::I64(v) => visitor.visit_i64(v),
            Value::F32(v) => visitor.visit_f32(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::Char(v) => visitor.visit_char(v),
            Value::String(v) => visitor.visit_string(v),
            Value::Str(v) => visitor.visit_borrowed_str(v),
            Value::ByteBuf(v) => visitor.visit_byte_buf(v),
            Value::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Value::Unit => visitor.visit_unit(),
            Value::None => visitor.visit_none(),
            Value::Some(v) => visitor.visit_some(ValueDeserializer::new(*v)),
            Value::Newtype(v) => visitor.visit_newtype_struct(ValueDeserializer::new(*v)),
            Value::Seq(v) => visit_value_seq(v, visitor),
            Value::Map(v) => visit_value_map(v, visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(v) => visitor.visit_bool(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::F32(v) => visitor.visit_f32(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::I64(v) => visitor.visit_i64(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::F64(v) => visitor.visit_f64(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::I64(v) => visitor.visit_i64(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Char(v) => visitor.visit_char(v),
            Value::String(v) => visitor.visit_string(v),
            Value::Str(v) => visitor.visit_borrowed_str(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(v) => visitor.visit_string(v),
            Value::Str(v) => visitor.visit_borrowed_str(v),
            Value::ByteBuf(v) => visitor.visit_byte_buf(v),
            Value::Bytes(v) => visitor.visit_borrowed_bytes(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(v) => visitor.visit_string(v),
            Value::Str(v) => visitor.visit_borrowed_str(v),
            Value::ByteBuf(v) => visitor.visit_byte_buf(v),
            Value::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Value::Seq(v) => visit_value_seq(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::Some(v) => visitor.visit_some(ValueDeserializer::new(*v)),
            Value::Unit => visitor.visit_unit(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Unit => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            // As a special case, allow deserializing untagged newtype
            // variant containing unit struct.
            //
            //     #[derive(Deserialize)]
            //     struct Info;
            //
            //     #[derive(Deserialize)]
            //     #[serde(tag = "topic")]
            //     enum Message {
            //         Info(Info),
            //     }
            //
            // We want {"topic":"Info"} to deserialize even though
            // ordinarily unit structs do not deserialize from empty map.
            Value::Map(ref v) if v.is_empty() => visitor.visit_unit(),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Newtype(v) => visitor.visit_newtype_struct(ValueDeserializer::new(*v)),
            _ => visitor.visit_newtype_struct(self),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Seq(v) => visit_value_seq(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Map(v) => visit_value_map(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Seq(v) => visit_value_seq(v, visitor),
            Value::Map(v) => visit_value_map(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self.value {
            Value::Map(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(de::Error::invalid_value(
                            Unexpected::Map,
                            &"map with a single key",
                        ));
                    }
                };
                // enums are encoded in json as maps with a single key:value pair
                if iter.next().is_some() {
                    return Err(de::Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ));
                }
                (variant, Some(value))
            }
            s @ Value::String(_) | s @ Value::Str(_) => (s, None),
            other => {
                return Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"string or map",
                ));
            }
        };

        visitor.visit_enum(EnumDeserializer::new(variant, value))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(v) => visitor.visit_string(v),
            Value::Str(v) => visitor.visit_borrowed_str(v),
            Value::ByteBuf(v) => visitor.visit_byte_buf(v),
            Value::Bytes(v) => visitor.visit_borrowed_bytes(v),
            Value::U8(v) => visitor.visit_u8(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }
}

impl<'de, E> ValueDeserializer<'de, E> {
    fn new(value: Value<'de>) -> Self {
        ValueDeserializer {
            value,
            err: PhantomData,
        }
    }
}

struct EnumDeserializer<'de, E>
where
    E: de::Error,
{
    variant: Value<'de>,
    value: Option<Value<'de>>,
    err: PhantomData<E>,
}

impl<'de, E> EnumDeserializer<'de, E>
where
    E: de::Error,
{
    fn new(variant: Value<'de>, value: Option<Value<'de>>) -> EnumDeserializer<'de, E> {
        EnumDeserializer {
            variant,
            value,
            err: PhantomData,
        }
    }
}

impl<'de, E> EnumAccess<'de> for EnumDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;
    type Variant = VariantDeserializer<'de, Self::Error>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), E>
    where
        V: DeserializeSeed<'de>,
    {
        let visitor = VariantDeserializer {
            value: self.value,
            err: PhantomData,
        };
        seed.deserialize(ValueDeserializer::new(self.variant))
            .map(|v| (v, visitor))
    }
}

struct VariantDeserializer<'de, E>
where
    E: de::Error,
{
    value: Option<Value<'de>>,
    err: PhantomData<E>,
}

impl<'de, E> VariantAccess<'de> for VariantDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), E> {
        match self.value {
            Some(value) => Deserialize::deserialize(ValueDeserializer::new(value)),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, E>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)),
            None => Err(de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Seq(v)) => Deserializer::deserialize_any(SeqDeserializer::new(v), visitor),
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"tuple variant",
            )),
            None => Err(de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Map(v)) => Deserializer::deserialize_any(MapDeserializer::new(v), visitor),
            Some(Value::Seq(v)) => Deserializer::deserialize_any(SeqDeserializer::new(v), visitor),
            Some(other) => Err(de::Error::invalid_type(
                other.unexpected(),
                &"struct variant",
            )),
            None => Err(de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

struct SeqDeserializer<'de, E>
where
    E: de::Error,
{
    iter: <Vec<Value<'de>> as IntoIterator>::IntoIter,
    err: PhantomData<E>,
}

impl<'de, E> SeqDeserializer<'de, E>
where
    E: de::Error,
{
    fn new(vec: Vec<Value<'de>>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
            err: PhantomData,
        }
    }
}

impl<'de, E> Deserializer<'de> for SeqDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(ret)
            } else {
                Err(de::Error::invalid_length(len, &"fewer elements in array"))
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, E> SeqAccess<'de> for SeqDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)).map(Some),
            None => Ok(None),
        }
    }
}

struct MapDeserializer<'de, E>
where
    E: de::Error,
{
    iter: <Vec<(Value<'de>, Value<'de>)> as IntoIterator>::IntoIter,
    value: Option<Value<'de>>,
    err: PhantomData<E>,
}

impl<'de, E> MapDeserializer<'de, E>
where
    E: de::Error,
{
    fn new(map: Vec<(Value<'de>, Value<'de>)>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
            err: PhantomData,
        }
    }
}

impl<'de, E> MapAccess<'de> for MapDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(ValueDeserializer::new(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ValueDeserializer::new(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

impl<'de, E> Deserializer<'de> for MapDeserializer<'de, E>
where
    E: de::Error,
{
    type Error = E;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
