use crate::{Packet, PacketReader};
#[derive(Debug)]
pub struct ResetChat {}
impl Packet for ResetChat {
    const ID: i32 = 0x06;
    fn to_bytes(&self) -> Vec<u8> { vec![] }
    fn from_reader(_reader: &mut PacketReader) -> crate::errors::Result<Self> {
        Ok(Self {})
    }
}
impl ResetChat {
    pub fn new() -> Self {Self {}}
}