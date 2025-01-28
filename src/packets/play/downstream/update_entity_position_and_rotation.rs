use minecraft_net_proc::Packet;

Packet!(UpdateEntityPositionAndRotation, 0x30, {
    entity_id: VarInt,
    delta_x: Short
    delta_y: Short
    delta_z: Short
    yaw: Angle,
    pitch: Angle,
    on_ground: bool,
});