use minecraft_net_proc::Packet;

Packet!(AcknowledgeBlockChange, 0x05, {
    sequence_id: VarInt
});