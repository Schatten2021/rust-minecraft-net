use minecraft_net_proc::{Field, Packet};

#[derive(Debug, Field, Clone)]
pub struct ChunkBiomeData {
    chunk_z: i32,
    chunk_x: i32,
    #[Var]
    size: i32,
    #[len = "size"]
    data: Vec<u8>,
}
impl ChunkBiomeData {
    pub fn new(chunk_z: i32, chunk_x: i32, data: Vec<u8>) -> Self {
        Self { chunk_z, chunk_x, size: data.len() as i32, data }
    }
}

#[derive(Debug, Packet)]
#[id = 0x0E]
pub struct ChunkBiomes {
    #[Var]
    number_of_chunks: i32,
    #[len = "number_of_chunks"]
    chunks: Vec<ChunkBiomeData>
}

impl ChunkBiomes {
    pub fn new(chunks: Vec<ChunkBiomeData>) -> Self {
        Self { 
            number_of_chunks: chunks.len() as i32,
            chunks 
        }
    }
}