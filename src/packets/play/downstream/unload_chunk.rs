use minecraft_net_proc::Packet;

Packet!(UnloadChunk, 0x22, {
    chunk_z: VarInt,
    chunk_x: VarInt,
});