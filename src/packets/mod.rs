use crate::fields::encode_long;
use crate::{Field, PacketReader, Result};

pub mod status;
pub mod handshake;
pub mod login;
pub mod configuration;
pub mod play;

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i16,
    z: i32,
}
impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position { x, y, z }
    }
}
impl Field for Position {
    fn to_bytes(&self) -> Vec<u8> {
        let val = ((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF);
        encode_long(val)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Position> {
        let val = reader.read_long();
        let mut x = ((val >> 38) & 0x3FFFFFF) as i32;
        let mut z = ((val >> 12) & 0x3FFFFFF) as i32;
        let mut y = (val & 0xFFF) as i16;
        if x >= 1 << 25 { x -= 1 << 26 }
        if y >= 1 << 11 { y -= 1 << 12 }
        if z >= 1 << 25 { z -= 1 << 26 }
        Ok(Position {x, y, z})
    }
}