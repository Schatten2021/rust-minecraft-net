use rand::random;
use crate::{Packet, PacketReader};
use crate::fields::encode_int;
#[derive(Debug)]
pub struct Ping {
    pub id: i32,
}
impl Packet for Ping {
    const ID: i32 = 0x05;
    fn to_bytes(&self) -> Vec<u8> {
        encode_int(self.id)
    }
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        Ok(Self { id: reader.read_int() })
    }
}
impl Ping {
    pub fn new() -> Self {
        Self{id: random()}
    }
}