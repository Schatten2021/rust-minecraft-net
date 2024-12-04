use crate::{Packet, PacketReader, Result};
use crate::fields::encode_long;

#[derive(Debug)]
pub struct ClientBoundKeepAlive {
    pub keep_alive_id: i64,
}
impl Packet for ClientBoundKeepAlive {
    const ID: i32 = 0x26;
    fn to_bytes(&self) -> Vec<u8> {
        encode_long(self.keep_alive_id)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {keep_alive_id: reader.read_long()})
    }
}
impl ClientBoundKeepAlive {
    pub fn new(keep_alive_id: i64) -> Self {Self {keep_alive_id}}
}