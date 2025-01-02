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
pub(crate) fn write_name<W: Write>(name: &str, w: &mut W) -> std::io::Result<()> {
    w.write_u16::<BigEndian>(name.len() as u16)?;
    w.write_all(name.as_bytes())
}

pub(crate) fn write_string<W: Write>(string: &str, writer: &mut W) -> std::io::Result<()> {
    writer.write_u16::<BigEndian>(string.len() as u16)?;
    writer.write_all(string.as_bytes())?;
    Ok(())
}

pub(crate) fn read_name<R: Read>(r: &mut R) -> Result<String, NbtError> {
    let len = r.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    r.read(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

pub(crate) fn read_string<R: Read>(reader: &mut R) -> Result<String, NbtError> {
    let len = reader.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    reader.read(&mut buf)?;
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
        TAG_END => NbtList::End,
        _ => panic!("Unknown tag: {}", tag),
    }))
}
