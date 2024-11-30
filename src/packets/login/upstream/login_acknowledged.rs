use crate::{Errors, Packet, PacketReader};
#[derive(Debug)]
pub struct LoginAcknowledged {}
impl Packet for LoginAcknowledged {
    const ID: i32 = 0x03;
    fn to_bytes(&self) -> Vec<u8> { vec![] }
    fn from_reader(_reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {})
    }
}
impl LoginAcknowledged {
    pub fn new() -> Self {Self {}}
}