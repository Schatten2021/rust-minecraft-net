use minecraft_net_proc::Packet;

Packet!(EntityAnimation, 0x03, {
    entity_id: VarInt,
    animation: UByte,
});