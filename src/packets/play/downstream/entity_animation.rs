use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x03]
pub struct EntityAnimation {
    #[Var]
    entity_id: i32,
    animation: u8,
}
impl EntityAnimation {
    pub fn new(entity_id: i32, animation: u8) -> Self {
        Self {entity_id, animation}
    }
}