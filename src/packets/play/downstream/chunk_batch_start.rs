use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct ChunkBatchStart {}

impl Packet for ChunkBatchStart {
    const ID: i32 = 0x0D;
    fn to_bytes(&self) -> Vec<u8> { vec![] }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> { Ok(Self::new()) }
}
impl ChunkBatchStart { pub fn new() -> Self {Self {}} }