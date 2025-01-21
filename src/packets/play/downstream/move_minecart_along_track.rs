use minecraft_net_proc::{Field, Packet};
use crate::fields::types::{Angle, Double, Float, PrefixedArray, Short, VarInt};
#[derive(Debug, Field)]
pub struct Steps {
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub velocity_x: Double,
    pub velocity_y: Double,
    pub velocity_z: Double,
    pub yaw: Angle,
    pub pitch: Angle,
    pub weight: Float,
}
#[derive(Debug, Packet)]
#[id = 0x31]
pub struct MoveMinecartAlongTrack {
    pub entity_id: VarInt,
    pub steps: PrefixedArray<Steps>
}