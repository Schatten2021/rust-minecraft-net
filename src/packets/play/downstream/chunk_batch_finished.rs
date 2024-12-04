use crate::fields::encode_var_int;
use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct ChunkBatchFinished {
    pub batch_size: i32,
}

impl Packet for ChunkBatchFinished {
    const ID: i32 = 0x0C;
    fn to_bytes(&self) -> Vec<u8> {
        encode_var_int(self.batch_size)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {batch_size: reader.read_var_int()?})
    }
}

impl ChunkBatchFinished {
    pub fn new(batch_size: i32) -> Self {Self {batch_size}}
}