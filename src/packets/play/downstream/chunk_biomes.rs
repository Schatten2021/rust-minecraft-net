use crate::fields::{encode_int, encode_var_int};
use crate::{Packet, PacketReader, Result};

//TODO: add Packet
#[derive(Debug)]
pub struct ChunkBiomeData {
    chunk_z: i32,
    chunk_x: i32,
    size: i32,
    data: Vec<u8>,
}
impl ChunkBiomeData {
    pub fn new(chunk_z: i32, chunk_x: i32, data: Vec<u8>) -> Self {
        Self { chunk_z, chunk_x, size: data.len() as i32, data }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_int(self.chunk_z),
            encode_int(self.chunk_x),
            encode_var_int(self.size),
            self.data.clone(),
        ].iter().flatten().cloned().collect()
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let chunk_z = reader.read_int();
        let chunk_x = reader.read_int();
        let size = reader.read_var_int()?;
        let data = reader.read_byte_array(size as usize);
        Ok(Self { chunk_z, chunk_x, size, data })
    }
}

#[derive(Debug)]
pub struct ChunkBiomes {
    number_of_chunks: i32,
    chunks: Vec<ChunkBiomeData>
}

impl Packet for ChunkBiomes {
    const ID: i32 = 0x0E;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.number_of_chunks),
            self.chunks.iter().flat_map(|d| d.to_bytes()).collect()
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let number_of_chunks = reader.read_var_int()?;
        let mut chunks = Vec::with_capacity(number_of_chunks as usize);
        for _ in 0..number_of_chunks {
            chunks.push(ChunkBiomeData::from_reader(reader)?);
        }
        Ok(Self { number_of_chunks, chunks })
    }
}
impl ChunkBiomes {
    pub fn new(chunks: Vec<ChunkBiomeData>) -> Self {
        Self { 
            number_of_chunks: chunks.len() as i32,
            chunks 
        }
    }
}