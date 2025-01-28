use minecraft_net_proc::Packet;

Packet!(HurtAnimation, 0x25, {
    entity_id: VarInt,
    yaw: Float,
});