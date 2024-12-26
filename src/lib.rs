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

impl NbtField {
    pub fn write_name<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        w.write_u16::<BigEndian>(self.name.len() as u16)?;
        w.write_all(self.name.as_bytes())
    }

    pub fn read_name<R: Read>(r: &mut R) -> Result<String, NbtError> {
        let len = r.read_u16::<BigEndian>()?;
        let mut buf = vec![0; len as usize];
        r.read(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn write<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        match &self.value {
            NbtValue::Byte(b) => {
                w.write_u8(TAG_BYTE)?;
                self.write_name(w)?;
                w.write_u8(*b)
            }
            NbtValue::Boolean(b) => {
                w.write_u8(TAG_BYTE)?;
                self.write_name(w)?;
                w.write_u8(if *b { 1 } else { 0 })
            }
            NbtValue::Short(s) => {
                w.write_u8(TAG_SHORT)?;
                self.write_name(w)?;
                w.write_i16::<BigEndian>(*s)
            }
            NbtValue::Int(i) => {
                w.write_u8(TAG_INT)?;
                self.write_name(w)?;
                w.write_i32::<BigEndian>(*i)
            }
            NbtValue::Long(l) => {
                w.write_u8(TAG_LONG)?;
                self.write_name(w)?;
                w.write_i64::<BigEndian>(*l)
            }
            NbtValue::Float(f) => {
                w.write_u8(TAG_FLOAT)?;
                self.write_name(w)?;
                w.write_f32::<BigEndian>(*f)
            }
            NbtValue::Double(d) => {
                w.write_u8(TAG_DOUBLE)?;
                self.write_name(w)?;
                w.write_f64::<BigEndian>(*d)
            }
            NbtValue::String(s) => {
                w.write_u8(TAG_STRING)?;
                self.write_name(w)?;
                w.write_u16::<BigEndian>(s.len() as u16)?;
                w.write_all(s.as_bytes())
            }
            NbtValue::List(l) => {
                w.write_u8(TAG_LIST)?;
                self.write_name(w)?;
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
                    NbtList::String(v) => w.write_i32::<BigEndian>(v.len() as i32),
                    _ => Ok(()),
                }
            }
            NbtValue::Compound(c) => {
                w.write_u8(TAG_COMPOUND)?;
                self.write_name(w)?;
                for value in c {
                    value.write(w)?;
                }
                w.write_u8(TAG_END)
            }
            NbtValue::ByteArray(arr) => {
                w.write_u8(TAG_BYTE_ARRAY)?;
                self.write_name(w)?;
                w.write_i32::<BigEndian>(arr.len() as i32)?;
                w.write_all(&arr)
            }
            NbtValue::IntArray(arr) => {
                w.write_u8(TAG_INT_ARRAY)?;
                self.write_name(w)?;
                w.write_i32::<BigEndian>(arr.len() as i32)?;
                for i in arr {
                    w.write_i32::<BigEndian>(*i)?;
                }
                Ok(())
            }
            NbtValue::LongArray(arr) => {
                w.write_u8(TAG_LONG_ARRAY)?;
                self.write_name(w)?;
                self.write_name(w)?;
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
                name: Self::read_name(r)?,
                value: NbtValue::Byte(r.read_u8()?),
            },
            TAG_SHORT => NbtField {
                name: Self::read_name(r)?,
                value: NbtValue::Short(r.read_i16::<BigEndian>()?),
            },
            TAG_INT => NbtField {
                name: Self::read_name(r)?,
                value: NbtValue::Int(r.read_i32::<BigEndian>()?),
            },
            TAG_LONG => NbtField {
                name: Self::read_name(r)?,
                value: NbtValue::Long(r.read_i64::<BigEndian>()?),
            },
            TAG_FLOAT => NbtField {
                name: Self::read_name(r)?,
                value: NbtValue::Float(r.read_f32::<BigEndian>()?),
            },
            TAG_DOUBLE => NbtField {
                name: Self::read_name(r)?,
                value: NbtValue::Double(r.read_f64::<BigEndian>()?),
            },
            TAG_BYTE_ARRAY => {
                let name = Self::read_name(r)?;
                let len = r.read_i32::<BigEndian>()?;
                let mut buf = vec![0; len as usize];
                r.read(&mut buf)?;
                NbtField {
                    name,
                    value: NbtValue::ByteArray(buf),
                }
            }
            TAG_STRING => {
                let name = Self::read_name(r)?;
                let len = r.read_u16::<BigEndian>()?;
                let mut buf = vec![0; len as usize];
                r.read(&mut buf)?;
                NbtField {
                    name,
                    value: NbtValue::String(String::from_utf8(buf)?),
                }
            }
            TAG_LIST => {
                let name = Self::read_name(r)?;
                let tag = r.read_u8()?;
                let len = r.read_i32::<BigEndian>()?;
                match tag {
                    TAG_BYTE => {
                        let mut list = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            list.push(r.read_u8()?);
                        }

                        NbtField {
                            name,
                            value: NbtValue::List(NbtList::Byte(list)),
                        }
                    }
                    TAG_SHORT => {
                        let mut list = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            list.push(r.read_i16::<BigEndian>()?);
                        }

                        NbtField {
                            name,
                            value: NbtValue::List(NbtList::Short(list)),
                        }
                    }
                    TAG_INT => {
                        let mut list = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            list.push(r.read_i32::<BigEndian>()?);
                        }

                        NbtField {
                            name,
                            value: NbtValue::List(NbtList::Int(list)),
                        }
                    }
                    TAG_LONG => {
                        let mut list = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            list.push(r.read_i64::<BigEndian>()?);
                        }

                        NbtField {
                            name,
                            value: NbtValue::List(NbtList::Long(list)),
                        }
                    }
                    TAG_FLOAT => {
                        let mut list = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            list.push(r.read_f32::<BigEndian>()?);
                        }

                        NbtField {
                            name,
                            value: NbtValue::List(NbtList::Float(list)),
                        }
                    }
                    TAG_DOUBLE => {
                        let mut list = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            list.push(r.read_f64::<BigEndian>()?);
                        }

                        NbtField {
                            name,
                            value: NbtValue::List(NbtList::Double(list)),
                        }
                    }
                    _ => panic!("Unknown tag: {}", tag),
                }
            }
            TAG_COMPOUND => {
                let name = Self::read_name(r)?;
                let mut fields = Vec::new();
                loop {
                    let field = NbtField::read(r)?;
                    if field.value == NbtValue::End {
                        break;
                    }
                    fields.push(field);
                }
                NbtField {
                    name,
                    value: NbtValue::Compound(fields),
                }
            }
            _ => panic!("Unknown tag: {}", tag),
        })
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
}

#[derive(Debug, PartialEq)]
pub struct NbtField {
    pub name: String,
    pub value: NbtValue,
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

pub fn write_nbt<W: Write>(w: &mut W, root: &NbtField) -> Result<(), NbtError> {
    Ok(root.write(w)?)
}

pub fn read_nbt<R: Read>(r: &mut R) -> Result<NbtField, NbtError> {
    NbtField::read(r)
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
    }

    #[test]
    fn compound_read_write() {
        let compound = vec![NbtField {
            name: "int".to_string(),
            value: NbtValue::Int(1 >> 16),
        }];

        read_write_test(NbtField {
            name: "test".to_string(),
            value: NbtValue::Compound(compound),
        });
    }
}
