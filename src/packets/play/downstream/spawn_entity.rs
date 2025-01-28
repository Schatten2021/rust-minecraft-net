use minecraft_net_proc::Packet;

Packet!(SpawnEntity, 0x01, {
    entity_id: VarInt,
    entity_uuid: UUID,
    r#type: VarInt,
    x: Double,
    y: Double,
    z: Double,
    pitch: Angle,
    yaw: Angle,
    head_yaw: Angle,
    data: VarInt,
    vel_x: Short,
    vel_y: Short,
    vel_z: Short,
});