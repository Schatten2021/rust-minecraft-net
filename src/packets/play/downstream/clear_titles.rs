use crate::{Packet, PacketReader, Result};
use crate::fields::encode_bool;

#[derive(Debug)]
pub struct ClearTitles {
    pub reset: bool,
}
impl Packet for ClearTitles {
    const ID: i32 = 0x0F;
    fn to_bytes(&self) -> Vec<u8> {
        encode_bool(self.reset)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {reset: reader.read_bool()?})
    }
}
impl ClearTitles {
    pub fn new(reset: bool) -> Self {
        Self {reset}
    }
}