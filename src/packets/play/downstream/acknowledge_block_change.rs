use crate::{Packet, PacketReader, Result};
use crate::fields::encode_var_int;

#[derive(Debug)]
pub struct AcknowledgeBlockChange {
    pub sequence_id: i32,
}
impl Packet for AcknowledgeBlockChange {
    const ID: i32 = 0x05;
    fn to_bytes(&self) -> Vec<u8> {
        encode_var_int(self.sequence_id)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self { sequence_id: reader.read_var_int()?})
    }
}
impl AcknowledgeBlockChange {
    pub fn new(sequence_id: i32) -> Self {
        Self { sequence_id }
    }
}