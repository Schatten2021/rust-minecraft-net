use minecraft_net_proc::Packet;

Packet!(UpdateEntityPosition, 0x2F, {
    entity_id: VarInt,
    delta_x: Short,
    delta_y: Short,
    delta_z: Short,
    on_ground: bool,
});