use std::io::{Read, Write};

use crate::error::NbtError;
use crate::field::NbtField;
use crate::list::NbtList;
use crate::value::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

// ---- Write Trait --------------------------------------------------------------------------------
pub trait TagWrite {
    fn write<W: Write>(w: &mut W, tag: u8, name: &str) -> std::io::Result<()>;
}

pub(crate) struct TagWriteFull;
pub(crate) struct TagWriteNone;

impl TagWrite for TagWriteFull {
    fn write<W: Write>(w: &mut W, tag: u8, name: &str) -> std::io::Result<()> {
        w.write_u8(tag)?;
        w.write_u16::<BigEndian>(name.len() as u16)?;
        w.write_all(name.as_bytes())
    }
}

impl TagWrite for TagWriteNone {
    fn write<W: Write>(_w: &mut W, _tag: u8, _name: &str) -> std::io::Result<()> {
        Ok(())
    }
}

// ---- Helper functions ---------------------------------------------------------------------------
pub(crate) fn write_string<W: Write>(string: &str, writer: &mut W) -> std::io::Result<()> {
    writer.write_u16::<BigEndian>(string.len() as u16)?;
    writer.write_all(string.as_bytes())?;
    Ok(())
}

pub(crate) fn read_name<R: Read>(r: &mut R) -> Result<String, NbtError> {
    let len = r.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    r.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

pub(crate) fn read_string<R: Read>(reader: &mut R) -> Result<String, NbtError> {
    let len = reader.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

pub(crate) fn read_compound<R: Read>(r: &mut R) -> Result<NbtValue, NbtError> {
    let mut fields = Vec::new();
    loop {
        let field = NbtField::read(r)?;
        if field.value == NbtValue::End {
            break;
        }
        fields.push(field);
    }

    Ok(NbtValue::Compound(fields))
}

pub(crate) fn read_list<R: Read>(r: &mut R) -> Result<NbtValue, NbtError> {
    let tag = r.read_u8()?;
    let len = r.read_i32::<BigEndian>()?;
    Ok(NbtValue::List(match tag {
        TAG_BYTE => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(r.read_u8()?);
            }
            NbtList::Byte(list)
        }
        TAG_SHORT => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(r.read_i16::<BigEndian>()?);
            }
            NbtList::Short(list)
        }
        TAG_INT => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(r.read_i32::<BigEndian>()?);
            }

            NbtList::Int(list)
        }
        TAG_LONG => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(r.read_i64::<BigEndian>()?);
            }

            NbtList::Long(list)
        }
        TAG_FLOAT => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(r.read_f32::<BigEndian>()?);
            }

            NbtList::Float(list)
        }
        TAG_DOUBLE => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(r.read_f64::<BigEndian>()?);
            }

            NbtList::Double(list)
        }
        TAG_STRING => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(read_string(r)?)
            }

            NbtList::String(list)
        }
        TAG_LIST => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(read_list(r)?);
            }

            NbtList::List(list)
        }
        TAG_COMPOUND => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(NbtField {
                    name: String::new(),
                    value: read_compound(r)?,
                });
            }

            NbtList::Compound(list)
        }
        TAG_LONG_ARRAY => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let nlongs = r.read_i32::<BigEndian>()?;
                let mut buf = vec![0; nlongs as usize];
                for _ in 0..nlongs {
                    buf.push(r.read_i64::<BigEndian>()?);
                }
                list.push(buf);
            }

            NbtList::LongArray(list)
        }
        TAG_INT_ARRAY => {
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let nints = r.read_i32::<BigEndian>()?;
                let mut buf = vec![0; nints as usize];
                for _ in 0..nints {
                    buf.push(r.read_i32::<BigEndian>()?);
                }
                list.push(buf);
            }
            NbtList::IntArray(list)
        }
        TAG_END => NbtList::End,
        _ => panic!("Unknown tag: {}", tag),
    }))
}

pub(crate) fn write_list<T: TagWrite, W: Write>(w: &mut W, l: &NbtList, name: &str) -> std::io::Result<()> {
    T::write(w, TAG_LIST, name)?;
    match l {
        NbtList::Byte(v) => {
            w.write_u8(TAG_BYTE)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            w.write_all(v)
        }
        NbtList::Boolean(v) => {
            w.write_u8(TAG_BYTE)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for b in v {
                w.write_u8(if *b { 1 } else { 0 })?;
            }
            Ok(())
        }
        NbtList::Short(v) => {
            w.write_u8(TAG_SHORT)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for s in v {
                w.write_i16::<BigEndian>(*s)?;
            }
            Ok(())
        }
        NbtList::Int(v) => {
            w.write_u8(TAG_INT)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for i in v {
                w.write_i32::<BigEndian>(*i)?;
            }
            Ok(())
        }
        NbtList::Long(v) => {
            w.write_u8(TAG_LONG)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for l in v {
                w.write_i64::<BigEndian>(*l)?;
            }
            Ok(())
        }
        NbtList::Float(v) => {
            w.write_u8(TAG_FLOAT)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for f in v {
                w.write_f32::<BigEndian>(*f)?;
            }
            Ok(())
        }
        NbtList::Double(v) => {
            w.write_u8(TAG_DOUBLE)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for d in v {
                w.write_f64::<BigEndian>(*d)?;
            }
            Ok(())
        }
        NbtList::String(v) => {
            w.write_u8(TAG_STRING)?;
            w.write_i32::<BigEndian>(v.len() as i32)?;
            for s in v {
                write_string(s, w)?;
            }
            Ok(())
        }
        NbtList::Compound(c) => {
            w.write_u8(TAG_COMPOUND)?;
            w.write_i32::<BigEndian>(c.len() as i32)?;
            for value in c {
                value.write::<TagWriteNone, W>(w)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}