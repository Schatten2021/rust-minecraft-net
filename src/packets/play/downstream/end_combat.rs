use minecraft_net_proc::Packet;
use crate::fields::types::VarInt;

#[derive(Debug, Packet)]
#[id = 0x3C]
pub struct EndCombat {
    pub duration: VarInt,
}