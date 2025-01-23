use minecraft_net_proc::Packet;
use crate::fields::types::VarInt;

#[derive(Debug, Packet)]
#[id = 0x3E]
pub struct CombatDeath {
    pub player_id: VarInt,
    pub message: String,
}