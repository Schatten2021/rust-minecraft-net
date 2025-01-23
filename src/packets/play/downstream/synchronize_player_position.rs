use minecraft_net_proc::Packet;
use crate::fields::types::{Double, Float, VarInt};

#[derive(Debug, Packet)]
#[id = 0x42]
pub struct SynchronizePlayerPosition {
    pub teleport_id: VarInt,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub velocity_x: Double,
    pub velocity_y: Double,
    pub velocity_z: Double,
    pub yaw: Float,
    pub pitch: Float,
    // pub flags: TeleportFlags, TODO: figure out
}