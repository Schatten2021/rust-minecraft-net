use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x0D]
pub struct ChunkBatchStart {}

impl ChunkBatchStart { pub fn new() -> Self {Self {}} }