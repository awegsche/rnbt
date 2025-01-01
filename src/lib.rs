use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

mod bytes;
use bytes::*;

const TAG_END: u8 = 0;
const TAG_BYTE: u8 = 1;
const TAG_SHORT: u8 = 2;
const TAG_INT: u8 = 3;
const TAG_LONG: u8 = 4;
const TAG_FLOAT: u8 = 5;
const TAG_DOUBLE: u8 = 6;
const TAG_BYTE_ARRAY: u8 = 7;
const TAG_STRING: u8 = 8;
const TAG_LIST: u8 = 9;
const TAG_COMPOUND: u8 = 10;
const TAG_INT_ARRAY: u8 = 11;
const TAG_LONG_ARRAY: u8 = 12;

#[derive(Debug, PartialEq)]
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

// ---- public functions ---------------------------------------------------------------------------

pub fn write_nbt<W: Write>(w: &mut W, root: &NbtField) -> Result<(), NbtError> {
    Ok(root.write::<TagWriteFull, W>(w)?)
}

pub fn read_nbt<R: Read>(r: &mut R) -> Result<NbtField, NbtError> {
    NbtField::read(r)
}

pub fn from_bytes(bytes: &[u8]) -> Result<NbtField, NbtError> {
    let mut r = std::io::Cursor::new(bytes);
    read_nbt(&mut r)
}

// ---- Write Trait --------------------------------------------------------------------------------
pub trait TagWrite {
    fn write<W: Write>(w: &mut W, tag: u8, name: &str) -> std::io::Result<()>;
}

struct TagWriteFull;
struct TagWriteNone;

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

pub fn write_name<W: Write>(name: &str, w: &mut W) -> std::io::Result<()> {
    w.write_u16::<BigEndian>(name.len() as u16)?;
    w.write_all(name.as_bytes())
}

/*
// ---- Read Trait ---------------------------------------------------------------------------------
pub trait TagRead {
    fn read_tag<R: Read>(&self, r: &mut R) -> Result<u8, NbtError>;
    fn read_name<R: Read>(r: &mut R) -> Result<String, NbtError>;
}

struct TagReadFull;
struct TagReadNone(u8);

impl TagRead for TagReadFull {
    fn read_tag<R: Read>(&self, r: &mut R) -> Result<u8, NbtError> {
        Ok(r.read_u8()?)
    }
}

impl TagRead for TagReadNone {
    fn read_tag<R: Read>(&self, _r: &mut R) -> Result<u8, NbtError> {
        Ok(self.0)
    }
    fn read_name<R: Read>(_r: &mut R) -> Result<String, NbtError> {
        Ok(String::from(""))
    }
}
*/

impl NbtField {
    // ---- Read Write -----------------------------------------------------------------------------
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
                write_string(&s, w)
            }
            NbtValue::List(l) => {
                T::write(w, TAG_LIST, &self.name)?;
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
                w.write_all(&arr)
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
                r.read(&mut buf)?;
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
                let mut buf = vec![0; len as usize];
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
                let mut buf = vec![0; len as usize];
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
        match &self.value {
            NbtValue::Compound(fields) => fields.iter().find(|f| f.name == name),
            _ => None,
        }
    }

    pub fn get_path(&self, path: &[&str]) -> Option<&NbtField> {
        let mut path = path.iter();
        let mut child = Some(self);
        while let Some(name) = path.next() {
            if let Some(c) = child {
                child = c.get(*name);
            }
            else {
                return None;
            }
        }
        child
    }
}

#[derive(Debug, PartialEq)]
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
    End,
}

#[derive(Debug, PartialEq)]
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
            value: NbtValue::Int(i)
        }
    }
}

#[derive(Debug)]
pub enum NbtError {
    RootNotCompoundError,
    IOError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for NbtError {
    fn from(value: std::io::Error) -> Self {
        NbtError::IOError(value)
    }
}

impl From<std::string::FromUtf8Error> for NbtError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        NbtError::Utf8Error(value)
    }
}
fn write_string<W: Write>(string: &str, writer: &mut W) -> std::io::Result<()> {
    writer.write_u16::<BigEndian>(string.len() as u16)?;
    writer.write_all(string.as_bytes())?;
    Ok(())
}

fn read_name<R: Read>(r: &mut R) -> Result<String, NbtError> {
    let len = r.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    r.read(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

fn read_string<R: Read>(reader: &mut R) -> Result<String, NbtError> {
    let len = reader.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    reader.read(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

fn read_compound<R: Read>(r: &mut R) -> Result<NbtValue, NbtError> {
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

fn read_list<R: Read>(r: &mut R) -> Result<NbtValue, NbtError> {
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

#[cfg(test)]
mod tests {

    use super::*;

    fn read_write_test(field: NbtField) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        write_nbt(&mut buf, &field).unwrap();
        println!("{:?}", buf);
        let mut cursor = Cursor::new(buf);
        let read = read_nbt(&mut cursor).unwrap();
        assert_eq!(field, read);
    }

    #[test]
    fn pod_read_write() {
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Byte(255),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Short(256),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Int(1 >> 16),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Long(1 >> 32),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Float(3.14),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Double(3.14),
        });
    }

    #[test]
    fn byte_arrays_read_write() {
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::ByteArray(vec![1, 2, 3]),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::String("hello, world".to_string()),
        });
    }

    #[test]
    fn list_read_write() {
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Byte(vec![1, 2, 3])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Short(vec![1 >> 8, 2 >> 8, 3 >> 8])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Int(vec![1 >> 16, 2 >> 16, 3 >> 16])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Long(vec![1 >> 32, 2 >> 32, 3 >> 32])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Float(vec![1.0, 2.0, 3.0])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Double(vec![1.0, 2.0, 3.0])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::String(vec![
                "1".to_owned(),
                "2".to_owned(),
                "3".to_owned(),
            ])),
        });
        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::List(NbtList::Compound(vec![
                NbtField {
                    name: "".to_string(),
                    value: NbtValue::Compound(vec![
                        NbtField {
                            name: "int_a".to_string(),
                            value: NbtValue::Int(1 >> 16),
                        },
                        NbtField {
                            name: "int_b".to_string(),
                            value: NbtValue::Int(42 >> 16),
                        },
                    ]),
                },
                NbtField {
                    name: "".to_string(),
                    value: NbtValue::Compound(vec![NbtField {
                        name: "float".to_string(),
                        value: NbtValue::Float(1.0),
                    }]),
                },
            ])),
        });
    }

    #[test]
    fn compound_read_write() {
        let compound = vec![
            NbtField {
                name: "int_a".to_string(),
                value: NbtValue::Int(1 >> 16),
            },
            NbtField {
                name: "int_b".to_string(),
                value: NbtValue::Int(42 >> 16),
            },
        ];

        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Compound(compound),
        });
    }

    #[test]
    fn compound_path_access() {

        let root = NbtField::new_compound("test", vec![
            NbtField::new_i32("int_a", 1 >> 16),
            NbtField::new_i32("int_b", 1 >> 16),
            NbtField::new_compound("the", vec![
                NbtField::new_compound("path", vec![
                    NbtField::new_i32("int_c", 1 >> 16),
                    NbtField::new_i32("int_d", 1 >> 16),
                ])
            ])
        ]);

        assert_eq!(root.get_path(&["the", "path", "int_c"]), Some(&NbtField::new_i32("int_c", 1 >> 16)));

    }

    #[test]
    fn compound_access() {
        let compound = vec![
            NbtField {
                name: "int_a".to_string(),
                value: NbtValue::Int(1 >> 16),
            },
            NbtField {
                name: "int_b".to_string(),
                value: NbtValue::Int(42 >> 16),
            },
        ];

        let root = NbtField {
            name: "test".to_string(),
            value: NbtValue::Compound(compound),
        };

        assert_eq!(
            root.get("int_a"),
            Some(&NbtField {
                name: "int_a".to_string(),
                value: NbtValue::Int(1 >> 16)
            })
        );
        assert_eq!(
            root.get("int_b"),
            Some(&NbtField {
                name: "int_b".to_string(),
                value: NbtValue::Int(42 >> 16)
            })
        );
        assert_eq!(root.get("int_c"), None);
    }
}
