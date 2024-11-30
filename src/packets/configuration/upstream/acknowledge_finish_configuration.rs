use crate::{Result, PacketReader, Packet};

#[derive(Debug)]
pub struct AcknowledgeFinishConfiguration {}
impl Packet for AcknowledgeFinishConfiguration {
    const ID: i32 = 0x03;
    fn to_bytes(&self) -> Vec<u8> { vec![] }
    fn from_reader(_reader: &mut PacketReader) -> Result<Self> { Ok(Self{}) }
}
impl AcknowledgeFinishConfiguration {
    pub fn new() -> Self {
        Self {}
    }
}