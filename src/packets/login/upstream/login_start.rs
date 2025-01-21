use minecraft_net_proc::Packet;
use crate::fields::types::*;
#[derive(Debug, Packet)]
#[id = 0x00]
pub struct LoginStart {
    pub name: String,
    pub uuid: UUID,
}
impl LoginStart {
    pub fn new(name: String, uuid: u128) -> Self {
        Self {name, uuid}
    }
}