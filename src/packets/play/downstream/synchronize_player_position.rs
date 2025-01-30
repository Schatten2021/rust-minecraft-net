use minecraft_net_proc::Packet;
use crate::fields::general::TeleportFlags;

Packet!(SynchronizePlayerPosition, 0x42, {
    teleport_id: VarInt,
    x: Double,
    y: Double,
    z: Double,
    velocity_x: Double,
    velocity_y: Double,
    velocity_z: Double,
    yaw: Float,
    pitch: Float,
    flags: TeleportFlags,
});