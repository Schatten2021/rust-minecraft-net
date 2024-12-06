use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x21]
pub struct UnloadChunk {
    #[Const]
    chunk_z: i32,
    #[Const]
    chunk_x: i32,
}
impl UnloadChunk {
    pub fn new(chunk_z: i32, chunk_x: i32) -> Self {
        Self { chunk_z, chunk_x }
    }
}