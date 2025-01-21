use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x2F]
pub struct UpdateEntityPosition {
    #[Var]
    pub entity_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}
impl UpdateEntityPosition {
    pub fn new(entity_id: i32, delta_x: i16, delta_y: i16, delta_z: i16, on_ground: bool) -> Self {
        Self {entity_id, delta_x, delta_y, delta_z, on_ground}
    }
}