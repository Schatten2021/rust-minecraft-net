use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct CloseContainer {
    pub window_id: u8
}
impl Packet for CloseContainer {
    const ID: i32 = 0x12;
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {window_id: reader.read_ubyte()})
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![self.window_id]
    }
}
impl CloseContainer {
    pub fn new(window_id: u8) -> Self {
        Self {window_id}
    }
}