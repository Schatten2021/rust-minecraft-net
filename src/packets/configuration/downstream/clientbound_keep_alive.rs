use crate::Result;
use crate::{Packet, PacketReader};
use crate::fields::encode_long;
#[derive(Debug)]
pub struct ClientBoundKeepAlive {
    pub id: i64,
}
impl Packet for ClientBoundKeepAlive {
    const ID: i32 = 0x04;
    fn to_bytes(&self) -> Vec<u8> {
        encode_long(self.id)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {id: reader.read_long()})
    }
}
impl ClientBoundKeepAlive {
    pub fn new(id: i64) -> Self {
        Self {id}
    }
}