use crate::fields::encode_string;
use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct CookieRequest {
    pub key: String
}
impl Packet for CookieRequest {
    const ID: i32 = 0x05;
    fn to_bytes(&self) -> Vec<u8> {
        encode_string(self.key.clone())
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self { key: reader.read_string()?})
    }
}