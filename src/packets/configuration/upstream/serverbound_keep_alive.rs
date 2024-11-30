use crate::{Result, PacketReader, Packet};
use crate::fields::encode_long;

#[derive(Debug)]
pub struct ServerBoundKeepAlive {
    pub id: i64,
}
impl Packet for ServerBoundKeepAlive {
    const ID: i32 = 0x04;
    fn to_bytes(&self) -> Vec<u8> {
        encode_long(self.id)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            id: reader.read_long(),
        })
    }
}
impl ServerBoundKeepAlive {
    pub fn new(id: i64) -> Self {
        Self {id}
    }
}