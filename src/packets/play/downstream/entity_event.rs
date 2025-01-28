use minecraft_net_proc::Packet;

Packet!(EntityEvent, 0x1F, {
    entity_id: VarInt,
    entity_status: Int,
});