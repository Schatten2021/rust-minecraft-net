use minecraft_net_proc::{Field, Packet};

Field!(ChunkBiomeData, {
    chunk_z: Int,
    chunk_x: Int,
    data: PrefixedArray<Byte>,
});
Packet!(ChunkBiomes, 0x0E, {
    cunks: PrefixedArray<ChunkBiomeData>,
});