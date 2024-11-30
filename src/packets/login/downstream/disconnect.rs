use crate::fields::encode_string;
use crate::{Errors, Packet, PacketReader};

#[derive(Debug)]
pub struct Disconnect {
    pub reason: String,
}
impl Packet for Disconnect {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        encode_string(self.reason.clone())
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {reason: reader.read_string()?})
    }
}
impl Disconnect {
    pub fn new(reason: String) -> Self {
        Self {reason}
    }
}