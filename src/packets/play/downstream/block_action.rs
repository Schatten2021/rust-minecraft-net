use crate::packets::Position;
use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x08]
pub struct BlockAction {
    pub location: Position,
    pub action_id: u8,
    pub action_parameter: u8,
    #[Var]
    pub block_type: i32,
}
impl BlockAction {
    pub fn new(location: Position, action_id: u8, action_parameter: u8, block_type: i32) -> Self {
        Self {location, action_id, action_parameter, block_type}
    }
}