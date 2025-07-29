use crate::{field::NbtField, value::NbtValue};

#[derive(Debug, PartialEq, Clone)]
pub enum NbtList {
    Byte(Vec<u8>),
    Boolean(Vec<bool>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    String(Vec<String>),
    List(Vec<NbtValue>),
    Compound(Vec<NbtField>),
    LongArray(Vec<Vec<i64>>),
    IntArray(Vec<Vec<i32>>),
    End,
}

impl NbtList {
    pub fn as_byte_list(&self) -> Option<&Vec<u8>> {
        match self {
            NbtList::Byte(b) => Some(b),
            _ => None,
        }
    }
    pub fn as_boolean_list(&self) -> Option<&Vec<bool>> {
        match self {
            NbtList::Boolean(b) => Some(b),
            _ => None,
        }
    }
    pub fn as_short_list(&self) -> Option<&Vec<i16>> {
        match self {
            NbtList::Short(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_int_list(&self) -> Option<&Vec<i32>> {    
        match self {
            NbtList::Int(i) => Some(i),
            _ => None,
        }
    }
    pub fn as_long_list(&self) -> Option<&Vec<i64>> {
        match self {
            NbtList::Long(l) => Some(l),
            _ => None,
        }
    }
    pub fn as_float_list(&self) -> Option<&Vec<f32>> {
        match self {
            NbtList::Float(f) => Some(f),
            _ => None,
        }
    }
    pub fn as_double_list(&self) -> Option<&Vec<f64>> {
        match self {
            NbtList::Double(d) => Some(d),
            _ => None,
        }
    }
    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            NbtList::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&Vec<NbtValue>> {
        match self {
            NbtList::List(l) => Some(l),
            _ => None,
        }
    }
    pub fn as_compound_list(&self) -> Option<&Vec<NbtField>> {
        match self {
            NbtList::Compound(c) => Some(c),
            _ => None,
        }
    }
    pub fn as_long_array_list(&self) -> Option<&Vec<Vec<i64>>> {
        match self {
            NbtList::LongArray(l) => Some(l),
            _ => None,
        }
    }
    pub fn as_int_array_list(&self) -> Option<&Vec<Vec<i32>>> {
        match self {
            NbtList::IntArray(i) => Some(i),
            _ => None,
        }
    }
}

impl std::fmt::Display for NbtList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        match self {
            NbtList::Byte(b) => {
                for b in b {
                    write!(f, "{}, ", b)?;
                }
            }
            NbtList::Boolean(b) => {
                for b in b {
                    write!(f, "{}, ", b)?;
                }
            }
            NbtList::Short(s) => {
                for s in s {
                    write!(f, "{}, ", s)?;
                }
            }
            NbtList::Int(i) => {
                for i in i {
                    write!(f, "{}, ", i)?;
                }
            }
            NbtList::Long(l) => {
                for l in l {
                    write!(f, "{}, ", l)?;
                }
            }
            NbtList::Float(float) => {
                for float in float {
                    write!(f, "{}, ", float)?;
                }
            }
            NbtList::Double(d) => {
                for d in d {
                    write!(f, "{}, ", d)?;
                }
            }
            NbtList::String(s) => {
                for s in s {
                    write!(f, "{}, ", s)?;
                }
            }
            NbtList::List(l) => {
                for l in l {
                    write!(f, "{}, ", l)?;
                }
            }
            NbtList::Compound(c) => {
                for c in c {
                    write!(f, "{}, ", c)?;
                }
            }
            NbtList::LongArray(list) => {
                for l in list {
                    for l in l {
                        write!(f, "{}, ", l)?;
                    }
                }
            }
            NbtList::IntArray(list) => {
                for l in list {
                    for l in l {
                        write!(f, "{}, ", l)?;
                    }
                }
            }
            NbtList::End => {
                write!(f, "Empty")?;
            }
        }

        write!(f, "]")
    }
}
