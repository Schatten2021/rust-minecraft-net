use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x0C]
pub struct ChunkBatchFinished {
    #[Var]
    pub batch_size: i32,
}

impl ChunkBatchFinished {
    pub fn new(batch_size: i32) -> Self {Self {batch_size}}
}