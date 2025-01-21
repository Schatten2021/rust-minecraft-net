use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x30]
pub struct UpdateEntityPositionAndRotation {
    #[Var]
    pub entity_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}
impl UpdateEntityPositionAndRotation {
    pub fn new(entity_id: i32, delta_x: i16, delta_y: i16, delta_z: i16, yaw: u8, pitch: u8, on_ground: bool) -> Self {
        Self {entity_id, delta_x, delta_y, delta_z, yaw, pitch, on_ground}
    }
}