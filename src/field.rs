use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::utils::{read_compound, read_list, read_name, read_string, write_list, write_string, TagWrite, TagWriteFull};
use crate::value::{NbtValue, TAG_BYTE, TAG_BYTE_ARRAY, TAG_COMPOUND, TAG_DOUBLE, TAG_END, TAG_FLOAT, TAG_INT, TAG_INT_ARRAY, TAG_LIST, TAG_LONG, TAG_LONG_ARRAY, TAG_SHORT, TAG_STRING};

use crate::NbtError;

#[derive(Debug, PartialEq, Clone)]
pub struct NbtField {
    pub name: String,
    pub value: NbtValue,
}

impl NbtField {
    pub fn new_compound<S: Into<String>, F: Into<Vec<NbtField>>>(name: S, fields: F) -> NbtField {
        NbtField {
            name: name.into(),
            value: NbtValue::Compound(fields.into()),
        }
    }
    pub fn new_i32<S: Into<String>>(name: S, i: i32) -> NbtField {
        NbtField {
            name: name.into(),
            value: NbtValue::Int(i),
        }
    }
}

// ---- Read Write impls ---------------------------------------------------------------------------
impl NbtField {
    pub fn write<T: TagWrite, W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        match &self.value {
            NbtValue::Byte(b) => {
                T::write(w, TAG_BYTE, &self.name)?;
                w.write_u8(*b)
            }
            NbtValue::Boolean(b) => {
                T::write(w, TAG_BYTE, &self.name)?;
                w.write_u8(if *b { 1 } else { 0 })
            }
            NbtValue::Short(s) => {
                T::write(w, TAG_SHORT, &self.name)?;
                w.write_i16::<BigEndian>(*s)
            }
            NbtValue::Int(i) => {
                T::write(w, TAG_INT, &self.name)?;
                w.write_i32::<BigEndian>(*i)
            }
            NbtValue::Long(l) => {
                T::write(w, TAG_LONG, &self.name)?;
                w.write_i64::<BigEndian>(*l)
            }
            NbtValue::Float(f) => {
                T::write(w, TAG_FLOAT, &self.name)?;
                w.write_f32::<BigEndian>(*f)
            }
            NbtValue::Double(d) => {
                T::write(w, TAG_DOUBLE, &self.name)?;
                w.write_f64::<BigEndian>(*d)
            }
            NbtValue::String(s) => {
                T::write(w, TAG_STRING, &self.name)?;
                write_string(s, w)
            }
            NbtValue::List(l) => write_list::<T, W>(w, l, &self.name),
            NbtValue::Compound(c) => {
                T::write(w, TAG_COMPOUND, &self.name)?;
                for value in c {
                    value.write::<TagWriteFull, W>(w)?;
                }
                w.write_u8(TAG_END)
            }
            NbtValue::ByteArray(arr) => {
                T::write(w, TAG_BYTE_ARRAY, &self.name)?;
                w.write_i32::<BigEndian>(arr.len() as i32)?;
                w.write_all(arr)
            }
            NbtValue::IntArray(arr) => {
                T::write(w, TAG_INT_ARRAY, &self.name)?;
                w.write_i32::<BigEndian>(arr.len() as i32)?;
                for i in arr {
                    w.write_i32::<BigEndian>(*i)?;
                }
                Ok(())
            }
            NbtValue::LongArray(arr) => {
                T::write(w, TAG_LONG_ARRAY, &self.name)?;
                w.write_i32::<BigEndian>(arr.len() as i32)?;
                for i in arr {
                    w.write_i64::<BigEndian>(*i)?;
                }
                Ok(())
            }
            NbtValue::End => w.write_u8(TAG_END),
        }
    }

    pub fn read<R: Read>(r: &mut R) -> Result<NbtField, NbtError> {
        let tag = r.read_u8()?;

        Ok(match tag {
            TAG_END => NbtField {
                name: "".to_string(),
                value: NbtValue::End,
            },
            TAG_BYTE => NbtField {
                name: read_name(r)?,
                value: NbtValue::Byte(r.read_u8()?),
            },
            TAG_SHORT => NbtField {
                name: read_name(r)?,
                value: NbtValue::Short(r.read_i16::<BigEndian>()?),
            },
            TAG_INT => NbtField {
                name: read_name(r)?,
                value: NbtValue::Int(r.read_i32::<BigEndian>()?),
            },
            TAG_LONG => NbtField {
                name: read_name(r)?,
                value: NbtValue::Long(r.read_i64::<BigEndian>()?),
            },
            TAG_FLOAT => NbtField {
                name: read_name(r)?,
                value: NbtValue::Float(r.read_f32::<BigEndian>()?),
            },
            TAG_DOUBLE => NbtField {
                name: read_name(r)?,
                value: NbtValue::Double(r.read_f64::<BigEndian>()?),
            },
            TAG_BYTE_ARRAY => {
                let name = read_name(r)?;
                let len = r.read_i32::<BigEndian>()?;
                let mut buf = vec![0; len as usize];
                _ = r.read(&mut buf)?;
                NbtField {
                    name,
                    value: NbtValue::ByteArray(buf),
                }
            }
            TAG_STRING => {
                let name = read_name(r)?;
                NbtField {
                    name,
                    value: NbtValue::String(read_string(r)?),
                }
            }
            TAG_LIST => {
                let name = read_name(r)?;
                NbtField {
                    name,
                    value: read_list(r)?,
                }
            }
            TAG_INT_ARRAY => {
                let name = read_name(r)?;
                let len = r.read_i32::<BigEndian>()?;
                let mut buf = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(r.read_i32::<BigEndian>()?);
                }
                NbtField {
                    name,
                    value: NbtValue::IntArray(buf),
                }
            }
            TAG_COMPOUND => {
                let name = read_name(r)?;
                NbtField {
                    name,
                    value: read_compound(r)?,
                }
            }
            TAG_LONG_ARRAY => {
                let name = read_name(r)?;
                let len = r.read_i32::<BigEndian>()?;
                let mut buf = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    buf.push(r.read_i64::<BigEndian>()?);
                }
                NbtField {
                    name,
                    value: NbtValue::LongArray(buf),
                }
            }
            _ => panic!("Unknown tag: {}", tag),
        })
    }

    // ---- Element Access -------------------------------------------------------------------------
    pub fn get(&self, name: &str) -> Option<&NbtField> {
        self.value.get(name)
    }

    /// Pulls a field out of the compound and returns it (the field is removed from the original compound).
    pub fn swap_remove(&mut self, name: &str) -> Option<NbtField> {
        self.value.swap_remove(name)
    }

    pub fn get_path(&self, path: &[&str]) -> Option<&NbtField> {
        let path = path.iter();
        let mut child = Some(self);
        for name in path {
            if let Some(c) = child {
                child = c.get(name);
            } else {
                return None;
            }
        }
        child
    }
}

impl std::fmt::Display for NbtField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
