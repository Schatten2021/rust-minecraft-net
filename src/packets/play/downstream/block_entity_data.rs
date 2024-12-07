use minecraft_net_proc::Packet;
use crate::packets::Position;

#[derive(Debug, Packet)]
#[id = 0x07]
pub struct BlockEntityData {
    location: Position,
    #[Var]
    r#type: i32,
    data: crab_nbt::Nbt,
}

impl BlockEntityData {
    pub fn new(location: Position, r#type: i32, data: crab_nbt::Nbt) -> Self {
        Self {location, r#type, data}
    }
}