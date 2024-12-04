use crate::{Packet, PacketReader, Result};
use crate::fields::encode_int;

#[derive(Debug)]
pub struct UnloadChunk {
    chunk_z: i32,
    chunk_x: i32,
}
impl Packet for UnloadChunk {
    const ID: i32 = 0x21;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_int(self.chunk_z), 
            encode_int(self.chunk_x)
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            chunk_z: reader.read_int(),
            chunk_x: reader.read_int(),
        })
    }
}
impl UnloadChunk {
    pub fn new(chunk_z: i32, chunk_x: i32) -> Self {
        Self { chunk_z, chunk_x }
    }
}