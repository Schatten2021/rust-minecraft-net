use minecraft_net_proc::Packet;
use crate::packets::{ChunkData, LightData};

Packet!(ChunkDataWithLight, 0x28, {
    chunk_x: Int,
    chunk_z: Int,
    data: ChunkData,
    light: LightData, 
});