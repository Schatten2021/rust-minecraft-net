use minecraft_net_proc::Packet;

Packet!(UpdateEntityRotation, 0x32, {
    entity_id: VarInt,
    yaw: Angle,
    pitch: Angle,
    on_ground: bool,
});