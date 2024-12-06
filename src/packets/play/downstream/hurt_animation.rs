use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x24]
pub struct HurtAnimation {
    #[Var]
    pub entity_id: i32,
    pub yaw: f32
}
impl HurtAnimation {
    pub fn new(entity_id: i32, yaw: f32) -> Self {
        Self {entity_id, yaw}
    }
}