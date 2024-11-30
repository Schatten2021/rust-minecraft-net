use crate::fields::encode_string;
use crate::{Packet, PacketReader};
use crate::Result;
#[derive(Debug)]
pub struct CookieRequest {
    pub key: String,
}
impl Packet for CookieRequest {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        encode_string(self.key.clone())
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            key: reader.read_string()?,
        })
    }
}
impl CookieRequest {
    pub fn new(key: String) -> Self {
        Self {key}
    }
}