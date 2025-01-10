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
