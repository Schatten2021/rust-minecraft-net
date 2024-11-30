use crate::{Result, PacketReader, Packet};
use crate::fields::encode_int;

#[derive(Debug)]
pub struct Pong {
    pub id: i32
}
impl Packet for Pong {
    const ID: i32 = 0x05;
    fn to_bytes(&self) -> Vec<u8> {
        encode_int(self.id)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {id: reader.read_int()})
    }
}
impl Pong {
    pub fn new(id: i32) -> Self {
        Self {id}
    }
}