use crate::packets::Position;
use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x09]
pub struct BlockUpdate {
    pub location: Position,
    #[Var]
    pub block_id: i32,
}
impl BlockUpdate {
    pub fn new(location: Position, block_id: i32) -> Self {
        Self { location, block_id }
    }
}