use minecraft_net_proc::Packet;
use crate::fields::types::{Double, Float, VarInt};

#[derive(Debug, Packet)]
#[id = 0x20]
pub struct TeleportEntity {
    pub entity_id: VarInt,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub velocity_x: Double,
    pub velocity_y: Double,
    pub velocity_z: Double,
    pub yaw: Float,
    pub pitch: Float,
    pub on_ground: bool,
}