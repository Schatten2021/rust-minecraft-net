use minecraft_net_proc::Packet;

Packet!(ChunkBatchFinished, 0x0C, {
    batch_size: VarInt,
});