use crate::{field::NbtField, list::NbtList};


pub const TAG_END: u8 = 0;
pub const TAG_BYTE: u8 = 1;
pub const TAG_SHORT: u8 = 2;
pub const TAG_INT: u8 = 3;
pub const TAG_LONG: u8 = 4;
pub const TAG_FLOAT: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_BYTE_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_INT_ARRAY: u8 = 11;
pub const TAG_LONG_ARRAY: u8 = 12;

#[derive(Debug, PartialEq, Clone)]
pub enum NbtValue {
    Byte(u8),
    Boolean(bool),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(NbtList),
    Compound(Vec<NbtField>),
    ByteArray(Vec<u8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    End,
}

impl std::fmt::Display for NbtValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtValue::Byte(b) => write!(f, "{}", b),
            NbtValue::Boolean(b) => write!(f, "{}", b),
            NbtValue::Short(s) => write!(f, "{}i16", s),
            NbtValue::Int(i) => write!(f, "{}i32", i),
            NbtValue::Long(l) => write!(f, "{}i64", l),
            NbtValue::Float(value) => write!(f, "{}f32", value),
            NbtValue::Double(d) => write!(f, "{}f64", d),
            NbtValue::String(s) => write!(f, "\"{}\"", s),
            NbtValue::List(l) => write!(f, "{}", l),
            NbtValue::Compound(c) => {
                write!(f, "{{")?;
                for field in c {
                    write!(f, "{}, ", field)?;
                }
                write!(f, "}}")
            },
            NbtValue::ByteArray(b) => write!(f, "byte[...]"),
            NbtValue::IntArray(i) => write!(f, "int[...]"),
            NbtValue::LongArray(l) => write!(f, "long[...]"),
            NbtValue::End => write!(f, "End"),
        }
    }
}
