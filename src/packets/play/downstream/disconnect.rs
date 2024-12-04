use crate::{Packet, PacketReader, Result};
use crate::fields::encode_string;

#[derive(Debug)]
pub struct Disconnect {
    pub reason: String,
}
impl Packet for Disconnect {
    const ID: i32 = 0x1D;
    fn to_bytes(&self) -> Vec<u8> {
        encode_string(self.reason.clone())
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {reason: reader.read_string()?})
    }
}
impl Disconnect {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}