use minecraft_net_proc::Packet;

#[derive(Packet, Debug)]
#[id = 0x3A]
pub struct PlayerAbilities {
    pub flags: i8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}
impl PlayerAbilities {
    pub fn new(flags: i8, flying_speed: f32, fov_modifier: f32) -> Self {
        Self {flags, flying_speed, fov_modifier}
    }
}