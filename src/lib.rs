use std::io::{Read, Write};
use utils::TagWriteFull;

mod error;
mod field;
mod list;
mod utils;
mod value;

pub use error::NbtError;
pub use field::NbtField;
pub use list::NbtList;
pub use value::NbtValue;

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

#[cfg(test)]
mod tests {

    use super::*;

}
