use minecraft_net_proc::Packet;
use crate::fields::types::{VarInt, UUID};

#[derive(Debug, Packet)]
#[id = 0x3F]
pub struct PlayerInfoRemoved {
    len: VarInt,
    #[len = "len"]
    pub uuids: Vec<UUID>
}