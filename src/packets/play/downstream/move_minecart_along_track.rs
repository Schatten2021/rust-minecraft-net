use minecraft_net_proc::{Field, Packet};
Field!(Steps, {
    x: Double,
    y: Double,
    z: Double,
    velocity_x: Double,
    velocity_y: Double,
    velocity_z: Double,
    yaw: Angle,
    pitch: Angle,
    weight: Float,
});
Packet!(MoveMinecartAlongTrack, 0x31, {
    entity_id: VarInt,
    steps: PrefixedArray<Steps>,
});