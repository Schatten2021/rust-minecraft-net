use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x1F]
pub struct EntityEvent {
    #[Const]
    entity_id: i32,
    entity_status: i8,
}
impl EntityEvent {
    pub fn new(entity_id: i32, entity_status: i8) -> Self {
        Self {entity_id, entity_status}
    }
}