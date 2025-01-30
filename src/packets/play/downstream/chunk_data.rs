use minecraft_net_proc::Packet;
use crate::fields::chunk_and_light::{ChunkData, LightData};

Packet!(ChunkDataWithLight, 0x28, {
    chunk_x: Int,
    chunk_z: Int,
    data: ChunkData,
    light: LightData, 
});