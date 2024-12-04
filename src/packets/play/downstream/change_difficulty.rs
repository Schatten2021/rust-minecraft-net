use crate::{Errors, Packet, PacketReader, Result};
use crate::fields::encode_bool;

#[derive(Debug)]
pub struct ChangeDifficulty {
    difficulty: u8,
    difficulty_locked: bool,
}
impl Packet for ChangeDifficulty {
    const ID: i32 = 0x0B;
    fn to_bytes(&self) -> Vec<u8> {
        vec![self.difficulty, encode_bool(self.difficulty_locked)[0]]
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            difficulty: reader.read_ubyte(),
            difficulty_locked: reader.read_bool()?,
        })
    }
}
impl ChangeDifficulty {
    pub fn new(difficulty: u8, locked: bool) -> Self {
        Self {difficulty, difficulty_locked: locked}
    }
}