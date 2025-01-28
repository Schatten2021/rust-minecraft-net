use minecraft_net_proc::Packet;

Packet!(TeleportEntity, 0x20, {
    entity_id: VarInt,
    x: Double,
    y: Double,
    z: Double
    velocity_x: Double,
    velocity_y: Double,
    velocity_z: Double,
    yaw: Float,
    pitch: Float,
    on_ground: bool,
});