use minecraft_net_proc::Packet;
use crate::fields::types::{Double, VarInt};

#[derive(Debug, Packet)]
#[id = 0x41]
pub struct LookAt {
    pub feet_eyes: VarInt,
    pub target_x: Double,
    pub target_y: Double,
    pub target_z: Double,
    pub is_entity: bool,
    #[when = "is_entity"]
    pub entity_id: Option<VarInt>,
    #[when = "is_entity"]
    pub entity_feet_eyes: Option<VarInt>
}