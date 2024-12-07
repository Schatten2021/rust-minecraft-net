use crate::packets::Position;
use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x06]
pub struct SetBlockDestroyStage {
    #[Var]
    entity_id: i32,
    location: Position,
    destroy_stage: i8
}
impl SetBlockDestroyStage {
    pub fn new(entity_id: i32, location: Position, destroy_stage: i8) -> Self {
        Self {entity_id, location, destroy_stage}
    }
}